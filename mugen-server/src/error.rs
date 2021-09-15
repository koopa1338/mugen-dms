use diesel::result::Error as DieselError;

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;

pub enum ServiceError {
    AuthenticationError(String),
    DatabaseError(String),
    UnknownError,
}

impl From<DieselError> for ServiceError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::InvalidCString(_) => Self::UnknownError,
            DieselError::DatabaseError(_, info) => Self::DatabaseError(info.message().to_string()),
            DieselError::NotFound => Self::DatabaseError("Not Found Error".to_string()),
            DieselError::QueryBuilderError(msg) => {
                Self::DatabaseError(format!("Error in Querybuilder: {}", msg))
            }
            DieselError::DeserializationError(msg) => {
                Self::DatabaseError(format!("Error on Deserialization: {}", msg))
            }
            DieselError::SerializationError(msg) => {
                Self::DatabaseError(format!("Error on Serialization: {}", msg))
            }
            DieselError::RollbackTransaction => {
                Self::DatabaseError("Error while transaction rollback".to_string())
            }
            DieselError::AlreadyInTransaction => {
                Self::DatabaseError("Already in transaction".to_string())
            }
            DieselError::__Nonexhaustive => Self::UnknownError,
        }
    }
}
