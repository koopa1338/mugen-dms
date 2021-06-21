use diesel::result::Error as dieselError;

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;

pub enum ServiceError {
    AuthenticationError(String),
    DatabaseError(String),
    UnknownError,
}

impl From<dieselError> for ServiceError {
    fn from(err: dieselError) -> Self {
        match err {
            dieselError::InvalidCString(_) => Self::UnknownError,
            dieselError::DatabaseError(_, info) => Self::DatabaseError(info.message().to_string()),
            dieselError::NotFound => Self::DatabaseError("Not Found Error".to_string()),
            _ => Self::DatabaseError("Other Error.".to_string()),
        }
    }
}
