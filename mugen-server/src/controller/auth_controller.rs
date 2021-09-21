use rocket::post;
use rocket::serde::json::Json;

use crate::config::db::DbPool;
use crate::models::user::{UserLoginRequest, UserSignupRequest};
use crate::services::auth_service;

#[post("/login", data = "<user>", format = "json")]
pub async fn login(user: Json<UserLoginRequest>, connection: DbPool) {
    match auth_service::login(user.into_inner(), &connection) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

#[post("/signup", data = "<user>", format = "json")]
pub async fn signup(user: Json<UserSignupRequest>, connection: DbPool) {
    match auth_service::signup(user.into_inner(), &connection) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
