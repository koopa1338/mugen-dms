use super::CRUDTrait;
use common::models::category::Category;
use entity::prelude::*;
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult};
use tracing_attributes::instrument;

impl CRUDTrait for Category {
    type Entity = CategoryEntity;
    type Pk = i32;
    type AModel = CategoryAM;
}

/// Creates a new category with the given data and returns it on success.
/// 
/// # Arguments
/// 
/// * `data` - A `Category` object containing the data for the new category.
/// * `conn` - A reference to a `DatabaseConnection` object representing the database connection.
/// 
/// # Returns
///
/// A `Result` object containing the newly created `Category` object on success, or a `DbErr` error on failure.
///
/// # Example
/// 
/// ```rust
/// let category = Category {
///     id: None,
///     title: String::from("My Category"),
/// };
/// let result = create_category(category, &conn);
/// assert!(result.is_ok());
/// // ...
/// ```
#[instrument(skip(conn, data))]
pub async fn create_category(data: Category, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Create category.");
    Category::create_entity(data, conn).await
}

/// Gets all categories from the database and returns them on success.
/// 
/// # Arguments
///
/// * `conn` - A reference to a `DatabaseConnection` object representing the database connection.
///
/// # Returns
///
/// A `Result` object containing a `Vec` of `Category` objects on success, or a `DbErr` error on failure.
///
/// # Example
///
/// ```rust
/// let result = get_categories(&conn);
/// assert!(result.is_ok());
///
/// let categories = result.unwrap();
/// assert!(categories.len() >= 0);
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn get_categories(conn: &DatabaseConnection) -> Result<Vec<Category>, DbErr> {
    tracing::debug!("Requested all categories.");
    Category::get_entities(conn).await
}

/// Gets a category with the specified ID from the database and returns it on success.
///
/// ## Arguments
///
/// * `id` - An `i32` value representing the ID of the category to retrieve.
/// * `conn` - A reference to a `DatabaseConnection` object representing the database connection.
///
/// ## Returns
///
/// A `Result` object containing the retrieved `Category` object on success, or a `DbErr` error on failure.
///
/// ## Example
///
/// ```rust
/// let result = get_category_by_id(1, &conn);
/// assert!(result.is_ok());
///
/// let category = result.unwrap();
/// assert_eq!(category.title, "Example Category");
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn get_category_by_id(id: i32, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Requested category with id {id}.");
    Category::get_entity_by_pk(id, conn).await
}

/// Updates a category entity in the database with new data.
///
/// ## Arguments
///
/// * `data` - A `Category` struct representing the updated data for the category.
/// * `id` - The primary key ID of the `Category` entity to be updated.
/// * `conn` - A reference to the `DatabaseConnection` to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing the updated `Category` entity, or a `DbErr` if the operation failed.
///
/// ## Example
///
/// ```
/// let category = Category {
///     id: Some(1),
///     name: "new category name".to_string(),
/// };
///
/// let result = update_category(category, 1, &conn).await.unwrap();
/// assert_eq!(result.title, "new category name");
/// // ...
/// ```
#[instrument(skip(conn, data))]
pub async fn update_category(
    data: Category,
    id: i32,
    conn: &DatabaseConnection,
) -> Result<Category, DbErr> {
    tracing::debug!("Updating category with id {id}.");
    Category::update_entity_by_pk(data, id, conn).await
}

/// Try to delete the Category by id. This fails if the Category is used by at least one Document.
/// Deletes a category entity from the database with the given ID.
///
/// ## Arguments
///
/// * `id` - The primary key ID of the `Category` entity to be deleted.
/// * `conn` - A reference to the `DatabaseConnection` to be used for the operation.
///
/// ## Returns
///
/// A `Result` containing a `DeleteResult` struct representing the result of the delete operation,
/// or a `DbErr` if the operation failed.
///
/// ## Example
///
/// ```
/// let result = delete_category(1, &conn).await.unwrap();
/// assert_eq!(result.rows_affected, 1);
/// // ...
/// ```
#[instrument(skip(conn))]
pub async fn delete_category(id: i32, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete category with id {id}.");
    Category::delete_entity_by_pk(id, conn).await
}
