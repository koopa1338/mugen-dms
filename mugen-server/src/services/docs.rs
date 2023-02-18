use super::SeaServiceTrait;
use common::models::document::Doc;
use entity::prelude::*;
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult};
use tracing_attributes::instrument;

impl SeaServiceTrait for Doc {
    type Entity = DocumentEntity;
    type Pk = i64;
    type AModel = DocumentAM;
}

#[instrument(skip(conn))]
pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<Doc>, DbErr> {
    tracing::debug!("Requested all documents.");
    Doc::get_entities(conn).await
}

#[instrument(skip(conn))]
pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Requested document with id {id}.");
    Doc::get_entity_by_pk(id, conn).await
}

#[instrument(skip(conn, data))]
pub async fn create_doc(data: Doc, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Create document.");
    Doc::create_entity(data, conn).await
}

#[instrument(skip(conn, data))]
pub async fn update_doc(data: Doc, id: i64, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Updating document with id {id}.");
    Doc::update_entity_by_pk(data, id, conn).await
}

#[instrument(skip(conn))]
pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete document with id {id}.");
    DocumentEntity::delete_by_id(id).exec(conn).await
}

/// Returns all Documents that are linked to the Category with the passed `id`.
#[instrument(skip(conn))]
pub async fn get_docs_by_category(id: i32, conn: &DatabaseConnection) -> Result<Vec<Doc>, DbErr> {
    tracing::debug!("Fetch Documents with linked Category id {id}.");
    Ok(DocumentEntity::find()
        .filter(category::Column::Id.eq(id))
        .all(conn)
        .await?
        .into_iter()
        .collect::<Vec<_>>())
}
