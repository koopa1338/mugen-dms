use entity::PrimaryKeySetter;
use migration::DbErr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait,
    IntoActiveModel, PrimaryKeyTrait,
};

pub mod categories;
pub mod docs;

/// This trait provides basic CRUD operations for a given entity.
#[async_trait::async_trait]
trait CRUDTrait
where
    Self: From<<<Self as crate::services::CRUDTrait>::Entity as EntityTrait>::Model>
        + Into<Self::AModel>
        + Send,
    Vec<Self>: FromIterator<<<Self as crate::services::CRUDTrait>::Entity as EntityTrait>::Model>,
{
    type Entity: EntityTrait;
    type Pk: Into<<<<Self as crate::services::CRUDTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + std::fmt::Display
        + Clone
        + Sync
        + Send;
    type AModel: ActiveModelTrait<Entity = Self::Entity>
        + PrimaryKeySetter<Self::Entity, Self::Pk>
        + ActiveModelBehavior
        + Send;

    /// Create a new entity in the database.
    ///
    /// ## Arguments
    ///
    /// * `data` - The data to be inserted as a new entity in the database.
    /// * `conn` - A reference to a database connection.
    ///
    /// ## Returns
    ///
    /// Returns a result containing the newly created entity on success, or an error on failure.
    async fn create_entity(data: Self, conn: &DatabaseConnection) -> Result<Self, DbErr>
    where
        <<Self as crate::services::CRUDTrait>::Entity as EntityTrait>::Model:
            IntoActiveModel<Self::AModel>,
    {
        let active_model: Self::AModel = data.into();
        active_model.insert(conn).await.map(Into::into)
    }

    /// Get all entities of this type from the database.
    ///
    /// ## Arguments
    ///
    /// * `conn` - A reference to a database connection.
    ///
    /// ## Returns
    ///
    /// Returns a result containing a vector of all entities on success, or an error on failure.
    async fn get_entities(conn: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        let entities = Self::Entity::find().all(conn).await?;
        Ok(<Vec<Self>>::from_iter(entities))
    }

    /// Get an entity of this type from the database by its primary key.
    ///
    /// ## Arguments
    ///
    /// * `pk` - The primary key of the entity to be retrieved.
    /// * `conn` - A reference to a database connection.
    ///
    /// ## Returns
    ///
    /// Returns a result containing the entity on success, or an error on failure.
    async fn get_entity_by_pk(pk: Self::Pk, conn: &DatabaseConnection) -> Result<Self, DbErr> {
        Self::Entity::find_by_id(pk.clone().into())
            .one(conn)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("No Entity with primary key {pk} found")))
            .map(Into::into)
    }

    /// Update an entity of this type in the database by its primary key.
    ///
    /// ## Arguments
    ///
    /// * `data` - The new data to replace the existing entity.
    /// * `pk` - The primary key of the entity to be updated.
    /// * `conn` - A reference to a database connection.
    ///
    /// ## Returns
    ///
    /// Returns a result containing the updated entity on success, or an error on failure.
    async fn update_entity_by_pk(
        data: Self,
        pk: Self::Pk,
        conn: &DatabaseConnection,
    ) -> Result<Self, DbErr>
    where
        <<Self as crate::services::CRUDTrait>::Entity as sea_orm::EntityTrait>::Model:
            IntoActiveModel<<Self as crate::services::CRUDTrait>::AModel>,
    {
        let mut active_model: Self::AModel = data.into();
        active_model.set_pk(pk);
        Self::Entity::update(active_model)
            .exec(conn)
            .await
            .map(Into::into)
    }

    /// Deletes an entity from the database by its primary key.
    ///
    /// ## Arguments
    ///
    /// * `pk` - The primary key of the entity to be deleted.
    /// * `conn` - The database connection to execute the deletion query.
    ///
    /// ## Returns
    ///
    /// Returns a `Result` containing the number of deleted rows on success, or a `DbErr` on failure.
    async fn delete_entity_by_pk(
        pk: Self::Pk,
        conn: &DatabaseConnection,
    ) -> Result<DeleteResult, DbErr> {
        Self::Entity::delete_by_id(pk).exec(conn).await
    }
}
