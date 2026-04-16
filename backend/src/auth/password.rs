use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::any::Any, Surreal};

use common::models::user::User;
use crate::{
    auth::{AuthProvider, Claims},
    db::models::DbUser,
    error::{ApiError, ApiResult},
};

/// Credentials payload for password-based login.
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCredentials {
    pub username: String,
    pub password: String,
}

/// Registration payload for creating a new user.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationData {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Authentication provider that stores argon2-hashed passwords in SurrealDB.
#[derive(Clone)]
pub struct PasswordAuthProvider {
    db: Surreal<Any>,
}

impl PasswordAuthProvider {
    pub fn new(db: Surreal<Any>) -> Self {
        Self { db }
    }
}

impl AuthProvider for PasswordAuthProvider {
    type Credentials = PasswordCredentials;
    type RegistrationData = RegistrationData;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
        jwt_secret: &str,
    ) -> Result<String, ApiError> {
        let user = fetch_user_by_username(&self.db, &credentials.username).await?
            .ok_or_else(|| ApiError::Unauthorized("Invalid username or password".to_string()))?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| ApiError::Internal(format!("Invalid stored hash: {e}")))?;

        Argon2::default()
            .verify_password(credentials.password.as_bytes(), &parsed_hash)
            .map_err(|_| ApiError::Unauthorized("Invalid username or password".to_string()))?;

        issue_jwt(&user, jwt_secret)
    }

    async fn register(&self, data: Self::RegistrationData) -> Result<User, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(data.password.as_bytes(), &salt)
            .map_err(|e| ApiError::Internal(format!("Password hashing failed: {e}")))?
            .to_string();

        let bucket_path = data.username.clone();
        let now = Utc::now();

        let created: Option<DbUser> = self
            .db
            .query(
                "CREATE user SET
                   username = $username,
                   email = $email,
                   password_hash = $password_hash,
                   created_at = $now,
                   bucket_path = $bucket_path
                 RETURN AFTER",
            )
            .bind(("username", data.username))
            .bind(("email", data.email))
            .bind(("password_hash", password_hash))
            .bind(("now", now))
            .bind(("bucket_path", bucket_path))
            .await?
            .take(0)?;

        created
            .map(db_user_to_common)
            .ok_or_else(|| ApiError::Internal("User creation returned no record".to_string()))
    }
}

async fn fetch_user_by_username(
    db: &Surreal<Any>,
    username: &str,
) -> ApiResult<Option<DbUser>> {
    let mut result = db
        .query("SELECT * FROM user WHERE username = $username LIMIT 1")
        .bind(("username", username.to_string()))
        .await?;
    let user: Option<DbUser> = result.take(0)?;
    Ok(user)
}

fn issue_jwt(user: &DbUser, jwt_secret: &str) -> Result<String, ApiError> {
    let expiry = (Utc::now() + chrono::Duration::hours(24)).timestamp() as u64;
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        bucket_path: user.bucket_path.clone(),
        exp: expiry,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(format!("JWT signing failed: {e}")))
}

pub fn db_user_to_common(db: DbUser) -> User {
    User {
        id: Some(db.id.to_string()),
        username: db.username,
        email: db.email,
        created_at: db.created_at,
        bucket_path: db.bucket_path,
    }
}
