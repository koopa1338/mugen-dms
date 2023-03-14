use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use migration::DbErr;
use sea_orm::JsonValue;
use serde_json::json;
use std::{error::Error, fmt::Display};

/// Possible API errors that can be returned.
#[derive(Clone, Debug)]
pub enum ApiError {
    /// The requested resource was not found.
    NotFound(String),
    /// An internal server error occurred.
    Internal(String),
    /// The request timed out before a response could be obtained.
    Timeout(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    /// Returns the HTTP status code associated with the error.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
        }
    }

    /// Returns an error message associated with the error.
    fn error_message(&self) -> String {
        // TODO: custom error messages for each case
        match self {
            Self::NotFound(ref msg) | Self::Internal(ref msg) | Self::Timeout(ref msg) => {
                msg.clone()
            }
        }
    }

    /// Returns a tuple containing the HTTP status code and a JSON object representing the error message.
    fn response(&self) -> (StatusCode, Json<JsonValue>) {
        (
            self.status_code(),
            Json(json!({
                "error": self.error_message()
            })),
        )
    }
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        // TODO: custom error messages for each case
        match err {
            DbErr::TryIntoErr { from, into, source } => {
                tracing::error!("Failed to convert {source} from {from} into {into}");
                ApiError::Internal("Conversion error".to_string())
            }
            DbErr::Conn(_) | DbErr::ConnectionAcquire => {
                Self::Timeout("Connection Error".to_string())
            }
            DbErr::Exec(runtime_error) | DbErr::Query(runtime_error) => {
                tracing::error!("Runtime Error {runtime_error}");
                Self::Internal("Runtime Error".to_string())
            }
            DbErr::RecordNotFound(msg) => Self::NotFound(msg),
            DbErr::AttrNotSet(msg)
            | DbErr::Custom(msg)
            | DbErr::Type(msg)
            | DbErr::Json(msg)
            | DbErr::Migration(msg) => {
                tracing::error!(msg);
                Self::Internal(msg)
            }
            DbErr::ConvertFromU64(msg) => {
                let error_msg = msg.to_string();
                tracing::error!(error_msg);
                Self::Internal(error_msg)
            }
            DbErr::RecordNotInserted
            | DbErr::RecordNotUpdated
            | DbErr::UnpackInsertId
            | DbErr::UpdateGetPrimaryKey => {
                tracing::error!("Internal server error");
                Self::Internal("Internal server error".to_string())
            }
        }
    }
}

impl Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Statuscode: {}", self.status_code())?;
        writeln!(f, "Body: {:?}", self.error_message())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.response().into_response()
    }
}
