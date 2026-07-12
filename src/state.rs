use crate::config::Config;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: PgPool,
}

impl AppState {
    pub fn new(cfg: Config, db: PgPool) -> Self {
        Self { cfg: Arc::new(cfg), db }
    }
}
