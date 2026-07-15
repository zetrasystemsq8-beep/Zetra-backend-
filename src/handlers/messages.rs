use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::{middleware::AuthUser, password},
    error::{ApiError, ApiResult},
    models::{Message, ServiceApp, User},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SendMessageInput {
    /// Zetra ID, username, ZetraMail, phone, or email of the
    /// recipient user.
    pub identifier: String,
    pub subject: String,
    pub body: String,
    pub code: Option<String>,
    #[serde(default = "default_kind")]
    pub kind: String,
}

fn default_kind() -> String {
    "verification_code".to_string()
}

#[derive(Debug, Serialize)]
pub struct SendMessageOutput {
    pub message_id: Uuid,
    pub delivered_to: String,
}

/// Verifies the `X-Zetra-App-Name` / `X-Zetra-App-Key` headers
/// against a registered, non-revoked service app.
async fn verify_service_app(state: &AppState, headers: &HeaderMap) -> ApiResult<ServiceApp> {
    let app_name = headers
        .get("x-zetra-app-name")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized)?;
    let app_key = headers
        .get("x-zetra-app-key")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized)?;

    let service_app = sqlx::query_as::<_, ServiceApp>(
        "SELECT * FROM service_apps WHERE name = $1 AND revoked_at IS NULL"
    )
    .bind(app_name)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    if !password::verify_password(app_key, &service_app.api_key_hash) {
        return Err(ApiError::Unauthorized);
    }

    Ok(service_app)
}

/// Called by other Zetra apps (NAI, Nigergram, ZTC) to deliver a
/// verification code or notification into a user's Zetra inbox.
/// Requires X-Zetra-App-Name and X-Zetra-App-Key headers.
pub async fn send_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SendMessageInput>,
) -> ApiResult<Json<SendMessageOutput>> {
    let service_app = verify_service_app(&state, &headers).await?;

    if input.subject.trim().is_empty() || input.body.trim().is_empty() {
        return Err(ApiError::Validation("subject and body are required".into()));
    }

    let identifier = input.identifier.trim().to_lowercase();
    let recipient = sqlx::query_as::<_, User>(
        "SELECT * FROM users
         WHERE lower(zetra_id) = $1
            OR lower(username) = $1
            OR lower(zetramail) = $1
            OR lower(email) = $1
            OR phone = $1
         LIMIT 1"
    )
    .bind(&identifier)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound)?;

    let rec = sqlx::query_as::<_, Message>(
        "INSERT INTO messages (user_id, from_app, kind, subject, body, code)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(recipient.id)
    .bind(&service_app.name)
    .bind(&input.kind)
    .bind(&input.subject)
    .bind(&input.body)
    .bind(&input.code)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(SendMessageOutput {
        message_id: rec.id,
        delivered_to: recipient.zetra_id,
    }))
}

/// Returns the authenticated user's inbox, most recent first.
pub async fn list_messages(
    State(state): State<AppState>,
    user: AuthUser,
) -> ApiResult<Json<Vec<Message>>> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE user_id = $1 ORDER BY created_at DESC LIMIT 100"
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(messages))
}

/// Marks a single message as read. Only the owning user can do this.
pub async fn mark_read(
    State(state): State<AppState>,
    user: AuthUser,
    Path(message_id): Path<Uuid>,
) -> ApiResult<Json<Message>> {
    let rec = sqlx::query_as::<_, Message>(
        "UPDATE messages SET read_at = NOW()
         WHERE id = $1 AND user_id = $2
         RETURNING *"
    )
    .bind(message_id)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(rec))
}

#[derive(Debug, Deserialize)]
pub struct RegisterServiceAppInput {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterServiceAppOutput {
    pub name: String,
    /// Shown once. Store this in the calling app's own secrets —
    /// it cannot be recovered later, only rotated.
    pub api_key: String,
}

/// Registers a new Zetra app (e.g. "nai", "nigergram", "ztc") and
/// issues it an API key for sending messages. Protected by the
/// ADMIN_API_KEY environment variable via the X-Admin-Key header.
pub async fn register_service_app(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<RegisterServiceAppInput>,
) -> ApiResult<Json<RegisterServiceAppOutput>> {
    let expected = std::env::var("ADMIN_API_KEY")
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("ADMIN_API_KEY not configured")))?;
    let provided = headers
        .get("x-admin-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;
    if provided != expected {
        return Err(ApiError::Unauthorized);
    }

    let raw_key = format!(
        "zsk_{}{}",
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple()
    );
    let key_hash = password::hash_password(&raw_key)?;

    sqlx::query("INSERT INTO service_apps (name, api_key_hash) VALUES ($1, $2)")
        .bind(&input.name)
        .bind(&key_hash)
        .execute(&state.db)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.constraint().is_some() => {
                ApiError::Conflict("an app with that name is already registered".into())
            }
            _ => ApiError::Sqlx(e),
        })?;

    Ok(Json(RegisterServiceAppOutput {
        name: input.name,
        api_key: raw_key,
    }))
      }
