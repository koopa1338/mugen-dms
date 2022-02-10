use anyhow::Result;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use super::app;

pub async fn get_database_connection_pool(config: app::Config) -> Result<DatabaseConnection> {
    let mut db= ConnectOptions::new(config.database_url);
    db.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    Ok(Database::connect(db).await?)
}
