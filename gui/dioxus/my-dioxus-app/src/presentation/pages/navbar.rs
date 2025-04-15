use crate::presentation::Route;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            div {
                div {
                    span { "JSONPlaceholder API Manager" }
                }

                div {
                    div {
                        Link { to: Route::Home {}, "Home" }
                        Link { to: Route::Users {}, "Users" }
                        Link { to: Route::Todos {}, "Todos" }
                        Link { to: Route::Posts {}, "Posts" }
                        Link { to: Route::Documents {}, "Documents" }
                    }
                }
            }
        }

        div { class: "container mx-auto",
            Outlet::<Route> {}
        }
    }
}
