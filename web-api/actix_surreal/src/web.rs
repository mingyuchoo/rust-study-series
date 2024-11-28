use crate::pages::*;
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
                    <Route path="" view=home::HomePage/>
                    <Route path="/srdb" view=srdb::SrdbPage>
                        <Route path="/" view=srdb_routes::SrdbRoutesPage/>
                        <Route path="person/:id" view=person::PersonPage/>
                        <Route path="people" view=people::PeoplePage/>
                    </Route>
                    <Route path="/*any" view=not_found::NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
