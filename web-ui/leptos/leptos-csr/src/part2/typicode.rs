use leptos::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[derive(Deserialize, Clone, Debug)]
struct Todo {
    userId:    u32,
    id:        u32,
    title:     String,
    completed: bool,
}

#[component]
pub fn Todos() -> impl IntoView {
    let (api_data, set_api_data) = create_signal(None::<String>);
    let fetch_api = move |_: ev::Event| {
        let set_api_data = set_api_data.clone();
        spawn_local(async move {
            if let Some(window) = window() {
                let resp = window.fetch_with_str("https://jsonplaceholder.typicode.com/todos/1")
                                 .await?
                                 .unwrap();
                let json = resp.json::<Todo>()
                               .await
                               .unwrap();
                set_api_data(Some(json.title));
            }
        });
    };

    view! {
        <div>
            <button on:click=move || fetch_api>"Fetch Data"</button>
            <p>{move || api_data.get().unwrap_or_else(|| "Loading...".to_string())}</p>
        </div>
    }
}
