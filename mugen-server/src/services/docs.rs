use entity::prelude::*;
use sea_orm::{
    prelude::*, ActiveValue::NotSet, DatabaseConnection, DeleteResult, IntoActiveModel, Set,
};
use tracing_attributes::instrument;

#[instrument(skip(conn))]
pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<DocumentsModel>, DbErr> {
    tracing::debug!("Requested all documents.");
    Documents::find().all(conn).await
}

#[instrument(skip(conn))]
pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<DocumentsModel, DbErr> {
    tracing::debug!("Requested document with id {id}.");
    Documents::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("No Document with id {id} found")))
}

#[instrument(skip(conn, data))]
pub async fn create_doc(
    data: DocumentsModel,
    conn: &DatabaseConnection,
) -> Result<DocumentsModel, DbErr> {
    tracing::debug!("Create document.");
    let mut entity = data.into_active_model();
    entity.id = NotSet;

    entity.insert(conn).await
}

#[instrument(skip(conn, data))]
pub async fn update_doc(
    data: DocumentsModel,
    id: i64,
    conn: &DatabaseConnection,
) -> Result<DocumentsModel, DbErr> {
    tracing::debug!("Updating document with id {id}.");
    let mut active_model = data.into_active_model();
    active_model.id = Set(id);
    Documents::update(active_model).exec(conn).await
}

#[instrument(skip(conn))]
pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete document with id {id}.");
    Documents::delete_by_id(id).exec(conn).await
}
