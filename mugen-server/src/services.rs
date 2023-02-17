use entity::PrimaryKeySetter;
use migration::DbErr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait,
};

pub mod categories;
pub mod docs;

#[async_trait::async_trait]
trait SeaServiceTrait
where
    Self: From<<<Self as crate::services::SeaServiceTrait>::Entity as EntityTrait>::Model>
        + Into<Self::AModel>
        + Send,
    Vec<Self>:
        FromIterator<<<Self as crate::services::SeaServiceTrait>::Entity as EntityTrait>::Model>,
{
    type Entity: EntityTrait;
    type Pk: Into<<<<Self as crate::services::SeaServiceTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + std::fmt::Display
        + Clone
        + Sync
        + Send;
    type AModel: ActiveModelTrait<Entity = Self::Entity>
        + PrimaryKeySetter<Self::Entity, Self::Pk>
        + ActiveModelBehavior
        + Send;

    async fn get_entity_by_pk(pk: Self::Pk, conn: &DatabaseConnection) -> Result<Self, DbErr> {
        Self::Entity::find_by_id(pk.clone().into())
            .one(conn)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("No Entity with primary key {pk} found")))
            .map(Into::into)
    }

    async fn get_entities(conn: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        let entities = Self::Entity::find().all(conn).await?;
        Ok(<Vec<Self>>::from_iter(entities))
    }

    async fn update_entity_by_pk(
        data: Self,
        pk: Self::Pk,
        conn: &DatabaseConnection,
    ) -> Result<Self, DbErr>
    where
        <<Self as crate::services::SeaServiceTrait>::Entity as sea_orm::EntityTrait>::Model:
            IntoActiveModel<<Self as crate::services::SeaServiceTrait>::AModel>,
    {
        let mut active_model: Self::AModel = data.into();
        active_model.set_pk(pk);
        Self::Entity::update(active_model)
            .exec(conn)
            .await
            .map(Into::into)
    }

    async fn create_entity(data: Self, conn: &DatabaseConnection) -> Result<Self, DbErr>
    where
        <<Self as crate::services::SeaServiceTrait>::Entity as EntityTrait>::Model:
            IntoActiveModel<Self::AModel>,
    {
        let active_model: Self::AModel = data.into();
        active_model.insert(conn).await.map(Into::into)
    }
}
