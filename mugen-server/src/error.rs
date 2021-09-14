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
            dieselError::QueryBuilderError(msg) => {
                Self::DatabaseError(format!("Error in Querybuilder: {}", msg))
            }
            dieselError::DeserializationError(msg) => {
                Self::DatabaseError(format!("Error on Deserialization: {}", msg))
            }
            dieselError::SerializationError(msg) => {
                Self::DatabaseError(format!("Error on Serialization: {}", msg))
            }
            dieselError::RollbackTransaction => {
                Self::DatabaseError("Error while transaction rollback".to_string())
            }
            dieselError::AlreadyInTransaction => {
                Self::DatabaseError("Already in transaction".to_string())
            }
            dieselError::__Nonexhaustive => Self::UnknownError,
        }
    }
}
