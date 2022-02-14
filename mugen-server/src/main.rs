mod config;
mod handler;
mod services;

use config::app;
use config::db;
use dotenv::dotenv;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = app::Config::parse();
    let conn = db::get_database_connection_pool(config.clone());
    let backend = app::api_routes(conn.await?);

    #[cfg(feature = "yew-frontend")]
    let frontend = app::static_routes(config.asset_path);

    #[cfg(feature = "yew-frontend")]
    tokio::join!(frontend, backend);

    #[cfg(not(feature = "yew-frontend"))]
    backend.await;

    Ok(())
}
