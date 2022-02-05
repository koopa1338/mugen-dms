use std::net::SocketAddr;
use std::time::Duration;
use std::{convert::Infallible, net::Ipv4Addr};

use tower::{BoxError, ServiceBuilder};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, get_service},
    AddExtensionLayer, Router,
};

use clap::Parser;
use sea_orm::DatabaseConnection;

use crate::handler::docs;

static LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
}

async fn handle_io_error(error: std::io::Error) -> Result<impl IntoResponse, Infallible> {
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled error: {}", error),
    ))
}

pub async fn static_routes() {
    let frontend = Router::new()
        .route(
            "/",
            get(|| async move { Redirect::to("/app".parse().unwrap()) }),
        )
        .nest(
            "/assets",
            get_service(ServeDir::new("assets")).handle_error(handle_io_error),
        )
        .route(
            "/app/*path",
            get_service(ServeFile::new("assets/index.html")).handle_error(handle_io_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    serve(frontend, 3000).await
}

pub async fn api_routes(conn: DatabaseConnection) {
    let backend = Router::new()
        .nest("/api", docs::router())
        .layer(AddExtensionLayer::new(conn))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    serve(backend, 4000).await
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((LOCALHOST, port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
