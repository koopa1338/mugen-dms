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
            DbErr::Conn(msg)
            | DbErr::Exec(msg)
            | DbErr::Query(msg)
            | DbErr::Custom(msg)
            | DbErr::Type(msg)
            | DbErr::Json(msg)
            | DbErr::Migration(msg) => {
                error!(msg);
                Self::Internal(msg)
            }
            DbErr::RecordNotFound(msg) => {
                error!(msg);
                Self::NotFound(msg)
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
