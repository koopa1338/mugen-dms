use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Extension, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::services;
use tracing::{debug, trace};
use tracing_attributes::instrument;

use super::ApiError;

use common::models::category::Category;
use services::categories;

pub fn router() -> Router {
    Router::new()
        .route("/categories", get(category_list).post(category_create))
        .route(
            "/categories/:id",
            get(category_by_id).patch(category_update), // TODO: .delete(category_delete),
        )
}

#[instrument(skip(conn))]
pub async fn category_list(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match categories::get_categories(conn).await {
        Ok(categories) => {
            debug!("Retrieved {} categories", categories.len());
            trace!("{categories:?}");
            Ok(Json(categories))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn))]
pub async fn category_by_id(
    Path(id): Path<i32>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match categories::get_category_by_id(id, conn).await {
        Ok(category) => {
            debug!("Retrieved category with id {:?}", category.id);
            trace!("{category}");
            Ok(Json(category))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn, input))]
pub async fn category_create(
    Json(input): Json<Category>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    let result = categories::create_category(input, conn).await;
    match result {
        Ok(category) => {
            debug!("Created category with id {:?}", category.id);
            trace!("{category}");
            Ok(Json(category))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn, input))]
pub async fn category_update(
    Path(id): Path<i32>,
    Json(input): Json<Category>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ApiError> {
    match categories::update_category(input, id, conn).await {
        Ok(category) => {
            debug!("Category with id {:?} was updated", category.id);
            trace!("New data {category}");
            Ok(Json(category))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

// #[instrument(skip(conn))]
// pub async fn doc_delete(
//     Path(id): Path<i64>,
//     Extension(ref conn): Extension<DatabaseConnection>,
// ) -> Result<impl IntoResponse, ApiError> {
//     match services::docs::delete_doc(id, conn).await {
//         Ok(document) => {
//             debug!("deleted document with id {id}");
//             // TODO: we might want to respond with useful json
//             Ok(Json(document.rows_affected))
//         }
//         Err(dberror) => Err(dberror.into()),
//     }
// }
//
