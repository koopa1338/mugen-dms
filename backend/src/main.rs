#![allow(clippy::module_name_repetitions)]

mod auth;
mod config;
mod db;
mod error;
mod handler;
mod services;
mod storage;
mod utils;

use clap::Parser;
use dotenv::dotenv;
use dyn_logger::DynamicLogger;

use auth::password::PasswordAuthProvider;
use config::{app, db as db_config};
use storage::FilesystemStorage;
use utils::cron::scanner_cron;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let logger = DynamicLogger::new(dotenv::var("LOGGING")?)?
        .with_stdout()?
        .with_file_logger()?;
    logger.init();

    let config = app::Config::parse();

    let db = db_config::get_db(&config).await?;
    let storage = FilesystemStorage::new(config.storage_root.clone());
    let auth = PasswordAuthProvider::new(db.clone());
    let state = app::AppState::new(db, storage, auth, config.jwt_secret.clone(), config.port);

    let backend = app::api_routes(state);
    scanner_cron().await;

    backend.await.map_err(|e| e.to_string())?;

    Ok(())
}
