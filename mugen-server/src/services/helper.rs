use entity::PrimaryKeySetter;
use migration::DbErr;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PrimaryKeyTrait, Set, ActiveModelBehavior,
};

pub async fn get_entity_by_pk<T, R, P>(pk: P, conn: &DatabaseConnection) -> Result<R, DbErr>
where
    T: EntityTrait,
    R: From<T::Model>,
    P: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + std::fmt::Display + Clone,
{
    T::find_by_id(pk.clone().into())
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("No Entity with primary key {pk} found")))
        .map(Into::into)
}

pub async fn get_entities<T, R>(conn: &DatabaseConnection) -> Result<R, DbErr>
where
    T: EntityTrait,
    R: FromIterator<T::Model>
{
    let entities = T::find().all(conn).await?;
    Ok(R::from_iter(entities))
}

pub async fn update_entity_by_pk<T, R, P, A>(
    data: R,
    pk: P,
    conn: &DatabaseConnection,
) -> Result<R, DbErr>
where
    T: EntityTrait,
    R: From<T::Model> + Into<A>,
    P: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + std::fmt::Display + Clone,
    A: ActiveModelTrait<Entity = T> + PrimaryKeySetter<T, P>,
    T::Model: IntoActiveModel<A>,
{
    let mut active_model: A = data.into();
    active_model.set_pk(pk);
    T::update(active_model).exec(conn).await.map(Into::into)
}

pub async fn create_entity<T, R, A>(
    data: R,
    conn: &DatabaseConnection,
) -> Result<R, DbErr>
where
    T: EntityTrait,
    R: From<T::Model> + Into<A>,
    A: ActiveModelTrait<Entity = T> + ActiveModelBehavior + Send,
    T::Model: IntoActiveModel<A>,
{
    let active_model: A = data.into();
    active_model.insert(conn).await.map(Into::into)
}
