// Declare all modules
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

// Import what we need
use dioxus::prelude::*;
use crate::application::use_cases::{Route, FAVICON, MAIN_CSS, TAILWIND_CSS};

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}