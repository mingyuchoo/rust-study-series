use crate::presentation::Route;
use dioxus::prelude::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            p { "Welcome to the JSONPlaceholder API Manager. This application allows you to manage users, todos, and posts using the JSONPlaceholder API." }
            p { "Use the navigation tabs above to access different sections of the application." }
            div {
                div {
                    h2 { "Users" }
                    p { "Manage user accounts with CRUD operations." }
                    Link { to: Route::Users {}, "Go to Users" }
                }
                div {
                    h2 { "Todos" }
                    p { "Create, read, update, and delete todo items." }
                    Link { to: Route::Todos {}, "Go to Todos" }
                }
                div {
                    h2 { "Posts" }
                    p { "Manage blog posts with full CRUD functionality." }
                    Link { to: Route::Posts {}, "Go to Posts" }
                }
                div {
                    h2 { "Documents" }
                    p { "View API documentation." }
                    Link { to: Route::Docs {}, "Go to Documents" }
                }
            }
        }
    }
}
