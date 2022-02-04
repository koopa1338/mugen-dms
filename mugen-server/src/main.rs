mod config;
mod error;
mod handler;
mod models;
mod services;

use std::time::Duration;

use config::app;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database};

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = app::Config::parse();

    let mut db = ConnectOptions::new(config.database_url);
    db.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let conn = Database::connect(db).await?;

    let frontend = app::static_routes();
    let backend = app::api_routes(conn);

    tokio::join!(frontend, backend);

    Ok(())
}
