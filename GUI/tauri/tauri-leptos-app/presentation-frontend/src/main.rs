use leptos::prelude::*;
use presentation_frontend::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
