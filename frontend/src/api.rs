use gloo_net::http::{Method, RequestBuilder};
use leptos::Serializable;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use gloo_utils::errors::JsError;

pub async fn api_call<T, P>(
    path: &str,
    method: Method,
    payload: Option<P>,
    query_params: Option<Vec<(&str, &str)>>,
) -> Result<T, FetchError>
where
    T: Serializable, // this means both serializable and deserializable
    P: Serialize,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|abort| abort.signal());

    // abort in-flight requests if, e.g., we've navigated away from this page
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let mut builder = RequestBuilder::new(&format!("http://localhost:4000/{path}"))
        .method(method)
        .abort_signal(abort_signal.as_ref());
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

    T::de(&json).map_err(|e| FetchError::GenericError(e.to_string()))
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
