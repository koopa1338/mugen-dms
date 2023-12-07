use axum::serve;
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use tower::{BoxError, ServiceBuilder};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, Router};

use clap::Parser;
use tower_http::trace::TraceLayer;

use crate::handler::categories;
use crate::handler::docs;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const BACKEND_PORT: u16 = 4000;

#[derive(Clone, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
}

/// The global state of the application that holds the connection to the database.
#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    /// The connection to the database.
    database: DatabaseConnection,
}

impl AppState {
    /// Creates a new instance of `AppState` with the given `DatabaseConnection`.
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}

/// Mounts the API routes onto a router and serves the backend on a specified port.
pub async fn api_routes(app_state: AppState) {
    let backend = Router::new()
        .nest("/api", Router::merge(docs::router(), categories::router()))
        .with_state(app_state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_origin(Any))
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
                .into_inner(),
        );
    tracing::debug!("Serving backend on {LOCALHOST}:{BACKEND_PORT}");
    init(backend, BACKEND_PORT).await;
}

/// Starts serving the provided `app` on the specified `port`.
async fn init(app: Router, port: u16) {
    let listener = TcpListener::bind((LOCALHOST, port))
        .await
        .expect("Error creating tcp listener on {LOCALHOST}:{port}");
    serve(listener, app)
        .await
        .expect("Error serving app service");
}
