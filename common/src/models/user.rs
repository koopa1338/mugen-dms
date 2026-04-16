use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User as exposed to the frontend — never includes password_hash.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    /// Logical bucket name used as directory/prefix in the storage backend.
    pub bucket_path: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "User (")?;
        if let Some(id) = &self.id {
            writeln!(f, "\tid: {id}")?;
        }
        writeln!(f, "\tusername: {}", self.username)?;
        writeln!(f, "\temail: {}", self.email)?;
        write!(f, ")")
    }
}
