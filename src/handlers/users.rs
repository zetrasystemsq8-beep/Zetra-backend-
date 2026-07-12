use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use crate::{error::ApiResult, models::{PublicUser, User}, state::AppState};

pub async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<PublicUser>> {
    let u = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    Ok(Json(u.into()))
}
