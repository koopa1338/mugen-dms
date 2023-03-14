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

/// Returns a router for the category resource.
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

/// Handler for retrieving a list of categories.
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
///
/// ## Returns
///
/// Returns a JSON response containing a list of [Category] on success, or a [DbErr] error on failure.
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

/// Handler for retrieving a [Category] by its ID.
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Path(id)` - ID of the [Category] to retrieve
///
/// ## Returns
///
/// Returns a JSON-encoded category if successful, otherwise returns a [DbErr] error response.
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

/// Handler for creating a new category.
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Json(input)` - The [Category] to create, provided as JSON data.
///
/// ## Returns
///
/// Returns a JSON response containing the newly created [Category] if successful, otherwise returns a [DbErr] error response.
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

/// Handler for updating a [Category].
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Path(id)` - ID of the [Category] to update
/// * `Json(input)` - New JSON data for the [Category].
///
/// ## Returns
///
/// Returns the JSON-encoded updated category if successful, otherwise returns a [DbErr] error response.
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

/// Handler for deleting a [Category].
///
/// ## Arguments
///
/// * `State(ref conn)` - Database connection state
/// * `Path(id)` - ID of the [Category] to delete
///
/// ## Returns
///
/// Returns the JSON-encoded affected rows if successful, otherwise returns a [DbErr] error response.
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
