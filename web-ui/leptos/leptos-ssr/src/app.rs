// use crate::models::todo::Todo;
use crate::server::todo::fetch_todos;
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
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/todo" view=TodosPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn TodosPage() -> impl IntoView {
    let todos = create_resource(|| (), |_| fetch_todos());

    view! {
        <div>
            <h1>"Todo List"</h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    todos.get().map(|todos| match todos {
                        Ok(todos) => {
                            view! {
                                <ul>
                                    {todos.into_iter().map(|todo| {
                                        view! {
                                            <li>
                                                <input type="checkbox" checked=todo.completed/>
                                                {todo.title}
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }.into_view()
                        }
                        Err(e) => view! { <p>"Error loading todos: " {e.to_string()}</p> }.into_view()
                    })
                }}
            </Suspense>
        </div>
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
