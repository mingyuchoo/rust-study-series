use crate::presentation::components::PostsTab;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Posts page
#[component]
pub fn Posts() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        PostsTab {}
    }
}
