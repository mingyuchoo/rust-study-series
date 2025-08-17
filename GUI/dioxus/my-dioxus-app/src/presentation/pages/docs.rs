use crate::presentation::components::DocsTab;
use dioxus::prelude::*;

/// Docs page
#[component]
pub fn Docs() -> Element {
    rsx! {
        DocsTab {}
    }
}
