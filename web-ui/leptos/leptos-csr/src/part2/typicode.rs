use gloo_net::http::Request;
use leptos::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Todo {
    userId:    u32,
    id:        u32,
    title:     String,
    completed: bool,
}

#[component]
pub fn Api() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(false);
    let fetch_todos = create_action(move |_: &()| async move {
        set_loading.set(true);
        let fetched_todos: Vec<Todo> =
            Request::get("https://jsonplaceholder.typicode.com/todos").send()
                                                                      .await
                                                                      .unwrap()
                                                                      .json()
                                                                      .await
                                                                      .unwrap();
        set_loading.set(false);
        fetched_todos
    });

    create_effect(move |_| {
        if let Some(fetched_todos) = fetch_todos.value()
                                                .get()
        {
            set_todos.set(fetched_todos);
        }
    });

    view! {
        <main>
            <h1>
                "Typicode API"
            </h1>
            <button on:click=move |_| fetch_todos.dispatch(())>
                "Fetch Todos"
            </button>
            {move ||
                if loading() {
                    view! { <p>"Loading..."</p>}.into_any()
                } else {
                    view! {
                        <ul>
                            {move || todos().into_iter().map(|todo| view! {
                                <li>
                                    {format!("User Id: {}, Id: {}, Title: {}, Completed: {}", todo.userId, todo.id, todo.title, todo.completed)}
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_any()
                }
            }
        </main>
    }
}
