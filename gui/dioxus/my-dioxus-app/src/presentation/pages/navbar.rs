use crate::application::use_cases::Route;
use dioxus::prelude::*;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "bg-gray-800 text-white p-4",
            div { class: "container mx-auto flex flex-wrap items-center justify-between",
                div { class: "flex items-center flex-shrink-0 mr-6",
                    span { class: "font-semibold text-xl tracking-tight", "JSONPlaceholder API Manager" }
                }

                div { class: "w-full block flex-grow lg:flex lg:items-center lg:w-auto",
                    div { class: "text-sm lg:flex-grow",
                        Link { class: "block mt-4 lg:inline-block lg:mt-0 text-gray-200 hover:text-white mr-4", to: Route::Home {}, "Home" }
                        Link { class: "block mt-4 lg:inline-block lg:mt-0 text-gray-200 hover:text-white mr-4", to: Route::Users {}, "Users" }
                        Link { class: "block mt-4 lg:inline-block lg:mt-0 text-gray-200 hover:text-white mr-4", to: Route::Todos {}, "Todos" }
                        Link { class: "block mt-4 lg:inline-block lg:mt-0 text-gray-200 hover:text-white", to: Route::Posts {}, "Posts" }
                    }
                }
            }
        }

        div { class: "container mx-auto",
            Outlet::<Route> {}
        }
    }
}
