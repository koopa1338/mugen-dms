use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Request timeout: {0}")]
    Timeout(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("IO error: {0}")]
    Io(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        match &err {
            // Parse errors mean the caller passed a malformed record ID — that's a 422.
            surrealdb::Error::Api(surrealdb::error::Api::ParseError { .. }) => {
                ApiError::Validation(err.to_string())
            }
            _ => {
                tracing::error!("SurrealDB error: {err}");
                ApiError::Internal(err.to_string())
            }
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        tracing::error!("IO error: {err}");
        ApiError::Io(err.to_string())
    }
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) | Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({ "error": self.to_string() });
        (self.status_code(), Json(body)).into_response()
    }
}
