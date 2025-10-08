// Import what we need
use dioxus::prelude::*;
use dioxus_app::presentation::{FAVICON, MAIN_CSS, Route};

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

fn main() {
    // Launch the application
    dioxus::launch(App);
}
