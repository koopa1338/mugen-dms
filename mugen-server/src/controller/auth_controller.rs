use actix_web::{web, HttpResponse, Result};
use crate::models::user::UserLoginRequestDto;
use crate::services::auth_service;
use crate::config::db::Pool;

pub async fn login(creds: web::Json<UserLoginRequestDto>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match auth_service::login(creds.into_inner(), pool.as_ref()) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}
