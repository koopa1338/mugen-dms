#![allow(clippy::module_name_repetitions)]

mod config;
mod error;
mod handler;
mod services;
mod utils;

use config::app;
use config::db;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use crate::utils::cron::scanner_cron;
use dyn_logger::DynamicLogger;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let logger = DynamicLogger::new(dotenv::var("LOGGING")?)?;
    logger.init()?;

    let config = app::Config::parse();
    let conn = db::get_database_connection_pool(config.clone());
    let backend = app::api_routes(app::AppState::new(conn.await?));

    let cron = scanner_cron();

    let (_be, cron) = tokio::join!(backend, cron);
    cron?;

    Ok(())
}
