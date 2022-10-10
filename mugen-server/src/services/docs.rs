use super::helper;
use common::models::document::Doc;
use entity::prelude::*;
use sea_orm::{
    prelude::*, ActiveValue::NotSet, DatabaseConnection, DeleteResult, Set,
};
use tracing_attributes::instrument;

#[instrument(skip(conn))]
pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<Doc>, DbErr> {
    tracing::debug!("Requested all documents.");
    helper::get_entities::<DocumentEntity, Vec<Doc>>(conn).await
}

#[instrument(skip(conn))]
pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Requested document with id {id}.");
    helper::get_entity_by_pk::<DocumentEntity, Doc, i64>(id, conn).await
}

#[instrument(skip(conn, data))]
pub async fn create_doc(data: Doc, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Create document.");
    helper::create_entity::<DocumentEntity, Doc, DocumentAM>(data, conn).await
}

#[instrument(skip(conn, data))]
pub async fn update_doc(data: Doc, id: i64, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Updating document with id {id}.");
    helper::update_entity_by_pk::<DocumentEntity, Doc, i64, DocumentAM>(data, id, conn).await
}

#[instrument(skip(conn))]
pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete document with id {id}.");
    DocumentEntity::delete_by_id(id).exec(conn).await
}
