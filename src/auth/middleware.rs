use axum::{
    extract::{FromRequestParts, State},
    http::request::Parts,
    RequestPartsExt,
};
use axum::async_trait;
use uuid::Uuid;

use crate::{auth::jwt, error::ApiError, state::AppState};

pub struct AuthUser {
    pub id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let _ = parts.extract::<State<AppState>>().await;
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiError::Unauthorized)?;
        let token = header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;
        let claims = jwt::decode_token(&state.cfg.jwt_secret, token)?;
        if claims.typ != "access" {
            return Err(ApiError::Unauthorized);
        }
        Ok(AuthUser { id: claims.sub })
    }
}
