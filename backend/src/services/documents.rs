use chrono::Utc;
use surrealdb::{engine::any::Any, Surreal};
use tracing_attributes::instrument;
use uuid::Uuid;

use common::models::document::Document;
use crate::{
    db::{models::DbDocument, parse_thing, record_id_to_string},
    error::{ApiError, ApiResult},
};

impl From<DbDocument> for Document {
    fn from(db: DbDocument) -> Self {
        Self {
            id: Some(record_id_to_string(&db.id)),
            filename: db.filename,
            filepath: db.filepath,
            filetype: db.filetype,
            size: db.size,
            created_at: db.created_at,
            updated_at: db.updated_at,
            collection_id: db.collection.map(|t| record_id_to_string(&t)),
            owner_id: record_id_to_string(&db.owner),
            ocr_text: db.ocr_text,
            tags: db.tags,
        }
    }
}

/// Generate a storage key from the original filename.
/// e.g. "invoice.pdf" → "550e8400-e29b-41d4-a716-446655440000-invoice.pdf"
pub fn make_storage_key(filename: &str) -> String {
    format!("{}-{}", Uuid::new_v4(), filename)
}

#[instrument(skip(db))]
pub async fn get_documents(
    db: &Surreal<Any>,
    owner_id: &str,
    page: Option<u64>,
    page_size: Option<u64>,
) -> ApiResult<Vec<Document>> {
    let limit = page_size.unwrap_or(50);
    let start = page.unwrap_or(0) * limit;
    let owner = parse_thing(owner_id)?;

    let docs: Vec<DbDocument> = db
        .query("SELECT * FROM document WHERE owner = $owner ORDER BY created_at DESC LIMIT $limit START $start")
        .bind(("owner", owner))
        .bind(("limit", limit))
        .bind(("start", start))
        .await?
        .take(0)?;

    Ok(docs.into_iter().map(Into::into).collect())
}

#[instrument(skip(db))]
pub async fn get_document_by_id(db: &Surreal<Any>, id: &str) -> ApiResult<Document> {
    let thing = parse_thing(id)?;
    let doc: Option<DbDocument> = db
        .query("SELECT * FROM ONLY $id")
        .bind(("id", thing))
        .await?
        .take(0)?;
    doc.map(Into::into)
        .ok_or_else(|| ApiError::NotFound(format!("Document {id} not found")))
}

#[instrument(skip(db))]
pub async fn get_documents_by_collection(
    db: &Surreal<Any>,
    collection_id: &str,
    page: Option<u64>,
    page_size: Option<u64>,
) -> ApiResult<Vec<Document>> {
    let limit = page_size.unwrap_or(50);
    let start = page.unwrap_or(0) * limit;
    let collection = parse_thing(collection_id)?;

    let docs: Vec<DbDocument> = db
        .query("SELECT * FROM document WHERE collection = $col ORDER BY created_at DESC LIMIT $limit START $start")
        .bind(("col", collection))
        .bind(("limit", limit))
        .bind(("start", start))
        .await?
        .take(0)?;

    Ok(docs.into_iter().map(Into::into).collect())
}

/// Parameters for creating a document record.
#[derive(Debug)]
pub struct CreateDocumentParams {
    pub filename: String,
    pub filepath: String,
    pub filetype: Option<String>,
    pub size: u64,
    pub owner_id: String,
    pub collection_id: Option<String>,
    pub tags: Vec<String>,
}

/// Create a document DB record. The file must already be written to storage before calling this.
#[instrument(skip(db))]
pub async fn create_document(db: &Surreal<Any>, params: CreateDocumentParams) -> ApiResult<Document> {
    let owner = parse_thing(&params.owner_id)?;
    let collection: Option<_> = params.collection_id.map(|s| parse_thing(&s)).transpose()?;
    let now = Utc::now();

    let created: Option<DbDocument> = db
        .query(
            "CREATE document SET
               filename = $filename,
               filepath = $filepath,
               filetype = $filetype,
               size = $size,
               created_at = $now,
               updated_at = $now,
               owner = $owner,
               collection = $collection,
               ocr_text = NONE,
               tags = $tags
             RETURN AFTER",
        )
        .bind(("filename", params.filename))
        .bind(("filepath", params.filepath))
        .bind(("filetype", params.filetype))
        .bind(("size", params.size.cast_signed()))
        .bind(("now", now))
        .bind(("owner", owner))
        .bind(("collection", collection))
        .bind(("tags", params.tags))
        .await?
        .take(0)?;

    created
        .map(Into::into)
        .ok_or_else(|| ApiError::Internal("Document creation returned no record".to_string()))
}

#[derive(Debug, Default)]
pub struct DocumentUpdate {
    pub filename: Option<String>,
    pub collection_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ocr_text: Option<String>,
}

#[instrument(skip(db))]
pub async fn update_document(
    db: &Surreal<Any>,
    id: &str,
    update: DocumentUpdate,
) -> ApiResult<Document> {
    let thing = parse_thing(id)?;
    let collection: Option<_> = update.collection_id.as_ref().map(|s| parse_thing(s)).transpose()?;

    let doc: Option<DbDocument> = db
        .query(
            "UPDATE $id MERGE {
               updated_at: time::now(),
               filename: $filename ?? this.filename,
               collection: $collection ?? this.collection,
               tags: $tags ?? this.tags,
               ocr_text: $ocr_text ?? this.ocr_text
             } RETURN AFTER",
        )
        .bind(("id", thing))
        .bind(("filename", update.filename))
        .bind(("collection", collection))
        .bind(("tags", update.tags))
        .bind(("ocr_text", update.ocr_text))
        .await?
        .take(0)?;

    doc.map(Into::into)
        .ok_or_else(|| ApiError::NotFound(format!("Document {id} not found")))
}

/// Delete the document record and return its filepath so the caller can also remove the file.
#[instrument(skip(db))]
pub async fn delete_document(db: &Surreal<Any>, id: &str) -> ApiResult<Document> {
    let thing = parse_thing(id)?;
    let doc: Option<DbDocument> = db
        .query("DELETE $id RETURN BEFORE")
        .bind(("id", thing))
        .await?
        .take(0)?;
    doc.map(Into::into)
        .ok_or_else(|| ApiError::NotFound(format!("Document {id} not found")))
}
