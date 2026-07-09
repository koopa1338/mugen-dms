use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: String,
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Collection (")?;
        if let Some(id) = &self.id {
            writeln!(f, "\tid: {id}")?;
        }
        writeln!(f, "\tname: {}", self.name)?;
        writeln!(f, "\towner_id: {}", self.owner_id)?;
        write!(f, ")")
    }
}
