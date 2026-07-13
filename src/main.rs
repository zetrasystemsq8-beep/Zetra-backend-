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

    let cfg = config::Config::from_env()
        .context("loading configuration")?;

    tokio::fs::create_dir_all(&cfg.upload_dir).await.ok();

    let pool = db::connect(&cfg.database_url)
        .await
        .context("connecting to database")?;

    db::migrate(&pool)
        .await
        .context("running database migrations")?;

    let state = state::AppState::new(cfg.clone(), pool);
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr)
        .await
        .with_context(|| format!("binding to {}", cfg.bind_addr))?;

    tracing::info!("Zetra backend listening on {}", cfg.bind_addr);

    axum::serve(listener, app)
        .await
        .context("starting Axum server")?;

    Ok(())
}
