use dioxus::prelude::*;
use crate::presentation::components::TodosTab;

/// Todos page
#[component]
pub fn Todos() -> Element {
    rsx! {
        TodosTab {}
    }
}
