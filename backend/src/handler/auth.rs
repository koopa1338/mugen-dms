use axum::{routing::post, Json, Router};
use serde::de::DeserializeOwned;
use serde_json::json;
use tracing_attributes::instrument;

use crate::{
    auth::AuthProvider,
    config::app::AppState,
    error::ApiResult,
    storage::StorageBackend,
};

pub fn router<S, A>() -> Router<AppState<S, A>>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
    A::Credentials: DeserializeOwned + Send + 'static,
    A::RegistrationData: DeserializeOwned + Send + 'static,
{
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(register::<S, A>))
            .route("/login", post(login::<S, A>)),
    )
}

#[instrument(skip_all)]
async fn register<S, A>(
    axum::extract::State(state): axum::extract::State<AppState<S, A>>,
    Json(data): Json<A::RegistrationData>,
) -> ApiResult<impl axum::response::IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
    A::RegistrationData: DeserializeOwned + Send + 'static,
{
    let user = state.auth.register(data).await?;
    tracing::debug!("Registered user: {}", user.username);
    Ok(Json(user))
}

#[instrument(skip_all)]
async fn login<S, A>(
    axum::extract::State(state): axum::extract::State<AppState<S, A>>,
    Json(credentials): Json<A::Credentials>,
) -> ApiResult<impl axum::response::IntoResponse>
where
    S: StorageBackend + Clone + Send + Sync + 'static,
    A: AuthProvider + Clone + Send + Sync + 'static,
    A::Credentials: DeserializeOwned + Send + 'static,
{
    let token = state.auth.authenticate(credentials, &state.jwt_secret).await?;
    tracing::debug!("Login successful, token issued");
    Ok(Json(json!({ "token": token })))
}
