use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, Router};
use clap::Parser;
use surrealdb::{engine::any::Any, Surreal};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::{Any as AnyOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;
use axum::serve;

use crate::auth::middleware::JwtSecret;
use crate::auth::AuthProvider;
use crate::storage::StorageBackend;

const LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

#[derive(Clone, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    /// Username for remote SurrealDB authentication (omit for embedded mode)
    #[clap(long, env)]
    pub db_user: Option<String>,

    /// Password for remote SurrealDB authentication
    #[clap(long, env)]
    pub db_pass: Option<String>,

    /// Root directory for filesystem file storage
    #[clap(long, env, default_value = "./storage")]
    pub storage_root: PathBuf,

    /// Secret key for JWT signing and verification
    #[clap(long, env)]
    pub jwt_secret: String,

    /// Port to listen on
    #[clap(long, env, default_value_t = 4000)]
    pub port: u16,
}

/// Global application state.
///
/// `S` is the storage backend (e.g. `FilesystemStorage`; swap for `S3Storage` later).
/// `A` is the auth provider (e.g. `PasswordAuthProvider`; swap for `OidcAuthProvider` later).
#[derive(Clone)]
pub struct AppState<S, A> {
    pub db: Surreal<Any>,
    pub storage: S,
    pub auth: A,
    pub jwt_secret: String,
    pub port: u16,
}

impl<S, A> AppState<S, A> {
    pub fn new(db: Surreal<Any>, storage: S, auth: A, jwt_secret: String, port: u16) -> Self {
        Self { db, storage, auth, jwt_secret, port }
    }
}

/// Mounts all API routes and starts the backend server.
pub async fn api_routes<S, A>(app_state: AppState<S, A>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
{
    use crate::handler::{auth, collections, documents};

    let port = app_state.port;
    let jwt_secret = app_state.jwt_secret.clone();

    let backend = Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(auth::router())
                .merge(documents::router())
                .merge(collections::router()),
        )
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_origin(AnyOrigin))
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                // Inject the JWT secret into request extensions so AuthUser can find it
                .layer(axum::middleware::from_fn(move |mut req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| {
                    let secret = jwt_secret.clone();
                    async move {
                        req.extensions_mut().insert(JwtSecret(secret));
                        next.run(req).await
                    }
                }))
                .into_inner(),
        );

    tracing::debug!("Serving backend on {LOCALHOST}:{port}");

    let listener = TcpListener::bind((LOCALHOST, port))
        .await
        .expect("Error creating TCP listener");
    serve(listener, backend)
        .await?;

    Ok(())
}
