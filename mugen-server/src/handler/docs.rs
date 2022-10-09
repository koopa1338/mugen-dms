use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Extension, http::StatusCode, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::services;
use tracing::{debug, trace};
use tracing_attributes::instrument;

use super::ApiError;

use common::models::document::Doc;

pub fn router() -> Router {
    Router::new()
        .route("/docs", get(doc_list).post(doc_create))
        .route(
            "/docs/:id",
            get(doc_by_id).patch(doc_update).delete(doc_delete),
        )
}

#[instrument(skip(conn))]
pub async fn doc_list(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match services::docs::get_docs(conn).await {
        Ok(documents) => {
            debug!("Retrieved {} documents", documents.len());
            Ok(Json(documents))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn))]
pub async fn doc_by_id(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match services::docs::get_doc_by_id(id, conn).await {
        Ok(document) => {
            debug!("Retrieved document with id {:?}", document.id);
            trace!("{document}");
            Ok(Json(document))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn, input))]
pub async fn doc_create(
    Json(input): Json<Doc>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    let result = services::docs::create_doc(input, conn).await;
    match result {
        Ok(document) => {
            debug!("Created document with id {:?}", document.id);
            trace!("{document}");
            Ok(Json(document))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn, input))]
pub async fn doc_update(
    Path(id): Path<i64>,
    Json(input): Json<Doc>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match services::docs::update_doc(input, id, conn).await {
        Ok(document) => {
            debug!("Document with id {:?} was updated", document.id);
            trace!("New data {document}");
            Ok(Json(document))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn))]
pub async fn doc_delete(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match services::docs::delete_doc(id, conn).await {
        Ok(document) => {
            debug!("deleted document with id {id}");
            // TODO: we might want to respond with useful json
            Ok(Json(document.rows_affected))
        }
        Err(dberror) => Err(dberror.into()),
    }
}
