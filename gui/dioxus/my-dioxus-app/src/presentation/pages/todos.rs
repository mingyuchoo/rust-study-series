use crate::presentation::components::TodosTab;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Todos page
#[component]
pub fn Todos() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        TodosTab {}
    }
}
