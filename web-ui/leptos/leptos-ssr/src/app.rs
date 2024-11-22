use crate::client::pages::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-ssr.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <nav>
                <ul>
                    <li><A href="">"Home"</A></li>
                    <li><A href="/todos">"Todos"</A></li>
                </ul>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=home::HomePage/>
                    <Route path="/todos" view=todos::TodosPage/>
                    <Route path="/*any" view=not_found::NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
