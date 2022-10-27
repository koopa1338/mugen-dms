use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use migration::DbErr;
use sea_orm::JsonValue;
use serde_json::json;
use std::{error::Error, fmt::Display};
use tracing::error;

#[derive(Clone, Debug)]
pub enum ApiError {
    NotFound(String),
    Internal(String),
    Timeout(String),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
        }
    }

    fn error_message(&self) -> String {
        // TODO: custom error messages for each case
        match self {
            Self::NotFound(msg) | Self::Internal(msg) | Self::Timeout(msg) => msg.clone(),
        }
    }

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
                error!("Failed to convert {source} from {from} into {into}");
                ApiError::Internal("Conversion error".to_string())
            }
            DbErr::Conn(runtime_error)
            | DbErr::Exec(runtime_error)
            | DbErr::Query(runtime_error) => {
                error!("Runtime Error {runtime_error}");
                Self::Internal("Runtime Error".to_string())
            }
            DbErr::RecordNotFound(msg) => Self::NotFound(msg),
            DbErr::AttrNotSet(msg)
            | DbErr::Custom(msg)
            | DbErr::Type(msg)
            | DbErr::Json(msg)
            | DbErr::Migration(msg) => {
                error!(msg);
                Self::Internal(msg)
            }
            DbErr::UnpackInsertId | DbErr::UpdateGetPrimeryKey | DbErr::ConnectionAcquire => {
                error!("Internal server error");
                Self::Internal("Internal server error".to_string())
            }
            DbErr::ConvertFromU64(msg) => {
                let error_msg = msg.to_string();
                error!(error_msg);
                Self::Internal(error_msg)
            }
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Statuscode: {}", self.status_code())?;
        writeln!(f, "Body: {:?}", self.error_message())
    }
}

impl Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.response().into_response()
    }
}
