use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::time::Duration;

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

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    database: DatabaseConnection,
}

impl AppState {
    pub fn new(database: DatabaseConnection) -> Self {
        Self {
            database
        }
    }
}

pub async fn api_routes(app_state: AppState) {
    let backend = Router::new()
        .nest(
            "/api",
            Router::merge(
                docs::router().with_state(app_state.clone()),
                categories::router().with_state(app_state.clone()),
            ),
        )
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
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
    serve(backend, BACKEND_PORT).await;
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((LOCALHOST, port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error serving app service");
}
