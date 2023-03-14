use anyhow::Result;

use migration::migrate_database;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use super::app;

/// Establishes a connection pool to the database specified in the provided app configuration,
/// sets connection pool options, connects to the database, runs database migrations, and returns
/// a handle to the connection pool.
///
/// ## Arguments
///
/// * `config` - An `app::Config` object specifying the database URL.
///
/// ## Returns
///
/// A `Result` containing a handle to the database connection pool or a database error.
///
/// ## Example
///
/// ```
/// let config = app::Config {
///     database_url: String::from("postgres://user:password@localhost:5432/db"),
/// };
/// let conn_pool = get_database_connection_pool(config).await.unwrap();
/// ```
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
