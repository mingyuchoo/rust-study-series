use dioxus::prelude::*;
use crate::presentation::components::PostsTab;

/// Posts page
#[component]
pub fn Posts() -> Element {
    rsx! {
        PostsTab {}
    }
}
