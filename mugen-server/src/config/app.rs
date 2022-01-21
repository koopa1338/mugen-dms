use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;

use axum::response::Redirect;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};

use tower::{BoxError, ServiceBuilder};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::handler::docs;

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
            get(|| async move { Redirect::to("/app/main".parse().unwrap()) }),
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

pub async fn api_routes() {
    let backend = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/docs", get(docs::docs_index).post(docs::docs_create))
                .route(
                    "/docs/:id",
                    get(docs::docs_by_id)
                        .patch(docs::docs_update)
                        .delete(docs::docs_delete),
                ), // .layer(AddExtensionLayer::new(db));
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
    serve(backend, 4000).await
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
