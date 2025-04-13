// Only import what we need

mod application;
mod domain;
mod infrastructure;
mod presentation;

// Import App component from application services
use application::services::App;

fn main() {
    // Launch the application
    dioxus::launch(App);
}
