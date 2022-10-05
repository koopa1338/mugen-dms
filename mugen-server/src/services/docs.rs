use common::models::documents::Docs;
use entity::prelude::*;
use sea_orm::{
    prelude::*, ActiveValue::NotSet, DatabaseConnection, DeleteResult, IntoActiveModel, Set,
};
use tracing_attributes::instrument;

#[instrument(skip(conn))]
pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<Docs>, DbErr> {
    tracing::debug!("Requested all documents.");
    let documents = Documents::find().all(conn).await?;

    Ok(Vec::<Docs>::from_iter(documents))
}

#[instrument(skip(conn))]
pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<Docs, DbErr> {
    tracing::debug!("Requested document with id {id}.");
    Documents::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("No Document with id {id} found")))
        .map(Into::into)
}

#[instrument(skip(conn, data))]
pub async fn create_doc(data: Docs, conn: &DatabaseConnection) -> Result<Docs, DbErr> {
    tracing::debug!("Create document.");
    let active_model: DocumentsActiveModel = data.into();

    active_model.insert(conn).await.map(Into::into)
}

#[instrument(skip(conn, data))]
pub async fn update_doc(
    data: Docs,
    id: i64,
    conn: &DatabaseConnection,
) -> Result<Docs, DbErr> {
    tracing::debug!("Updating document with id {id}.");
    let mut active_model: DocumentsActiveModel = data.into();
    active_model.id = Set(id);
    Documents::update(active_model).exec(conn).await.map(Into::into)
}

#[instrument(skip(conn))]
pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete document with id {id}.");
    Documents::delete_by_id(id).exec(conn).await
}
