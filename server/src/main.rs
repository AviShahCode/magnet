pub mod config;
pub mod urls;
pub mod utils;

use anyhow::Context;
use log::info;
use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::signal;

#[derive(Clone)]
pub struct AppState {
    db: sqlx::PgPool,
    redis: ConnectionManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting Magnet");

    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL must be set in the environment")?;
    let database = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
        .context("Failed to establish a connection pool to PostgreSQL")?;
    info!("Connected to PostgreSQL");

    let redis_url = env::var("REDIS_URL").context("REDIS_URL must be set in the environment")?;
    let redis_client = redis::Client::open(redis_url).context("Failed to create Redis client")?;
    let redis = ConnectionManager::new(redis_client)
        .await
        .context("Failed to create Redis connection manager")?;
    info!("Connected to the Redis");

    let state = AppState {
        db: database,
        redis,
    };

    let app = urls::router().with_state(state);

    let listen_url = env::var("MAGNET_LISTEN_URL")
        .context("MAGNET_LISTEN_URL must be set in the environment")?;
    let listener = tokio::net::TcpListener::bind(&listen_url)
        .await
        .context("Failed to bind to listen URL")?;

    info!("Magnet listening on {}", listen_url);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Magnet crashed")?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
