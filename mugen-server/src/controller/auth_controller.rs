use rocket::post;
use rocket::serde::json::Json;

use crate::config::db::DbPool;
use crate::models::user::UserLoginRequest;
use crate::services::auth_service;

#[post("/", data = "<user>", format = "json")]
pub async fn login(user: Json<UserLoginRequest>, connection: DbPool) {
    auth_service::login(user.into_inner(), &connection);
}

/* old actix web
pub async fn login(
    creds: web::Json<UserLoginRequestDto>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match auth_service::login(creds.into_inner(), pool.as_ref()) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}
*/
