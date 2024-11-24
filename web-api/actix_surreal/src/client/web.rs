use crate::client::pages::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/actix_surreal.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
        <nav class="bg-gray-800 p-4">
            <div class="container mx-auto flex gap-6">
                <A href="" class="text-white hover:text-gray-300 transition-colors" >"Home"</A>
                <A href="/srdb" class="text-white hover:text-gray-300 transition-colors" >"SurrealDB"</A>
            </div>
        </nav>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/srdb" view=SrdbPage>
                        <Route path="/" view=SrdbRoutesPage/>
                        <Route path="person/:id" view=PersonPage/>
                        <Route path="people" view=PeoplePage/>
                    </Route>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
