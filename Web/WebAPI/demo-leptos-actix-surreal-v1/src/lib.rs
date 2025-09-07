pub mod config;
pub mod db;
pub mod web;

mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(web::App);
}
