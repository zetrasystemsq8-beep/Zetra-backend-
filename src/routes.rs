use axum::{http::StatusCode, routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "✅ Zetra Backend Running" }))
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
