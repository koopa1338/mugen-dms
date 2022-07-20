use entity::document::{ActiveModel, Entity as Document, Model as DocumentModel};
use sea_orm::{
    prelude::*, ActiveValue::NotSet, DatabaseConnection, DeleteResult, IntoActiveModel, Set,
};
use tracing_attributes::instrument;

#[instrument]
pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<DocumentModel>, DbErr> {
    tracing::debug!("Requested all documents.");
    Document::find().all(conn).await
}

#[instrument]
pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<DocumentModel, DbErr> {
    tracing::debug!("Requested document with id {id}.");
    Document::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("No Document with id {id} found")))
}

#[instrument]
pub async fn create_doc(
    data: DocumentModel,
    conn: &DatabaseConnection,
) -> Result<DocumentModel, DbErr> {
    tracing::debug!("Create document.");
    let mut entity = data.into_active_model();
    entity.id = NotSet;

    entity.insert(conn).await
}

#[instrument]
pub async fn update_doc(
    data: DocumentModel,
    id: i64,
    conn: &DatabaseConnection,
) -> Result<DocumentModel, DbErr> {
    tracing::debug!("Updating document with id {id}.");
    let mut active_model = data.into_active_model();
    active_model.id = Set(id);
    Document::update(active_model).exec(conn).await
}

#[instrument]
pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete document with id {id}.");
    Document::delete_by_id(id).exec(conn).await
}
