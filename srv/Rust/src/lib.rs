#![recursion_limit = "256"]

include!(concat!(env!("OUT_DIR"), "/dynamic_routes.rs"));

pub mod app;
pub mod components;
pub use app::App;

#[cfg(feature = "hydrate")]
use {leptos::mount, wasm_bindgen::prelude::wasm_bindgen};

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount::hydrate_body(App);
}
