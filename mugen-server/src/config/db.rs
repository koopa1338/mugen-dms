use anyhow::Result;

use migration::migrate_database;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrJsonValue {
    kind: String,
    message: String,
}

impl From<DbErr> for ErrJsonValue {
    fn from(err: DbErr) -> Self {
        let error: String = err.to_string();
        if let Some((kind, message)) = error.split_once(": ") {
            return Self {
                kind: kind.into(),
                message: message.into(),
            };
        }

        Self {
            kind: String::from("Unknown"),
            message: error,
        }
    }
}
