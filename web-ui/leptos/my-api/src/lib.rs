use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::api::Api;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>
        <Title text="Welcome to Leptos CSR"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/api" view=Api/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}
