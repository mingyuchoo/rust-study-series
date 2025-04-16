use crate::presentation::components::TodosTab;
use dioxus::prelude::*;

/// Todos page
#[component]
pub fn Todos() -> Element {
    rsx! {


        TodosTab {}
    }
}
