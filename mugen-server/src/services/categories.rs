use crate::services::docs;

use super::helper;
use common::models::category::Category;
use entity::prelude::*;
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult};
use tracing_attributes::instrument;

#[instrument(skip(conn))]
pub async fn get_categories(conn: &DatabaseConnection) -> Result<Vec<Category>, DbErr> {
    tracing::debug!("Requested all categories.");
    helper::get_entities::<CategoryEntity, Vec<Category>>(conn).await
}

#[instrument(skip(conn))]
pub async fn get_category_by_id(id: i32, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Requested category with id {id}.");
    helper::get_entity_by_pk::<CategoryEntity, Category, i32>(id, conn).await
}

#[instrument(skip(conn, data))]
pub async fn create_category(data: Category, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Create category.");
    helper::create_entity::<CategoryEntity, Category, CategoryAM>(data, conn).await
}

#[instrument(skip(conn, data))]
pub async fn update_category(
    data: Category,
    id: i32,
    conn: &DatabaseConnection,
) -> Result<Category, DbErr> {
    tracing::debug!("Updating category with id {id}.");
    helper::update_entity_by_pk::<CategoryEntity, Category, i32, CategoryAM>(data, id, conn).await
}

/// Try to delete the Category by id. This fails if the Category is used by at least one Document.
#[instrument(skip(conn))]
pub async fn delete_category(id: i32, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete category with id {id}.");
    let docs_with_category = docs::get_docs_by_category(id, conn).await?;
    if docs_with_category.is_empty() {
        tracing::debug!("No Documents linked to Category id {id}, deleting...");
        return CategoryEntity::delete_by_id(id).exec(conn).await;
    } else {
        tracing::error!(
            "Failed to delete Category with id {id}, Documents are linked: {docs_with_category:#?}"
        );
        Err(DbErr::Custom(
            "Cannot delete category, it is referenced by at least one document.".to_owned(),
        ))
    }
}
