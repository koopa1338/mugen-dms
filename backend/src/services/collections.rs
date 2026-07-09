use chrono::Utc;
use surrealdb::{engine::any::Any, Surreal};
use tracing_attributes::instrument;

use common::models::collection::Collection;
use crate::{
    db::{models::DbCollection, parse_thing, record_id_to_string},
    error::{ApiError, ApiResult},
};

impl From<DbCollection> for Collection {
    fn from(db: DbCollection) -> Self {
        Self {
            id: Some(record_id_to_string(&db.id)),
            name: db.name,
            created_at: db.created_at,
            updated_at: db.updated_at,
            owner_id: record_id_to_string(&db.owner),
        }
    }
}

#[instrument(skip(db))]
pub async fn get_collections(db: &Surreal<Any>, owner_id: &str) -> ApiResult<Vec<Collection>> {
    let owner = parse_thing(owner_id)?;

    let collections: Vec<DbCollection> = db
        .query("SELECT * FROM collection WHERE owner = $owner ORDER BY name ASC")
        .bind(("owner", owner))
        .await?
        .take(0)?;

    Ok(collections.into_iter().map(Into::into).collect())
}

#[instrument(skip(db))]
pub async fn get_collection_by_id(db: &Surreal<Any>, id: &str) -> ApiResult<Collection> {
    let thing = parse_thing(id)?;
    let col: Option<DbCollection> = db
        .query("SELECT * FROM ONLY $id")
        .bind(("id", thing))
        .await?
        .take(0)?;
    col.map(Into::into)
        .ok_or_else(|| ApiError::NotFound(format!("Collection {id} not found")))
}

#[instrument(skip(db))]
pub async fn create_collection(
    db: &Surreal<Any>,
    name: String,
    owner_id: &str,
) -> ApiResult<Collection> {
    let owner = parse_thing(owner_id)?;
    let now = Utc::now();

    let created: Option<DbCollection> = db
        .query(
            "CREATE collection SET
               name = $name,
               created_at = $now,
               updated_at = $now,
               owner = $owner
             RETURN AFTER",
        )
        .bind(("name", name))
        .bind(("now", now))
        .bind(("owner", owner))
        .await?
        .take(0)?;

    created
        .map(Into::into)
        .ok_or_else(|| ApiError::Internal("Collection creation returned no record".to_string()))
}

#[instrument(skip(db))]
pub async fn update_collection(
    db: &Surreal<Any>,
    id: &str,
    name: String,
) -> ApiResult<Collection> {
    let thing = parse_thing(id)?;

    let col: Option<DbCollection> = db
        .query(
            "UPDATE $id MERGE {
               name: $name,
               updated_at: time::now()
             } RETURN AFTER",
        )
        .bind(("id", thing))
        .bind(("name", name))
        .await?
        .take(0)?;

    col.map(Into::into)
        .ok_or_else(|| ApiError::NotFound(format!("Collection {id} not found")))
}

#[instrument(skip(db))]
pub async fn delete_collection(db: &Surreal<Any>, id: &str) -> ApiResult<Collection> {
    let thing = parse_thing(id)?;
    let col: Option<DbCollection> = db
        .query("DELETE $id RETURN BEFORE")
        .bind(("id", thing))
        .await?
        .take(0)?;
    col.map(Into::into)
        .ok_or_else(|| ApiError::NotFound(format!("Collection {id} not found")))
}
