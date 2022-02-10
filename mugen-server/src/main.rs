mod config;
mod error;
mod handler;
mod models;
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
    let conn = db::get_database_connection_pool(config);

    let frontend = app::static_routes();
    let backend = app::api_routes(conn.await?);

    tokio::join!(frontend, backend);

    Ok(())
}
