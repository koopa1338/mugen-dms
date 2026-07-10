use gloo_net::http::{Method, RequestBuilder};
use gloo_utils::errors::JsError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Make an HTTP request to the backend API.
///
/// # Errors
///
/// Returns [`FetchError`] if the request fails to build, send, or deserialize the response.
pub async fn api_call<T, P>(
    path: &str,
    method: Method,
    payload: Option<P>,
    query_params: Option<Vec<(&str, &str)>>,
) -> Result<T, FetchError>
where
    T: for<'de> Deserialize<'de> + Serialize,
    P: Serialize,
{
    // Change this for production deployments
    const BASE_URL: &str = "http://localhost:4000";
    let mut builder = RequestBuilder::new(&format!("{BASE_URL}/{path}"))
        .method(method);
    if let Some(params) = query_params {
        builder = builder.query(params);
    }

    let req = if let Some(payload) = payload {
        builder
            .json(&payload)
            .map_err(|e| FetchError::PayloadError(e.to_string()))?
    } else {
        builder
            .build()
            .map_err(|e| FetchError::ClientError(e.to_string()))?
    };

    let json = req
        .send()
        .await
        .map_err(|e| {
            let error_message = match e {
                gloo_net::Error::JsError(JsError { message, .. }) => message,
                gloo_net::Error::SerdeError(e) => e.to_string(),
                gloo_net::Error::GlooError(msg) => msg,
            };
            FetchError::GenericError(error_message)
        })?
        .text()
        .await
        .map_err(|e| FetchError::GenericError(e.to_string()))?;

    serde_json::from_str(&json).map_err(|e| FetchError::GenericError(e.to_string()))
}

#[derive(Error, Clone, Debug, Deserialize, Serialize)]
pub enum FetchError {
    #[error("Generic Error: {0}")]
    GenericError(String),
    #[error("Error initializing request client: {0}")]
    ClientError(String),
    #[error("Error setting payload: {0}")]
    PayloadError(String),
}
