#![recursion_limit = "512"]

mod components;

use components::composer::Composer;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    let element = yew::utils::document()
        .query_selector("#app-container")
        .unwrap()
        .expect("Cannot find app-container element");

    yew::App::<Composer>::new().mount(element);
    yew::run_loop();

    Ok(())
}
