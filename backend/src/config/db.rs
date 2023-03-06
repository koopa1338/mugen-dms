use anyhow::Result;

use migration::migrate_database;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use super::app;

pub async fn get_database_connection_pool(config: app::Config) -> Result<DatabaseConnection> {
    let mut db = ConnectOptions::new(config.database_url);
    tracing::debug!("Connecting to database.");
    db.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let connection = Database::connect(db).await?;
    migrate_database(&connection).await?;
    Ok(connection)
}
