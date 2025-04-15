// Import what we need
use dioxus::prelude::*;
use my_dioxus_app::presentation::{FAVICON, MAIN_CSS, Route, TAILWIND_CSS};

// Explicitly enable native-db feature
#[cfg(not(feature = "native-db"))]
compile_error!("This application requires the native-db feature to be enabled. Please rebuild with --features native-db");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

fn main() {
    // Launch the application
    dioxus::launch(App);
}
