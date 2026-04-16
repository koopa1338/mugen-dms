use std::future::Future;
use std::pin::Pin;
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

    // `FromRequestParts` in axum-core 0.4 is defined with `#[async_trait]`, which expands the
    // trait method to the explicit boxed-future signature below. We must match that expansion
    // exactly — native `async fn` produces a different RPIT capture and triggers E0195.
    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        // Extract all needed data from `parts` upfront so the async block owns everything.
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let jwt_secret = parts
            .extensions
            .get::<JwtSecret>()
            .map(|s| s.0.clone());

        Box::pin(async move {
            let reject = |msg: &'static str| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({ "error": msg })),
                )
            };

            let auth_header =
                auth_header.ok_or_else(|| reject("Missing Authorization header"))?;
            let token = auth_header
                .strip_prefix("Bearer ")
                .map(str::to_owned)
                .ok_or_else(|| reject("Authorization header must use Bearer scheme"))?;
            let jwt_secret =
                jwt_secret.ok_or_else(|| reject("JWT secret not configured"))?;

            let claims = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| reject("Invalid or expired token"))?
            .claims;

            Ok(AuthUser {
                user_id: claims.sub,
                username: claims.username,
                bucket_path: claims.bucket_path,
            })
        })
    }
}

/// Extension type that carries the JWT secret into the request pipeline.
#[derive(Clone)]
pub struct JwtSecret(pub String);
