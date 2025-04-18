use crate::presentation::Route;
use dioxus::prelude::*;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            div {
                h1 { "JSONPlaceholder API Manager" }
            }
            div {
                div {
                    Link { to: Route::Home {}, "Home" }
                    Link { to: Route::Users {}, "Users" }
                    Link { to: Route::Todos {}, "Todos" }
                    Link { to: Route::Posts {}, "Posts" }
                    Link { to: Route::Docs {}, "Documents" }
                }
            }
        }
        div {
            Outlet::<Route> {}
        }
    }
}
