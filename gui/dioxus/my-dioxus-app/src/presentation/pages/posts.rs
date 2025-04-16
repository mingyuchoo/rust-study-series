use crate::presentation::components::PostsTab;
use dioxus::prelude::*;

/// Posts page
#[component]
pub fn Posts() -> Element {
    rsx! {


        PostsTab {}
    }
}
