use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Document {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub filename: String,
    /// Backend-agnostic relative storage path, e.g. "alice/uuid-invoice.pdf"
    pub filepath: String,
    pub filetype: Option<String>,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub collection_id: Option<String>,
    pub owner_id: String,
    pub ocr_text: Option<String>,
    pub tags: Vec<String>,
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Document (")?;
        if let Some(id) = &self.id {
            writeln!(f, "\tid: {id}")?;
        }
        writeln!(f, "\tfilename: {}", self.filename)?;
        writeln!(f, "\tfilepath: {}", self.filepath)?;
        if let Some(ft) = &self.filetype {
            writeln!(f, "\tfiletype: {ft}")?;
        }
        writeln!(f, "\tsize: {}", self.size)?;
        writeln!(f, "\tcreated_at: {}", self.created_at)?;
        writeln!(f, "\tupdated_at: {}", self.updated_at)?;
        write!(f, ")")
    }
}
