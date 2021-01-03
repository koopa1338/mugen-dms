use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: i64,
    pub client_id: String,
    pub timestamp: i64,
    pub filetype: String, // crate for filetype?
    pub tags: HashSet<String>,
}
