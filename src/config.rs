use anyhow::{Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_ttl_minutes: i64,
    pub jwt_refresh_ttl_days: i64,
    pub bind_addr: String,

    pub upload_dir: String,
    pub max_upload_bytes: usize,

    // Backblaze B2
    pub b2_key_id: String,
    pub b2_application_key: String,
    pub b2_bucket_name: String,
    pub b2_bucket_id: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let max_mb: usize = env::var("MAX_UPLOAD_MB")
            .unwrap_or_else(|_| "100".into())
            .parse()
            .unwrap_or(100);

        // Read PORT from Render, fallback to 8080
        let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
        let bind_addr = format!("0.0.0.0:{}", port);

        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL is required")?,

            jwt_secret: env::var("JWT_SECRET")
                .context("JWT_SECRET is required")?,

            jwt_access_ttl_minutes: env::var("JWT_ACCESS_TTL_MINUTES")
                .unwrap_or_else(|_| "15".into())
                .parse()
                .unwrap_or(15),

            jwt_refresh_ttl_days: env::var("JWT_REFRESH_TTL_DAYS")
                .unwrap_or_else(|_| "30".into())
                .parse()
                .unwrap_or(30),

            bind_addr,

            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".into()),

            max_upload_bytes: max_mb * 1024 * 1024,

            // Backblaze B2
            b2_key_id: env::var("B2_KEY_ID")
                .context("B2_KEY_ID is required")?,

            b2_application_key: env::var("B2_APPLICATION_KEY")
                .context("B2_APPLICATION_KEY is required")?,

            b2_bucket_name: env::var("B2_BUCKET_NAME")
                .context("B2_BUCKET_NAME is required")?,

            b2_bucket_id: env::var("B2_BUCKET_ID")
                .context("B2_BUCKET_ID is required")?,
        })
    }
}
