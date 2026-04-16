pub mod middleware;
pub mod password;

pub use middleware::AuthUser;
pub use password::{PasswordAuthProvider, PasswordCredentials, RegistrationData};

use std::future::Future;
use common::models::user::User;
use serde::de::DeserializeOwned;
use crate::error::ApiError;

/// JWT claims embedded in the token.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    /// Subject: the SurrealDB user ID string (e.g. "user:abc123")
    pub sub: String,
    /// Username, for convenience in the frontend
    pub username: String,
    /// Bucket path, so the handler can resolve storage paths without a DB lookup
    pub bucket_path: String,
    /// Expiry as Unix timestamp (seconds)
    pub exp: u64,
}

/// Trait for authentication providers.
///
/// Use an associated `Credentials` type so each provider can define its own
/// credential payload — password auth uses `{username, password}`, OIDC uses
/// an authorization code + PKCE verifier, LDAP uses bind credentials, etc.
///
/// `AppState` is generic over `A: AuthProvider`, meaning the provider is
/// selected at compile time. Swap `PasswordAuthProvider` for `OidcAuthProvider`
/// in `main.rs` to change the active provider.
pub trait AuthProvider: Send + Sync + Clone {
    /// The deserialized credentials type expected from the login endpoint.
    type Credentials: DeserializeOwned + Send + 'static;

    /// The deserialized payload expected from the registration endpoint.
    /// For providers like OIDC that handle user creation out-of-band (e.g.
    /// JIT provisioning on first login), this can be `()` or a no-op type.
    type RegistrationData: DeserializeOwned + Send + 'static;

    /// Authenticate the given credentials and return a signed JWT on success.
    fn authenticate(
        &self,
        credentials: Self::Credentials,
        jwt_secret: &str,
    ) -> impl Future<Output = Result<String, ApiError>> + Send;

    /// Register a new user and return the created user record.
    fn register(
        &self,
        data: Self::RegistrationData,
    ) -> impl Future<Output = Result<User, ApiError>> + Send;
}
