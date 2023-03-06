use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use sea_orm::DatabaseConnection;

use crate::config::app::AppState;
use crate::services;
use tracing::{debug, trace};
use tracing_attributes::instrument;

use crate::error::ApiResult as Result;

use common::models::category::Category;
use services::categories;

pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/category",
        Router::new()
            .route("/", get(category_list).post(category_create))
            .route(
                "/:id",
                get(category_by_id)
                    .put(category_update)
                    .delete(category_delete),
            ),
    )
}

#[instrument(skip(conn))]
pub async fn category_list(
    State(ref conn): State<DatabaseConnection>,
) -> Result<impl IntoResponse> {
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
    State(ref conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
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
    State(ref conn): State<DatabaseConnection>,
    Json(input): Json<Category>,
) -> Result<impl IntoResponse> {
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
    State(ref conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(input): Json<Category>,
) -> Result<impl IntoResponse> {
    match categories::update_category(input, id, conn).await {
        Ok(category) => {
            debug!("Category with id {:?} was updated", category.id);
            trace!("New data {category}");
            Ok(Json(category))
        }
        Err(dberror) => Err(dberror.into()),
    }
}

#[instrument(skip(conn))]
pub async fn category_delete(
    State(ref conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    match services::categories::delete_category(id, conn).await {
        Ok(category) => {
            debug!("deleted category with id {id}");
            // TODO: we might want to respond with useful json
            Ok(Json(category.rows_affected))
        }
        Err(dberror) => Err(dberror.into()),
    }
}
