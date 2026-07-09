use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::auth::Claims;

/// The validated JWT claims, extracted from the `Authorization: Bearer <token>` header.
///
/// Use this as an extractor in protected handlers:
/// ```rust,ignore
/// async fn my_handler(auth: AuthUser, ...) { ... }
/// ```
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub username: String,
    pub bucket_path: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let reject = |msg: &'static str| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({ "error": msg })),
            )
        };

        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned)
            .ok_or_else(|| reject("Missing Authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .map(str::to_owned)
            .ok_or_else(|| reject("Authorization header must use Bearer scheme"))?;

        let jwt_secret = parts
            .extensions
            .get::<JwtSecret>()
            .ok_or_else(|| reject("JWT secret not configured"))?;

        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.0.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| reject("Invalid or expired token"))?
        .claims;

        Ok(AuthUser {
            user_id: claims.sub,
            username: claims.username,
            bucket_path: claims.bucket_path,
        })
    }
}

/// Extension type that carries the JWT secret into the request pipeline.
#[derive(Clone)]
pub struct JwtSecret(pub String);
