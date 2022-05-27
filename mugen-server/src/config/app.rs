use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::time::Duration;

use axum::Extension;
use tower::{BoxError, ServiceBuilder};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, Router};

use clap::Parser;
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

use crate::handler::docs;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const BACKEND_PORT: u16 = 4000;
#[cfg(feature = "yew-frontend")]
const FRONTEND_PORT: u16 = 3000;

#[derive(Clone, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    #[cfg(feature = "yew-frontend")]
    #[clap(long, env)]
    pub asset_path: String,
}

#[cfg(feature = "yew-frontend")]
pub async fn static_routes(asset_path: String) {
    use crate::handler::error;
    use axum::{
        response::Redirect,
        routing::{get, get_service},
    };
    use tower_http::services::{ServeDir, ServeFile};

    let frontend = Router::new()
        .route(
            "/",
            get(|| async move { Redirect::to("/app".parse().unwrap()) }),
        )
        .nest(
            "/assets",
            get_service(ServeDir::new(&asset_path)).handle_error(error::handle_io_error),
        )
        .route(
            "/app/*path",
            get_service(ServeFile::new(format!("{asset_path}/index.html")))
                .handle_error(error::handle_io_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error::handle_timeout_error))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    tracing::debug!("Serving frontend on {LOCALHOST}:{FRONTEND_PORT}");
    serve(frontend, FRONTEND_PORT).await
}

pub async fn api_routes(conn: DatabaseConnection) {
    let backend = Router::new()
        .nest("/api", docs::router())
        .layer(Extension(conn))
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
    serve(backend, BACKEND_PORT).await
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((LOCALHOST, port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
