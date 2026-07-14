use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        header::AUTHORIZATION,
        request::Parts,
    },
};
use uuid::Uuid;

use crate::{
    auth::jwt,
    error::ApiError,
    state::AppState,
};

pub struct AuthUser {
    pub id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        tracing::info!("========== AUTH MIDDLEWARE ==========");

        // Read Authorization header
        let header = match parts.headers.get(AUTHORIZATION) {
            Some(h) => h,
            None => {
                tracing::error!("Missing Authorization header");
                return Err(ApiError::Unauthorized);
            }
        };

        let header = match header.to_str() {
            Ok(h) => h,
            Err(e) => {
                tracing::error!("Invalid Authorization header: {:?}", e);
                return Err(ApiError::Unauthorized);
            }
        };

        tracing::info!("Authorization header: {}", header);

        // Must start with Bearer
        let token = match header.strip_prefix("Bearer ") {
            Some(t) => t,
            None => {
                tracing::error!("Authorization header does not start with 'Bearer '");
                return Err(ApiError::Unauthorized);
            }
        };

        tracing::info!("Token received (first 20 chars): {}", &token[..token.len().min(20)]);

        // Decode JWT
        let claims = match jwt::decode_token(&state.cfg.jwt_secret, token) {
            Ok(c) => {
                tracing::info!("JWT decoded successfully");
                tracing::info!("User ID: {}", c.sub);
                tracing::info!("Token type: {}", c.typ);
                tracing::info!("Expires at: {}", c.exp);
                c
            }
            Err(e) => {
                tracing::error!("JWT decode failed: {:?}", e);
                return Err(ApiError::Unauthorized);
            }
        };

        if claims.typ != "access" {
            tracing::error!(
                "Wrong token type. Expected 'access', got '{}'",
                claims.typ
            );
            return Err(ApiError::Unauthorized);
        }

        tracing::info!("Authentication successful");
        tracing::info!("====================================");

        Ok(AuthUser {
            id: claims.sub,
        })
    }
}
