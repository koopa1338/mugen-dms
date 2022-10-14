use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Category {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub title: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Category (")?;
        if let Some(id) = self.id {
            writeln!(f, "\tid: {}", id)?;
        }
        writeln!(f, "\ttitle: {}", self.title)?;
        write!(f, ")")
    }
}
