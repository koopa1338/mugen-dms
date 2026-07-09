use surrealdb::{engine::any::Any, Surreal};
use tracing_attributes::instrument;

use common::models::user::User;
use crate::{
    auth::password::db_user_to_common,
    db::{models::DbUser, parse_thing},
    error::{ApiError, ApiResult},
};

#[instrument(skip(db))]
pub async fn get_user_by_id(db: &Surreal<Any>, id: &str) -> ApiResult<User> {
    let thing = parse_thing(id)?;
    let user: Option<DbUser> = db
        .query("SELECT * FROM ONLY $id")
        .bind(("id", thing))
        .await?
        .take(0)?;
    user.map(db_user_to_common)
        .ok_or_else(|| ApiError::NotFound(format!("User {id} not found")))
}

#[instrument(skip(db))]
pub async fn get_user_by_username(db: &Surreal<Any>, username: &str) -> ApiResult<User> {
    let mut result = db
        .query("SELECT * FROM user WHERE username = $username LIMIT 1")
        .bind(("username", username.to_string()))
        .await?;

    let user: Option<DbUser> = result.take(0)?;
    user.map(db_user_to_common)
        .ok_or_else(|| ApiError::NotFound(format!("User '{username}' not found")))
}
