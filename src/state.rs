use crate::config::Config;
use crate::storage::service::StorageService;

use anyhow::{Context, Result};
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
        let storage = match StorageService::new(&cfg).await {
            Ok(storage) => {
                tracing::info!("✅ Backblaze B2 initialized successfully");
                storage
            }
            Err(err) => {
                tracing::error!("❌ Failed to initialize Backblaze B2: {:#}", err);
                return Err(err).context("initializing Backblaze B2");
            }
        };

        Ok(Self {
            cfg: Arc::new(cfg),
            db,
            storage: Arc::new(storage),
        })
    }
}
