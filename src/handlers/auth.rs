use axum::{extract::State, Json};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    auth::{jwt, middleware::AuthUser, password},
    error::{ApiError, ApiResult},
    models::{PublicUser, User},
    state::AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshInput {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
    pub user: PublicUser,
}

fn hash_refresh(token: &str) -> String {
    use argon2::PasswordHasher;
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(token.as_bytes(), &salt)
        .map(|h| h.to_string())
        .unwrap_or_default()
}

async fn issue_pair(state: &AppState, user: User) -> ApiResult<TokenPair> {
    let access = jwt::issue(&state.cfg.jwt_secret, user.id, state.cfg.jwt_access_ttl_minutes, "access")?;
    let refresh = jwt::issue(
        &state.cfg.jwt_secret,
        user.id,
        state.cfg.jwt_refresh_ttl_days * 24 * 60,
        "refresh",
    )?;
    let hash = hash_refresh(&refresh);
    let expires_at = Utc::now() + Duration::days(state.cfg.jwt_refresh_ttl_days);
    sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(user.id)
        .bind(&hash)
        .bind(expires_at)
        .execute(&state.db)
        .await?;
    Ok(TokenPair {
        access_token: access,
        refresh_token: refresh,
        token_type: "Bearer",
        expires_in: state.cfg.jwt_access_ttl_minutes * 60,
        user: user.into(),
    })
}

pub async fn register(State(state): State<AppState>, Json(input): Json<RegisterInput>) -> ApiResult<Json<TokenPair>> {
    input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    let hash = password::hash_password(&input.password)?;
    let rec = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&input.username)
    .bind(&input.email.to_lowercase())
    .bind(&hash)
    .fetch_one(&state.db)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db) if db.constraint().is_some() => ApiError::Conflict("email or username already in use".into()),
        _ => ApiError::Sqlx(e),
    })?;
    Ok(Json(issue_pair(&state, rec).await?))
}

pub async fn login(State(state): State<AppState>, Json(input): Json<LoginInput>) -> ApiResult<Json<TokenPair>> {
    input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(input.email.to_lowercase())
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::Unauthorized)?;
    if !password::verify_password(&input.password, &user.password_hash) {
        return Err(ApiError::Unauthorized);
    }
    Ok(Json(issue_pair(&state, user).await?))
}

pub async fn refresh(State(state): State<AppState>, Json(input): Json<RefreshInput>) -> ApiResult<Json<TokenPair>> {
    let claims = jwt::decode_token(&state.cfg.jwt_secret, &input.refresh_token)?;
    if claims.typ != "refresh" {
        return Err(ApiError::Unauthorized);
    }
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::Unauthorized)?;
    // Revoke previous non-expired tokens for this user (simple rotation policy).
    sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL")
        .bind(user.id)
        .execute(&state.db)
        .await?;
    Ok(Json(issue_pair(&state, user).await?))
}

pub async fn me(State(state): State<AppState>, user: AuthUser) -> ApiResult<Json<PublicUser>> {
    let u = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_one(&state.db)
        .await?;
    Ok(Json(u.into()))
}
