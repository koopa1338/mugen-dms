use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tracing_attributes::instrument;

use crate::{
    auth::{middleware::AuthUser, AuthProvider},
    config::app::AppState,
    error::{ApiError, ApiResult},
    services::documents::{self as svc, DocumentUpdate},
    storage::StorageBackend,
};

use super::{QueryCollection, QueryPagination};

pub fn router<S, A>() -> Router<AppState<S, A>>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    Router::new().nest(
        "/documents",
        Router::new()
            .route("/", get(document_list).post(document_upload))
            .route(
                "/:id",
                get(document_by_id).delete(document_delete),
            )
            .route("/:id/metadata", axum::routing::put(document_update_metadata)),
    )
}

#[instrument(skip(state))]
async fn document_list<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Query(pagination): Query<QueryPagination>,
    Query(filter): Query<QueryCollection>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let docs = if let Some(collection_id) = filter.collection_id {
        svc::get_documents_by_collection(
            &state.db,
            &collection_id,
            pagination.page,
            pagination.page_size,
        )
        .await?
    } else {
        svc::get_documents(
            &state.db,
            &auth.user_id,
            pagination.page,
            pagination.page_size,
        )
        .await?
    };

    tracing::debug!("Retrieved {} documents", docs.len());
    Ok(Json(docs))
}

#[instrument(skip(state))]
async fn document_by_id<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let doc = svc::get_document_by_id(&state.db, &id).await?;
    if doc.owner_id != auth.user_id {
        return Err(ApiError::Unauthorized("Access denied".to_string()));
    }
    Ok(Json(doc))
}

/// Upload a new document via `multipart/form-data`.
///
/// Expected fields:
/// - `file` — the file bytes (required)
/// - `collection_id` — SurrealDB collection record ID, e.g. `"collection:abc"` (optional)
/// - `tags` — comma-separated tag list (optional)
#[instrument(skip(state, multipart))]
async fn document_upload<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    mut multipart: Multipart,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut collection_id: Option<String> = None;
    let mut tags: Vec<String> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::Validation(e.to_string()))?
    {
        match field.name() {
            Some("file") => {
                filename = field.file_name().map(str::to_string);
                file_bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| ApiError::Validation(e.to_string()))?
                        .to_vec(),
                );
            }
            Some("collection_id") => {
                collection_id = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| ApiError::Validation(e.to_string()))?,
                );
            }
            Some("tags") => {
                let raw = field
                    .text()
                    .await
                    .map_err(|e| ApiError::Validation(e.to_string()))?;
                tags = raw
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();
            }
            _ => {}
        }
    }

    let bytes =
        file_bytes.ok_or_else(|| ApiError::Validation("No file field in request".to_string()))?;
    let name = filename.unwrap_or_else(|| "unnamed".to_string());
    let size = bytes.len() as u64;
    let filetype = infer::get(&bytes).map(|t| t.mime_type().to_string());
    let key = svc::make_storage_key(&name);

    state
        .storage
        .store(&auth.bucket_path, &key, &bytes)
        .await?;

    // filepath stored in DB is bucket-relative: "{bucket}/{key}"
    let filepath = format!("{}/{}", auth.bucket_path, key);

    let doc = svc::create_document(
        &state.db,
        svc::CreateDocumentParams {
            filename: name,
            filepath,
            filetype,
            size,
            owner_id: auth.user_id,
            collection_id,
            tags,
        },
    )
    .await?;

    tracing::debug!("Uploaded document: {:?}", doc.id);
    Ok((StatusCode::CREATED, Json(doc)))
}

#[derive(Debug, Deserialize)]
#[allow(clippy::option_option)]
struct UpdateMetadataBody {
    filename: Option<String>,
    collection_id: Option<Option<String>>,
    tags: Option<Vec<String>>,
    ocr_text: Option<Option<String>>,
}

#[instrument(skip(state))]
async fn document_update_metadata<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateMetadataBody>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let existing = svc::get_document_by_id(&state.db, &id).await?;
    if existing.owner_id != auth.user_id {
        return Err(ApiError::Unauthorized("Access denied".to_string()));
    }

    let doc = svc::update_document(
        &state.db,
        &id,
        DocumentUpdate {
            filename: body.filename,
            collection_id: body.collection_id,
            tags: body.tags,
            ocr_text: body.ocr_text,
        },
    )
    .await?;

    Ok(Json(doc))
}

#[instrument(skip(state))]
async fn document_delete<S, A>(
    auth: AuthUser,
    State(state): State<AppState<S, A>>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    let doc = svc::delete_document(&state.db, &id).await?;
    if doc.owner_id != auth.user_id {
        return Err(ApiError::Unauthorized("Access denied".to_string()));
    }

    // Split filepath "bucket/key" to delete from storage
    if let Some((bucket, key)) = doc.filepath.split_once('/') {
        if let Err(e) = state.storage.delete(bucket, key).await {
            // Log but don't fail — DB record is already gone
            tracing::warn!("Failed to delete file from storage: {e}");
        }
    }

    Ok(Json(doc))
}
