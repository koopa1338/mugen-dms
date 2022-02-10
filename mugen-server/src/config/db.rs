use anyhow::Result;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::app;

pub async fn get_database_connection_pool(config: app::Config) -> Result<DatabaseConnection> {
    let mut db = ConnectOptions::new(config.database_url);
    db.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    Ok(Database::connect(db).await?)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrJsonValue {
    kind: String,
    message: String,
}

impl From<DbErr> for DbErrJsonValue {
    fn from(err: DbErr) -> Self {
        let error_string = err.to_string();
        let (kind, message) = error_string.split_once(": ").unwrap();
        Self {
            kind: kind.to_string(),
            message: message.to_string(),
        }
    }
}
