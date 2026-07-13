use axum::{Router, routing::get};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        // ✅ Root route for quick browser test
        .route("/", get(|| async { "✅ Backend is running on Render!" }))
        // ✅ Health check route
        .route("/health", get(|| async { "✅ Health OK" }))
        // Merge in your other routes (auth, handlers, etc.)
        .merge(other_routes(state))
}

// Example placeholder for your other routes
fn other_routes(_state: AppState) -> Router {
    Router::new()
        .route("/example", get(|| async { "Example route works!" }))
}
