pub mod documents;

use common::models::documents::Docs;
use documents::{Model, ActiveModel};
use sea_orm::{ActiveValue::NotSet, Set};

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Document (")?;
        writeln!(f, "\tid: {}", self.id)?;
        writeln!(f, "\tcreated: {}", self.created)?;
        if let Some(last_updated) = self.last_updated {
            writeln!(f, "\tlast_updated: {}", last_updated)?;
        }
        if let Some(ft) = &self.filetype {
            writeln!(f, "\tfiletype: {}", ft)?;
        }
        writeln!(f, "\tversion: {}", self.version)?;
        writeln!(f, "\tsize: {}", self.size)?;
        write!(f, ")")
    }
}

impl From<Model> for Docs {
    fn from(doc: Model) -> Self {
        Self {
            id: Some(doc.id),
            created: doc.created,
            last_updated: doc.last_updated,
            filetype: doc.filetype,
            version: doc.version,
            size: doc.size,
            data: doc.data,
        }
    }
}

impl From<Docs> for ActiveModel {
    fn from(doc: Docs) -> Self {
        Self {
            id: NotSet,
            created: Set(doc.created),
            last_updated: Set(doc.last_updated),
            filetype: Set(doc.filetype),
            version: Set(doc.version),
            size: Set(doc.size),
            data: Set(doc.data),
        }
    }
}
impl FromIterator<Model> for Vec<Docs> {
    fn from_iter<T: IntoIterator<Item = Model>>(iter: T) -> Self {
        iter.into_iter()
            .map(Into::<Docs>::into)
            .collect()
    }
}

