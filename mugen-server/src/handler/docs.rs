use axum::{response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/docs", get(docs_index).post(docs_create))
        .route(
            "/docs/:id",
            get(docs_by_id).patch(docs_update).delete(docs_delete),
        )
}

pub async fn docs_index() -> impl IntoResponse {
    unimplemented!();
}

pub async fn docs_by_id() -> impl IntoResponse {
    unimplemented!();
}

pub async fn docs_update() -> impl IntoResponse {
    unimplemented!();
}

pub async fn docs_delete() -> impl IntoResponse {
    unimplemented!();
}

pub async fn docs_create() -> impl IntoResponse {
    unimplemented!();
}
