use crate::config::db::DbPool;
use crate::models::user::{UserLoginRequest, UserSignupRequest};

pub fn login(_creds: UserLoginRequest, _pool: &DbPool) {
    unimplemented!();
}

pub fn signup(_data: UserSignupRequest, _pool: &DbPool) {
    unimplemented!();
}
