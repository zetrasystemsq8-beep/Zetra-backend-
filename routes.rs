use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        // Root
        .route("/", get(|| async { "✅ Zetra Backend is running!" }))

        // Health
        .route("/health", get(|| async { "✅ Health OK" }))

        // Storage
        .route("/api/storage/upload", post(crate::storage::handler::upload_file))
        .route("/api/storage/delete/:id", delete(crate::storage::handler::delete_file))

        // Existing routes
        .merge(other_routes(state))
}

fn other_routes(_state: AppState) -> Router {
    Router::new()
        .route("/example", get(|| async { "Example route works!" }))
}
