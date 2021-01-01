#![recursion_limit = "512"]

mod components;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let element = yew::utils::document()
        .query_selector("#app-container")
        .unwrap()
        .expect("Cannot find app-container element");

    yew::App::<components::Composer>::new().mount(element);
    yew::run_loop();

    Ok(())
}
