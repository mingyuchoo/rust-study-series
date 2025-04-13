use dioxus::prelude::*;
mod api;
mod components;
mod models;

use components::{PostsTab, TodosTab, UsersTab};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/users")]
    Users {},
    #[route("/todos")]
    Todos {},
    #[route("/posts")]
    Posts {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6", "JSONPlaceholder API Manager" }
            p { class: "mb-4", "Welcome to the JSONPlaceholder API Manager. This application allows you to manage users, todos, and posts using the JSONPlaceholder API." }
            p { class: "mb-4", "Use the navigation tabs above to access different sections of the application." }

            div { class: "mt-8 grid grid-cols-1 md:grid-cols-3 gap-6",
                div { class: "border rounded-lg p-6 shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Users" }
                    p { "Manage user accounts with CRUD operations." }
                    Link { class: "mt-4 inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded", to: Route::Users {}, "Go to Users" }
                }

                div { class: "border rounded-lg p-6 shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Todos" }
                    p { "Create, read, update, and delete todo items." }
                    Link { class: "mt-4 inline-block bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded", to: Route::Todos {}, "Go to Todos" }
                }

                div { class: "border rounded-lg p-6 shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Posts" }
                    p { "Manage blog posts with full CRUD functionality." }
                    Link { class: "mt-4 inline-block bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded", to: Route::Posts {}, "Go to Posts" }
                }
            }
        }
    }
}

/// Users page
#[component]
fn Users() -> Element {
    rsx! {
        UsersTab {}
    }
}

/// Todos page
#[component]
fn Todos() -> Element {
    rsx! {
        TodosTab {}
    }
}

/// Posts page
#[component]
fn Posts() -> Element {
    rsx! {
        PostsTab {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
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
