use crate::error::ServiceError;
use crate::controller::auth_controller::Credentials;
use crate::config::db::Pool;

use actix_web::web::Data;

// Implement own Service error type
pub fn login(creds: Credentials, pool: &Data<Pool>) -> Result<String, ServiceError> {
    unimplemented!();
}
