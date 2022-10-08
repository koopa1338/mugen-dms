#[cfg(feature = "yew-frontend")]
use std::convert::Infallible;

#[cfg(feature = "yew-frontend")]
use axum::{http::StatusCode, response::IntoResponse, BoxError};

#[cfg(feature = "yew-frontend")]
pub async fn handle_timeout_error(err: BoxError) -> Result<impl IntoResponse, ApiError> {
    if err.is::<tower::timeout::error::Elapsed>() {
        ApiError::Timeout("Request took too long".to_string())
    } else {
        ApiError::Internal(format!("Unhandled internal error: {err}"))
    }
}

#[cfg(feature = "yew-frontend")]
pub async fn handle_io_error(error: std::io::Error) -> Result<impl IntoResponse, Infallible> {
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled error: {error}"),
    ))
}
