use crate::config::db::DbPool;
use crate::error::ServiceResult;
use crate::models::user::{UserLoginRequest, UserSignupRequest};

pub fn login(_creds: UserLoginRequest, _pool: &DbPool) -> ServiceResult<()> {
    unimplemented!();
}

pub fn signup(_data: UserSignupRequest, _pool: &DbPool) -> ServiceResult<()> {
    unimplemented!();
}
