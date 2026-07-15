use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState, models::Message};

#[derive(Deserialize)]
pub struct CreateIdentityRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateIdentityResponse {
    pub zetra_id: String,
}

pub async fn create_identity(
    State(state): State<AppState>,
    Json(payload): Json<CreateIdentityRequest>,
) -> Result<(StatusCode, Json<CreateIdentityResponse>), AppError> {
    let mut zetra_id = String::new();
    
    // Retry if the generated ID already exists
    for _ in 0..10 {
        let code: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(|b| char::from(b).to_ascii_uppercase())
            .collect();
        
        let candidate = format!("ZTR-{}", code);

        let exists = sqlx::query!("SELECT 1 as x FROM identities WHERE zetra_id = $1", candidate)
            .fetch_optional(&state.db)
            .await?;

        if exists.is_none() {
            zetra_id = candidate;
            break;
        }
    }

    if zetra_id.is_empty() {
        return Err(AppError::Internal("Failed to generate unique Zetra ID".into()));
    }

    sqlx::query!(
        "INSERT INTO identities (zetra_id, name) VALUES ($1, $2)",
        zetra_id,
        payload.name
    )
    .execute(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(CreateIdentityResponse { zetra_id })))
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(zetra_id): Path<String>,
) -> Result<Json<Vec<Message>>, AppError> {
    let identity = sqlx::query!("SELECT id FROM identities WHERE zetra_id = $1", zetra_id)
        .fetch_optional(&state.db)
        .await?;

    let identity = match identity {
        Some(row) => row,
        None => return Err(AppError::NotFound("Identity not found".into())),
    };

    let messages = sqlx::query_as!(
        Message,
        "SELECT id, identity_id, sender, content, received_at FROM messages WHERE identity_id = $1 ORDER BY received_at DESC",
        identity.id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(messages))
}
