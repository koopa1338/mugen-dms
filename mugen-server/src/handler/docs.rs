use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::{services, AppState};
use tracing::{debug, trace};
use tracing_attributes::instrument;

use crate::error::ApiResult as Result;

use common::models::document::Doc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/docs", get(doc_list).post(doc_create))
        .route(
            "/docs/:id",
            get(doc_by_id).patch(doc_update).delete(doc_delete),
        )
}

#[instrument(skip(conn))]
pub async fn doc_list(
    State(ref conn): State<DatabaseConnection>,
) -> Result<impl IntoResponse> {
    match services::docs::get_docs(conn).await {
        Ok(documents) => {
            debug!("Retrieved {} documents", documents.len());
            trace!("{documents:?}");
            Ok(Json(documents))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn))]
pub async fn doc_by_id(
    State(ref conn): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
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
    State(ref conn): State<DatabaseConnection>,
    Json(input): Json<Doc>,
) -> Result<impl IntoResponse> {
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
    State(ref conn): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(input): Json<Doc>,
) -> Result<impl IntoResponse> {
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
    State(ref conn): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    match services::docs::delete_doc(id, conn).await {
        Ok(document) => {
            debug!("deleted document with id {id}");
            // TODO: we might want to respond with useful json
            Ok(Json(document.rows_affected))
        }
        Err(dberror) => Err(dberror.into()),
    }
}
