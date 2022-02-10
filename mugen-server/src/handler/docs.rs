use axum::extract::Path;
use axum::Json;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::config::db::DbErrJsonValue;
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
) -> Result<Json<DocumentModel>, (StatusCode, Json<DbErrJsonValue>)> {
    //TODO: make this a function that takes the service call as a Fn
    let result = services::docs::get_doc_by_id(id, conn).await;
    match result {
        Ok(document) => Ok(Json(document)),
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
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
) -> Result<Json<DocumentModel>, (StatusCode, Json<DbErrJsonValue>)> {
    let result = services::docs::create_doc(input, conn).await;
    match result {
        Ok(document) => Ok(Json(document)),
        Err(dberror) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(dberror.into()))),
    }
}
