use axum::extract::Path;
use axum::Json;
use axum::{extract::Extension, http::StatusCode, routing::get, Router};
use common::models::documents::Docs;
use sea_orm::DatabaseConnection;

use crate::config::db::ErrJsonValue;
use crate::services;
use tracing::{debug, trace};
use tracing_attributes::instrument;

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
) -> Result<Json<Vec<Docs>>, (StatusCode, Json<ErrJsonValue>)> {
    match services::docs::get_docs(conn).await {
        Ok(documents) => {
            debug!("Retrieved {} documents", documents.len());
            Ok(Json(documents))
        }
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(skip(conn))]
pub async fn doc_by_id(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Docs>, (StatusCode, Json<ErrJsonValue>)> {
    match services::docs::get_doc_by_id(id, conn).await {
        Ok(document) => {
            debug!("Retrieved document with id {:?}", document.id);
            trace!("{document}");
            Ok(Json(document))
        }
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(skip(conn, input))]
pub async fn doc_create(
    Json(input): Json<Docs>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Docs>, (StatusCode, Json<ErrJsonValue>)> {
    let result = services::docs::create_doc(input, conn).await;
    match result {
        Ok(document) => {
            debug!("Created document with id {:?}", document.id);
            trace!("{document}");
            Ok(Json(document))
        }
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(skip(conn, input))]
pub async fn doc_update(
    Path(id): Path<i64>,
    Json(input): Json<Docs>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Docs>, (StatusCode, Json<ErrJsonValue>)> {
    match services::docs::update_doc(input, id, conn).await {
        Ok(document) => {
            debug!("Document with id {:?} was updated", document.id);
            trace!("New data {document}");
            Ok(Json(document))
        }
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}

#[instrument(skip(conn))]
pub async fn doc_delete(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<u64>, (StatusCode, Json<ErrJsonValue>)> {
    match services::docs::delete_doc(id, conn).await {
        Ok(document) => {
            debug!("deleted document with id {id}");
            // TODO: we might want to respond with useful json
            Ok(Json(document.rows_affected))
        }
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}
