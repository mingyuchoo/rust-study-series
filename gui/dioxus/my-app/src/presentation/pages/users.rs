use dioxus::prelude::*;
use crate::presentation::components::UsersTab;

/// Users page
#[component]
pub fn Users() -> Element {
    rsx! {
        UsersTab {}
    }
}
