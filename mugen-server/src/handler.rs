use crate::error::ApiError;
use axum::Json;

pub mod docs;
pub mod error;

type ApiResult<T> = Result<Json<T>, ApiError>;
