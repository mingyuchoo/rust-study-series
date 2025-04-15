use crate::presentation::components::UsersTab;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Users page
#[component]
pub fn Users() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        UsersTab {}
    }
}
