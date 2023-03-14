use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::config::app::AppState;
use crate::services;
use tracing::{debug, trace};
use tracing_attributes::instrument;

use crate::error::ApiResult as Result;

use common::models::document::Doc;

/// Returns a router for the document resource.
pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/doc",
        Router::new()
            .route("/", get(doc_list).post(doc_create))
            .route("/:id", get(doc_by_id).put(doc_update).delete(doc_delete)),
    )
}

#[derive(Debug, Deserialize)]
pub struct QueryCategory {
    pub category_id: Option<i32>,
}

/// Handler for retrieving a list of documents.
///
/// ## Arguments
///
/// * `conn` - A reference to a [DatabaseConnection].
/// * `category` - A optional [QueryCategory] query parameter to filter docs by category
///
/// ## Returns
///
/// Returns a JSON response containing the retrieved documents on success, or a [DbErr] error on failure.
#[instrument(skip(conn))]
pub async fn doc_list(
    State(ref conn): State<DatabaseConnection>,
    Query(category): Query<QueryCategory>,
) -> Result<impl IntoResponse> {
    let docs = if let Some(category_id) = category.category_id {
        services::docs::get_docs_by_category(category_id, conn).await
    } else {
        services::docs::get_docs(conn).await
    };
    match docs {
        Ok(documents) => {
            debug!("Retrieved {} documents", documents.len());
            trace!("{documents:?}");
            Ok(Json(documents))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

/// Handler for retrieving a [Doc] by its ID.
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Path(id)` - ID of the [Doc] to retrieve
///
/// ## Returns
///
/// Returns a JSON-encoded document if successful, otherwise returns a [DbErr] error response.
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

/// Handler for creating a new document.
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Json(input)` - The [Doc] to create, provided as JSON data.
///
/// ## Returns
///
/// Returns a JSON response containing the newly created [Doc] if successful, otherwise returns a [DbErr] error response.
#[instrument(skip(conn, input))]
pub async fn doc_create(
    State(ref conn): State<DatabaseConnection>,
    Json(input): Json<Doc>,
) -> Result<impl IntoResponse> {
    match services::docs::create_doc(input, conn).await {
        Ok(document) => {
            debug!("Created document with id {:?}", document.id);
            trace!("{document}");
            Ok(Json(document))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

/// Handler for updating a [Doc].
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Path(id)` - ID of the [Doc] to update
/// * `Json(input)` - New JSON data for the [Doc].
///
/// ## Returns
///
/// Returns the JSON-encoded updated document if successful, otherwise returns a [DbErr] error response.
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

/// Handler for deleting a [Doc].
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Path(id)` - ID of the [Doc] to delete
///
/// ## Returns
///
/// Returns the JSON-encoded affected rows if successful, otherwise returns a [DbErr] error response.
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
