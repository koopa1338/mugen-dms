use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// SurrealDB row for a user. Includes password_hash — never send this to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUser {
    pub id: Thing,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub bucket_path: String,
}

/// SurrealDB row for a collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbCollection {
    pub id: Thing,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner: Thing,
}

/// SurrealDB row for a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbDocument {
    pub id: Thing,
    pub filename: String,
    pub filepath: String,
    pub filetype: Option<String>,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub collection: Option<Thing>,
    pub owner: Thing,
    pub ocr_text: Option<String>,
    pub tags: Vec<String>,
}
