use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentQuery {
    pub id: i64,
    pub created: String,
    pub last_updated: Option<String>,
    pub filetype: String, // crate for filetype?
    pub version: i32,
    pub size: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub created: String,
    pub last_updated: Option<String>,
    pub filetype: String, // crate for filetype?
    pub version: i32,
    pub size: i64,
}
