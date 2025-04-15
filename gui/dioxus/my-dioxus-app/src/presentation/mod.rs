// Export modules
pub mod components;
pub mod pages;

use crate::presentation::pages::{Documents, Home, Navbar, Posts, Todos, Users};
use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/users")]
    Users {},
    #[route("/todos")]
    Todos {},
    #[route("/posts")]
    Posts {},
    #[route("/documents")]
    Documents {},
}
