pub mod models;
pub mod schema;

pub use models::{DbCollection, DbDocument, DbUser};

use surrealdb::sql::Thing;
use crate::error::{ApiError, ApiResult};

/// Parse a `"table:id"` string into a SurrealDB [`Thing`].
///
/// `Thing::from_str` returns `Result<_, ()>` and can't be converted to `ApiError` with `?`,
/// so we construct the value manually via `split_once(':')`.
pub fn parse_thing(id: &str) -> ApiResult<Thing> {
    let (table, key) = id
        .split_once(':')
        .ok_or_else(|| ApiError::Validation(format!("Invalid record ID: '{id}'")))?;
    Ok(Thing::from((table, key)))
}
