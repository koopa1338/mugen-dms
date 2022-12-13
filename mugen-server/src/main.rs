#![allow(clippy::module_name_repetitions)]

mod config;
mod error;
mod handler;
mod services;
mod utils;

use axum::extract::FromRef;
use config::app;
use config::db;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use sea_orm::DatabaseConnection;

use crate::utils::cron::scanner_cron;
use crate::utils::logging::init;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mut log_guard = init()?;

    let config = app::Config::parse();
    let conn = db::get_database_connection_pool(config.clone());
    let backend = app::api_routes(AppState {
        database: conn.await?,
    });
    let cron = scanner_cron();

    #[cfg(feature = "yew-frontend")]
    let frontend = app::static_routes(config.asset_path);

    #[cfg(feature = "yew-frontend")]
    tokio::join!(frontend, backend);

    #[cfg(not(feature = "yew-frontend"))]
    let (_be, cron) = tokio::join!(backend, cron);
    cron?;

    log_guard.clear();

    Ok(())
}
