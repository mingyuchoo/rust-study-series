use leptos::prelude::*;
mod app;
mod models;
mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <app::App/> })
}