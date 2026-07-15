use axum::{extract::State, Json};
use rand::{distributions::Alphanumeric, Rng};
use crate::{
    error::{ApiError, ApiResult},
    state::AppState,
    models::Message,
};

pub async fn generate_zetra_id(State(state): State<AppState>) -> ApiResult<Json<Message>> {
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    Ok(Json(Message {
        message: format!("ZETRA-{}", code.to_uppercase()),
    }))
}
