use common::models::category::Category;
use entity::prelude::*;
use sea_orm::{
    prelude::*, ActiveValue::NotSet, DatabaseConnection, DeleteResult, IntoActiveModel, Set,
};
use tracing_attributes::instrument;

#[instrument(skip(conn))]
pub async fn get_categories(conn: &DatabaseConnection) -> Result<Vec<Category>, DbErr> {
    tracing::debug!("Requested all categories.");
    let categories = CategoryEntity::find().all(conn).await?;

    Ok(Vec::<Category>::from_iter(categories))
}

#[instrument(skip(conn))]
pub async fn get_category_by_id(id: i64, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Requested category with id {id}.");
    CategoryEntity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("No Category with id {id} found")))
        .map(Into::into)
}

#[instrument(skip(conn, data))]
pub async fn create_category(data: Category, conn: &DatabaseConnection) -> Result<Category, DbErr> {
    tracing::debug!("Create category.");
    let active_model: CategoryAM = data.into();

    active_model.insert(conn).await.map(Into::into)
}

#[instrument(skip(conn, data))]
pub async fn update_category(
    data: Category,
    id: i64,
    conn: &DatabaseConnection,
) -> Result<Category, DbErr> {
    tracing::debug!("Updating category with id {id}.");
    let mut active_model: CategoryAM = data.into();
    active_model.id = Set(id);
    CategoryEntity::update(active_model)
        .exec(conn)
        .await
        .map(Into::into)
}

// #[instrument(skip(conn))]
// pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
//     tracing::debug!("Delete document with id {id}.");
//     DocumentEntity::delete_by_id(id).exec(conn).await
// }
//
