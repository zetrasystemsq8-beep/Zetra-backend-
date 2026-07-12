use axum::{extract::{Path, Query, State}, Json};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::middleware::AuthUser,
    error::{ApiError, ApiResult},
    models::Comment,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub target: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}
fn default_limit() -> i64 { 50 }

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInput {
    #[validate(length(min = 1, max = 200))]
    pub target: String,
    #[validate(length(min = 1, max = 5000))]
    pub body: String,
}

pub async fn list(State(state): State<AppState>, Query(q): Query<ListQuery>) -> ApiResult<Json<Vec<Comment>>> {
    let limit = q.limit.clamp(1, 200);
    let rows = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE target = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(q.target)
    .bind(limit)
    .bind(q.offset.max(0))
    .fetch_all(&state.db)
    .await?;
    Ok(Json(rows))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Json(input): Json<CreateInput>,
) -> ApiResult<Json<Comment>> {
    input.validate().map_err(|e| ApiError::Validation(e.to_string()))?;
    let row = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (user_id, target, body) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(user.id)
    .bind(input.target)
    .bind(input.body)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(row))
}

pub async fn delete(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<axum::http::StatusCode> {
    let row = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::NotFound)?;
    if row.user_id != user.id {
        return Err(ApiError::Forbidden);
    }
    sqlx::query("DELETE FROM comments WHERE id = $1").bind(id).execute(&state.db).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
