use dioxus::prelude::*;

mod domain;
mod infrastructure;
mod presentation;

// Import page components from the pages module
use presentation::pages::{Home, Users, Todos, Posts, Navbar};

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
