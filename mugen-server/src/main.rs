#![allow(unused_imports, dead_code, unused_variables)]

mod config;
mod handler;
mod services;
mod utils;

use config::app;
use config::db;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use crate::utils::cron::scanner_cron;
use crate::utils::logging::init_logging;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let log_guard = init_logging()?;

    let config = app::Config::parse();
    let conn = db::get_database_connection_pool(config.clone());
    let backend = app::api_routes(conn.await?);
    let cron = scanner_cron();

    #[cfg(feature = "yew-frontend")]
    let frontend = app::static_routes(config.asset_path);

    #[cfg(feature = "yew-frontend")]
    tokio::join!(frontend, backend);

    #[cfg(not(feature = "yew-frontend"))]
    let (be, cron) = tokio::join!(backend, cron);
    cron?;

    Ok(())
}
