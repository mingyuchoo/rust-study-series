pub mod client;

use crate::client::app::App;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use client::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
