use super::CRUDTrait;
use common::models::document::Doc;
use entity::prelude::*;
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult};
use tracing_attributes::instrument;

impl CRUDTrait for Doc {
    type Entity = DocumentEntity;
    type Pk = i64;
    type AModel = DocumentAM;
}

/// Inserts a new [Doc] entity into the database.
///
/// ## Arguments
///
/// * `data` - The [Doc] entity to be inserted into the database.
/// * `conn` - A reference to the [DatabaseConnection] to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing a [Doc] struct representing the newly created [Doc] entity,
/// or a [DbErr] if the operation failed.
///
/// ## Example
///
/// ```
/// let new_doc = Doc {
///     id: None,
///     // ...
/// };
///
/// let result = create_doc(new_doc, &conn).await.unwrap();
/// assert_eq!(result.title, "Example document");
/// assert_eq!(result.content, "This is an example document.");
/// // ...
/// ```
#[instrument(skip(conn, data))]
pub async fn create_doc(data: Doc, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Create document.");
    Doc::create_entity(data, conn).await
}

/// Returns all [Doc] entities from the database.
///
/// ## Arguments
///
/// * `conn` - A reference to the [DatabaseConnection] to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing a `Vec` of [Doc] structs representing all [Doc] entities in the database,
/// or a [DbErr] if the operation failed.
///
/// ## Example
///
/// ```
/// let result = get_docs(&conn).await.unwrap();
/// assert_eq!(result.len(), 3);
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<Doc>, DbErr> {
    tracing::debug!("Requested all documents.");
    Doc::get_entities(conn).await
}

/// Returns a [Doc] entity with the specified ID from the database.
///
/// ## Arguments
///
/// * `id` - The ID of the [Doc] entity to retrieve.
/// * `conn` - A reference to the [DatabaseConnection] to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing a [Doc] struct representing the [Doc] entity with the specified ID,
/// or a [DbErr] if the operation failed.
///
/// ## Example
///
/// ```
/// let result = get_doc_by_id(1, &conn).await.unwrap();
/// assert_eq!(result.id, 1);
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Requested document with id {id}.");
    Doc::get_entity_by_pk(id, conn).await
}

/// Updates a [Doc] entity with the specified data and ID in the database.
///
/// ## Arguments
///
/// * `data` - The new data for the [Doc] entity.
/// * `id` - The ID of the [Doc] entity to update.
/// * `conn` - A reference to the [DatabaseConnection] to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing a [Doc] struct representing the updated [Doc] entity,
/// or a [DbErr] if the operation failed.
///
/// ## Example
///
/// ```
/// let mut doc = Doc {
///     id: None,
///     // ...
/// };
///
/// let result = update_doc(doc, 1, &conn).await.unwrap();
/// assert_eq!(result.id, 1);
/// // ...
/// ```
#[instrument(skip(conn, data))]
pub async fn update_doc(data: Doc, id: i64, conn: &DatabaseConnection) -> Result<Doc, DbErr> {
    tracing::debug!("Updating document with id {id}.");
    Doc::update_entity_by_pk(data, id, conn).await
}

/// Deletes a [Doc] entity with the specified ID from the database.
///
/// ## Arguments
///
/// * `id` - The ID of the [Doc] entity to delete.
/// * `conn` - A reference to the [DatabaseConnection] to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing a [DeleteResult] struct representing the result of the
/// delete operation, or a [DbErr] if the operation failed.
///
/// ## Example
///
/// ```
/// let result = delete_doc(1, &conn).await.unwrap();
/// assert_eq!(result.rows_affected, 1);
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete document with id {id}.");
    Doc::delete_entity_by_pk(id, conn).await
}

/// Returns all documents that are linked to the category with the passed `id`.
///
/// ## Arguments
///
/// * `id` - An `i32` that represents the ID of the category to search documents for.
/// * `conn` - A reference to a [DatabaseConnection] to execute the query on.
///
/// ## Returns
///
/// A `Result` containing a `Vec` of [Doc] if the query is successful, or a [DbErr] if the query fails.
///
/// ## Examples
///
/// ```
/// let docs = get_docs_by_category(1, conn).await.unwrap();
/// assert_eq!(docs.len(), 1);
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn get_docs_by_category(id: i32, conn: &DatabaseConnection) -> Result<Vec<Doc>, DbErr> {
    tracing::debug!("Fetch Documents with linked Category id {id}.");
    Ok(DocumentEntity::find()
        .filter(document::Column::CategoryId.eq(id))
        .all(conn)
        .await?
        .into_iter()
        .collect::<Vec<_>>())
}
