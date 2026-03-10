use anyhow::Result;
use axum::Router;
use log::{debug, error, info, trace, warn};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod urls;
mod utils;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("starting server");

    // read static/config.toml
    let config = utils::config::get_config().inspect_err(|e| error!("{:?}", e))?;

    let database_url = config.database.url;
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .inspect_err(|e| error!("{:?}", e))?;
    info!("connected to PostgreSQL database");

    let (ip, port) = (config.server.ip, config.server.port);
    let addr = format!(
        "{}:{}",
        ip,
        port.unwrap_or_else(|| {
            warn!("server port not specified, using 7742");
            7742
        })
    );

    let listener = TcpListener::bind(addr.clone())
        .await
        .inspect_err(|e| error!("{:?}", e))?;
    info!("started server at {}", addr);

    let state = AppState { db_pool };
    let app = Router::new().merge(urls::router()).with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}
