use crate::error::{ServiceError, ServiceResult};
use crate::models::user::{User, UserLoginRequestDto, UserLoginResponseDto};
use crate::config::db::Pool;
use crate::models::schema::users::dsl::*;
use bcrypt::verify;
use diesel::prelude::*;


pub fn login(creds: UserLoginRequestDto, pool: &Pool) -> ServiceResult<UserLoginResponseDto> {
    let conn = pool.clone().get().unwrap();
    let mut items = users
        .filter(username.eq(&creds.username))
        .load::<User>(&conn)?;
    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &creds.password) {
            if matching {
                let user_dto = UserLoginResponseDto {
                    username: user.username,
                };
                return Ok(user_dto);
            }
        }
    }
    Err(ServiceError::AuthenticationError("Login failed.".to_string()))
}
