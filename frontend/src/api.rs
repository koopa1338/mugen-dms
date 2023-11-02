use gloo_net::http::{Method, RequestBuilder};
use leptos::Serializable;
use serde::Serialize;

pub async fn api_call<T, P>(
    path: &str,
    method: Method,
    payload: Option<P>,
    query_params: Option<Vec<(&str, &str)>>,
) -> Option<T>
where
    T: Serializable,
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

    let mut builder = RequestBuilder::new(path)
        .method(method)
        .abort_signal(abort_signal.as_ref());
    if let Some(params) = query_params {
        builder = builder.query(params);
    }

    let req = if let Some(payload) = payload {
        builder
            .json(&payload)
            .map_err(|e| log::error!("{e}"))
            .ok()?
    } else {
        builder.build().map_err(|e| log::error!("{e}")).ok()?
    };

    let json = req
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::de(&json).ok()
}
