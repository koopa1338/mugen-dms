use axum::{
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use migration::DbErr;
use std::{error::Error, fmt::Display};
use tracing::error;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Internal(String),
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_message(&self) -> String {
        match self {
            ApiError::NotFound(msg) | ApiError::Internal(msg) => msg.clone(),
        }
    }

    fn response(&self) -> (StatusCode, String) {
        (self.status_code(), self.error_message())
    }
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
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
