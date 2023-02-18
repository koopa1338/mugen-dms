use crate::services::docs;

use super::SeaServiceTrait;
use common::models::category::Category;
use entity::prelude::*;
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult};
use tracing_attributes::instrument;

impl SeaServiceTrait for Category {
    type Entity = CategoryEntity;
    type Pk = i32;
    type AModel = CategoryAM;
}

#[instrument(skip(conn))]
pub async fn get_categories(conn: &DatabaseConnection) -> Result<Vec<Category>, DbErr> {
    tracing::debug!("Requested all categories.");
    Category::get_entities(conn).await
}

#[instrument(skip(conn))]
pub async fn get_category_by_id(id: i32, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Requested category with id {id}.");
    Category::get_entity_by_pk(id, conn).await
}

#[instrument(skip(conn, data))]
pub async fn create_category(data: Category, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Create category.");
    Category::create_entity(data, conn).await
}

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
#[instrument(skip(conn))]
pub async fn delete_category(id: i32, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    tracing::debug!("Delete category with id {id}.");
    CategoryEntity::delete_by_id(id).exec(conn).await
}
