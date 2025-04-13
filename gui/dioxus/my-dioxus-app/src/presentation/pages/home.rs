use crate::application::services::Route;
use dioxus::prelude::*;

/// Home page
#[component]
pub fn Home() -> Element {
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
