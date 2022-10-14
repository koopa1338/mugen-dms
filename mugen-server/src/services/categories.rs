use super::helper;
use common::models::category::Category;
use entity::prelude::*;
use sea_orm::{
    prelude::*, DatabaseConnection,
};
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

// #[instrument(skip(conn))]
// pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
//     tracing::debug!("Delete document with id {id}.");
//     DocumentEntity::delete_by_id(id).exec(conn).await
// }
//
