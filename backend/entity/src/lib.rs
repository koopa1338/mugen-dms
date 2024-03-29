#![allow(clippy::doc_markdown)]

use sea_orm::{EntityTrait, PrimaryKeyTrait};

pub mod category;
pub mod document;

pub mod prelude;

pub trait PrimaryKeySetter<T, P>
where
    T: EntityTrait,
    P: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + std::fmt::Display + Clone,
{
    fn set_pk(&mut self, pk: P);
}

mod doc_utils {
    use super::document::{
        ActiveModel as DocumentAM, Entity as DocumentEntity, Model as DocumentModel,
    };
    use super::PrimaryKeySetter;
    use common::models::document::Doc;
    use sea_orm::{ActiveValue::NotSet, Set};

    impl PrimaryKeySetter<DocumentEntity, i64> for DocumentAM {
        fn set_pk(&mut self, pk: i64) {
            self.id = Set(pk);
        }
    }

    impl std::fmt::Display for DocumentModel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f)?;
            writeln!(f, "Document (")?;
            writeln!(f, "\tid: {}", self.id)?;
            writeln!(f, "\tcreated: {}", self.created)?;
            if let Some(updated) = self.updated {
                writeln!(f, "\tlast_updated: {updated}")?;
            }
            if let Some(ft) = &self.filetype {
                writeln!(f, "\tfiletype: {ft}")?;
            }
            writeln!(f, "\tversion: {}", self.version)?;
            writeln!(f, "\tsize: {}", self.size)?;
            if let Some(c) = &self.category_id {
                writeln!(f, "\tcategory_id: {c}")?;
            }
            write!(f, ")")
        }
    }

    impl From<DocumentModel> for Doc {
        fn from(doc: DocumentModel) -> Self {
            Self {
                id: Some(doc.id),
                created: doc.created,
                updated: doc.updated,
                filetype: doc.filetype,
                version: doc.version,
                size: doc.size,
                data: doc.data,
                category_id: doc.category_id,
            }
        }
    }

    impl From<Doc> for DocumentAM {
        fn from(doc: Doc) -> Self {
            Self {
                id: NotSet,
                created: Set(doc.created),
                updated: Set(doc.updated),
                filetype: Set(doc.filetype),
                version: Set(doc.version),
                size: Set(doc.size),
                data: Set(doc.data),
                category_id: Set(doc.category_id),
            }
        }
    }

    impl FromIterator<DocumentModel> for Vec<Doc> {
        fn from_iter<T: IntoIterator<Item = DocumentModel>>(iter: T) -> Self {
            iter.into_iter().map(Into::<Doc>::into).collect()
        }
    }
}

mod category_utils {
    use super::category::{
        ActiveModel as CategoryAM, Entity as CategoryEntity, Model as CategoryModel,
    };
    use super::PrimaryKeySetter;
    use common::models::category::Category;
    use sea_orm::{ActiveValue::NotSet, Set};

    impl PrimaryKeySetter<CategoryEntity, i32> for CategoryAM {
        fn set_pk(&mut self, pk: i32) {
            self.id = Set(pk);
        }
    }

    impl std::fmt::Display for CategoryModel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f)?;
            writeln!(f, "Category (")?;
            writeln!(f, "\tid: {}", self.id)?;
            writeln!(f, "\ttitle: {}", self.title)?;
            write!(f, ")")
        }
    }

    impl From<CategoryModel> for Category {
        fn from(category: CategoryModel) -> Self {
            Self {
                id: Some(category.id),
                title: category.title,
            }
        }
    }

    impl From<Category> for CategoryAM {
        fn from(category: Category) -> Self {
            Self {
                id: NotSet,
                title: Set(category.title),
            }
        }
    }

    impl FromIterator<CategoryModel> for Vec<Category> {
        fn from_iter<T: IntoIterator<Item = CategoryModel>>(iter: T) -> Self {
            iter.into_iter().map(Into::<Category>::into).collect()
        }
    }
}
