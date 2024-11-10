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
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <h1>"Welcome to Leptos!"</h1>
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button
                        on:click=move |_| set_count.update(|count| *count += 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-1-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {count}
                    </button>
                    <button
                        on:click=move |_| set_count.update(|value| *value -= 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg lb-blue-700 border-blue-800 text-white"
                        class:invisible=move || { count.get() < 1}
                    >
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
