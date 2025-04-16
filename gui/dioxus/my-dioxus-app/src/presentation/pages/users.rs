use crate::presentation::components::UsersTab;
use dioxus::prelude::*;

/// Users page
#[component]
pub fn Users() -> Element {
    rsx! {
        UsersTab {}
    }
}
