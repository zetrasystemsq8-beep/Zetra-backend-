mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod state;

use anyhow::Context;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("\n==============================");
        eprintln!("FATAL ERROR:");
        eprintln!("{:#}", err);
        eprintln!("==============================\n");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ✅ Step 1: Use Render's DATABASE_URL
    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("connecting to database")?;

    db::migrate(&pool)
        .await
        .context("running database migrations")?;

    let cfg = config::Config::from_env()
        .context("loading configuration")?;

    tokio::fs::create_dir_all(&cfg.upload_dir).await.ok();

    let state = state::AppState::new(cfg.clone(), pool);
    let app = routes::router(state);

    // ✅ Step 2: Bind to Render's dynamic PORT
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    tracing::info!("Zetra backend listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .context("starting Axum server")?;

    Ok(())
}
