use surrealdb::engine::any::Any;
use surrealdb::Surreal;

use crate::error::ApiError;

const SCHEMA_SQL: &str = include_str!("../../schema.surql");

pub async fn apply_schema(db: &Surreal<Any>) -> Result<(), ApiError> {
    db.query(SCHEMA_SQL)
        .await
        .map_err(|e| ApiError::Internal(format!("Schema apply failed: {e}")))?;
    tracing::info!("SurrealDB schema applied.");
    Ok(())
}
