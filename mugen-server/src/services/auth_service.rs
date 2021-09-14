use crate::config::db::DbPool;
use crate::models::user::UserLoginRequest;

pub fn login(_creds: UserLoginRequest, _pool: &DbPool) {
    unimplemented!();
    /*
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
    Err(ServiceError::AuthenticationError(
        "Login failed.".to_string(),
    ))
    */
}
