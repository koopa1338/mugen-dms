use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    Surreal,
};

use crate::{db::schema::apply_schema, error::ApiError};

use super::app::Config;

/// Connect to SurrealDB, authenticate if credentials are provided,
/// select the namespace/database, and apply the schema idempotently.
///
/// `config.database_url` examples:
/// - `ws://localhost:8000`       remote WebSocket (SurrealDB server)
/// - `wss://db.example.com`      remote WebSocket with TLS
/// - `rocksdb://./data/mugen`    embedded RocksDB (no separate process needed)
/// - `mem://`                    in-memory (tests / CI)
pub async fn get_db(config: &Config) -> Result<Surreal<Any>, ApiError> {
    tracing::debug!("Connecting to SurrealDB at {}", config.database_url);

    let db = connect(&config.database_url)
        .await
        .map_err(|e| ApiError::Internal(format!("SurrealDB connect failed: {e}")))?;

    if let (Some(user), Some(pass)) = (&config.db_user, &config.db_pass) {
        db.signin(Root { username: user, password: pass })
            .await
            .map_err(|e| ApiError::Internal(format!("SurrealDB auth failed: {e}")))?;
    }

    db.use_ns("mugen")
        .use_db("dms")
        .await
        .map_err(|e| ApiError::Internal(format!("SurrealDB use_ns/use_db failed: {e}")))?;

    apply_schema(&db).await?;

    tracing::info!("SurrealDB connected and schema applied.");
    Ok(db)
}
