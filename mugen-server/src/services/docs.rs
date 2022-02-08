use crate::models::document::{Entity as Document, Model as DocumentModel};
use sea_orm::{prelude::*, DatabaseConnection, IntoActiveModel};

pub async fn create_doc(
    data: DocumentModel,
    conn: &DatabaseConnection,
) -> Result<DocumentModel, DbErr> {
    Document::insert(data.into_active_model())
        .exec_with_returning(conn)
        .await
}

pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<DocumentModel, DbErr> {
    if let Some(found) = Document::find_by_id(id).one(conn).await? {
        Ok(found)
    } else {
        Err(DbErr::RecordNotFound(format!(
            "No Document with id {} found",
            id
        )))
    }
}
