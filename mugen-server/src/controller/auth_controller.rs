use rocket::post;
use rocket::serde::json::Json;

use crate::config::db::DbPool;
use crate::models::user::{UserLoginRequest, UserSignupRequest};
use crate::services::auth_service;

#[post("/login", data = "<user>", format = "json")]
pub async fn login(user: Json<UserLoginRequest>, connection: DbPool) {
    auth_service::login(user.into_inner(), &connection);
}

#[post("/signup", data = "<user>", format = "json")]
pub async fn signup(user: Json<UserSignupRequest>, connection: DbPool) {
    auth_service::signup(user.into_inner(), &connection);
}
