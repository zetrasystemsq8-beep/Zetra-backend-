use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    error::ApiResult,
    models::{PublicUser, User},
    state::AppState,
};

pub async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<PublicUser>> {
    let u = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    Ok(Json(u.into()))
}

/// Returns the caller's own Zetra ID, ZetraMail, username, etc.
/// This is a fetch — Zetra IDs are assigned automatically at signup,
/// never generated on demand.
pub async fn me_zetra_id(State(state): State<AppState>, user: AuthUser) -> ApiResult<Json<PublicUser>> {
    let u = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_one(&state.db)
        .await?;
    Ok(Json(u.into()))
}
