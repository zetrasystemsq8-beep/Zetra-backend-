use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::{handlers, state::AppState};

pub fn router(state: AppState) -> Router {
    let max = state.cfg.max_upload_bytes;
    let upload_dir = state.cfg.upload_dir.clone();

    let api = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh))
        .route("/auth/me", get(handlers::auth::me))
        .route("/users/:id", get(handlers::users::get_user))
        .route("/comments", get(handlers::comments::list).post(handlers::comments::create))
        .route("/comments/:id", delete(handlers::comments::delete))
        .route("/media/image", post(handlers::media::upload_image))
        .route("/media/video", post(handlers::media::upload_video));

    Router::new()
        .route("/healthz", get(|| async { (StatusCode::OK, "ok") }))
        .nest("/api", api)
        .nest_service("/media", ServeDir::new(upload_dir))
        .layer(DefaultBodyLimit::max(max))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
