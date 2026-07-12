use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("payload too large")]
    PayloadTooLarge,
    #[error("unsupported media type")]
    UnsupportedMediaType,
    #[error("validation failed: {0}")]
    Validation(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.clone()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".into()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".into()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            ApiError::Conflict(m) => (StatusCode::CONFLICT, m.clone()),
            ApiError::PayloadTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, "payload too large".into()),
            ApiError::UnsupportedMediaType => (StatusCode::UNSUPPORTED_MEDIA_TYPE, "unsupported media type".into()),
            ApiError::Validation(m) => (StatusCode::UNPROCESSABLE_ENTITY, m.clone()),
            ApiError::Sqlx(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, "not found".into()),
            ApiError::Sqlx(e) => {
                tracing::error!(error = %e, "sqlx error");
                (StatusCode::INTERNAL_SERVER_ERROR, "database error".into())
            }
            ApiError::Jwt(_) => (StatusCode::UNAUTHORIZED, "invalid token".into()),
            ApiError::Io(e) => {
                tracing::error!(error = %e, "io error");
                (StatusCode::INTERNAL_SERVER_ERROR, "io error".into())
            }
            ApiError::Internal(e) => {
                tracing::error!(error = ?e, "internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
