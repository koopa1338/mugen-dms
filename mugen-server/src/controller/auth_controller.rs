use crate::config::db::Pool;

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn login(creds: web::Json<Credentials>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    // get result of auth_service::login(creds, &pool);
    unimplemented!();
}
