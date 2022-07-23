use axum::extract::Path;
use axum::Json;
use axum::{extract::Extension, http::StatusCode, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::config::db::DbErrJsonValue;
use crate::services;
use entity::document::{ActiveModel, Model as DocumentModel};
use tracing_attributes::instrument;
use tracing::{debug, trace};

pub fn router() -> Router {
    Router::new()
        .route("/docs", get(doc_list).post(doc_create))
        .route(
            "/docs/:id",
            get(doc_by_id).patch(doc_update).delete(doc_delete),
        )
}

#[instrument(level = "debug", skip(conn))]
pub async fn doc_list(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Vec<DocumentModel>>, (StatusCode, Json<DbErrJsonValue>)> {
    let result = services::docs::get_docs(conn).await;
    match result {
        Ok(documents) => {
            debug!("Retrieved {} documents", documents.len());
            Ok(Json(documents))
        },
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(level = "debug", skip(conn))]
pub async fn doc_by_id(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<DocumentModel>, (StatusCode, Json<DbErrJsonValue>)> {
    //TODO: make this a function that takes the service call as a Fn
    let result = services::docs::get_doc_by_id(id, conn).await;
    match result {
        Ok(document) => {
            debug!("Retrieved document with id {}", document.id);
            trace!("{document}");
            Ok(Json(document))
        },
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(level = "debug", skip(conn, input))]
pub async fn doc_create(
    Json(input): Json<DocumentModel>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<DocumentModel>, (StatusCode, Json<DbErrJsonValue>)> {
    let result = services::docs::create_doc(input, conn).await;
    match result {
        Ok(document) => {
            debug!("Created document with id {}", document.id);
            trace!("{document}");
            Ok(Json(document))
        },
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(level = "debug", skip(conn, input))]
pub async fn doc_update(
    Path(id): Path<i64>,
    Json(input): Json<DocumentModel>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<DocumentModel>, (StatusCode, Json<DbErrJsonValue>)> {
    let result = services::docs::update_doc(input, id, conn).await;
    match result {
        Ok(document) => {
            debug!("Document with id {} was updated", document.id);
            trace!("New data {document}");
            Ok(Json(document))
        },
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(level = "debug", skip(conn))]
pub async fn doc_delete(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<u64>, (StatusCode, Json<DbErrJsonValue>)> {
    let result = services::docs::delete_doc(id, conn).await;
    match result {
        Ok(document) => {
            debug!("deleted document with id {id}");
            //TODO: we might want to respond with useful json
            Ok(Json(document.rows_affected))
        },
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}
