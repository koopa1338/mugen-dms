pub mod models;
pub mod schema;

use surrealdb::types::{RecordId, ToSql, SqlFormat};
use crate::error::{ApiError, ApiResult};

/// Format a [`RecordId`] as `"table:key"` string.
///
/// `RecordId` does not implement `Display` in SurrealDB 3.x, so we use `ToSql` directly.
pub fn record_id_to_string(id: &RecordId) -> String {
    let mut buf = String::new();
    id.fmt_sql(&mut buf, SqlFormat::SingleLine);
    buf
}

/// Parse a `"table:id"` string into a SurrealDB [`RecordId`].
///
/// `RecordId::parse_simple` returns `Result<_, anyhow>` and can't be converted to `ApiError` with `?`,
/// so we construct the value manually via `split_once(':')`.
pub fn parse_thing(id: &str) -> ApiResult<RecordId> {
    let (table, key) = id
        .split_once(':')
        .ok_or_else(|| ApiError::Validation(format!("Invalid record ID: '{id}'")))?;
    Ok(RecordId::new(table, key))
}
