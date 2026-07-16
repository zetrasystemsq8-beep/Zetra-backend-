use crate::config::Config;
use crate::storage::service::StorageService;

use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: PgPool,
    pub storage: Arc<StorageService>,
}

impl AppState {
    pub async fn new(cfg: Config, db: PgPool) -> Result<Self> {
        let storage = StorageService::new(&cfg).await?;

        Ok(Self {
            cfg: Arc::new(cfg),
            db,
            storage: Arc::new(storage),
        })
    }
}
