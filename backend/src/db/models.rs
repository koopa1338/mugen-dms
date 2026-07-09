use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;
use surrealdb::types::SurrealValue;

/// SurrealDB row for a user. Includes password_hash — never send this to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct DbUser {
    pub id: RecordId,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub bucket_path: String,
}

/// SurrealDB row for a collection.
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct DbCollection {
    pub id: RecordId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner: RecordId,
}

/// SurrealDB row for a document.
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct DbDocument {
    pub id: RecordId,
    pub filename: String,
    pub filepath: String,
    pub filetype: Option<String>,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub collection: Option<RecordId>,
    pub owner: RecordId,
    pub ocr_text: Option<String>,
    pub tags: Vec<String>,
}
