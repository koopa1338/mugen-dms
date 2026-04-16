use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tracing_attributes::instrument;

use crate::{
    auth::{middleware::AuthUser, AuthProvider},
    config::app::AppState,
    error::ApiResult,
    services::collections as svc,
    storage::StorageBackend,
};

pub fn router<S, A>() -> Router<AppState<S, A>>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    Router::new().nest(
        "/collections",
        Router::new()
            .route("/", get(collection_list).post(collection_create))
            .route(
                "/:id",
                get(collection_by_id)
                    .put(collection_update)
                    .delete(collection_delete),
            ),
    )
}

#[derive(Debug, Deserialize)]
struct CreateCollectionBody {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateCollectionBody {
    name: String,
}

#[instrument(skip(state))]
async fn collection_list<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let collections = svc::get_collections(&state.db, &auth.user_id).await?;
    tracing::debug!("Retrieved {} collections", collections.len());
    Ok(Json(collections))
}

#[instrument(skip(state))]
async fn collection_by_id<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let collection = svc::get_collection_by_id(&state.db, &id).await?;
    // Ensure the authenticated user owns this collection
    if collection.owner_id != auth.user_id {
        return Err(crate::error::ApiError::Unauthorized(
            "Access denied".to_string(),
        ));
    }
    Ok(Json(collection))
}

#[instrument(skip(state))]
async fn collection_create<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Json(body): Json<CreateCollectionBody>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let collection = svc::create_collection(&state.db, body.name, &auth.user_id).await?;
    tracing::debug!("Created collection: {:?}", collection.id);
    Ok(Json(collection))
}

#[instrument(skip(state))]
async fn collection_update<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCollectionBody>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    // Verify ownership before update
    let existing = svc::get_collection_by_id(&state.db, &id).await?;
    if existing.owner_id != auth.user_id {
        return Err(crate::error::ApiError::Unauthorized(
            "Access denied".to_string(),
        ));
    }
    let collection = svc::update_collection(&state.db, &id, body.name).await?;
    Ok(Json(collection))
}

#[instrument(skip(state))]
async fn collection_delete<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let existing = svc::get_collection_by_id(&state.db, &id).await?;
    if existing.owner_id != auth.user_id {
        return Err(crate::error::ApiError::Unauthorized(
            "Access denied".to_string(),
        ));
    }
    let collection = svc::delete_collection(&state.db, &id).await?;
    Ok(Json(collection))
}
