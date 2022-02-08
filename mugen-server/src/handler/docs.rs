use axum::extract::Path;
use axum::Json;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::models::document::Model as DocumentModel;
use crate::services;

pub fn router() -> Router {
    Router::new()
        .route("/docs", get(doc_list).post(doc_create))
        .route(
            "/docs/:id",
            get(doc_by_id).patch(doc_update).delete(doc_delete),
        )
}

pub async fn doc_list() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn doc_by_id(
    Path(id): Path<i64>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // TODO: create some sort of response type to return a StatusCode with json
    // the json should be a document or an error type with kind and message
    let result = services::docs::get_doc_by_id(id, conn).await.unwrap();
    Json(result)
}

pub async fn doc_update(
    Path(_id): Path<i64>,
    Extension(ref _conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn doc_delete(
    Path(_id): Path<i64>,
    Extension(ref _conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn doc_create(
    Json(input): Json<DocumentModel>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // TODO: see doc_by_id
    return match services::docs::create_doc(input, conn).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
}
