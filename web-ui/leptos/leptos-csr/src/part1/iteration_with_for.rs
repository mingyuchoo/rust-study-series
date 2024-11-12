use leptos::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key:   String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    let (data, set_data) =
        create_signal(vec![DatabaseEntry { key:   "foo".to_string(),
                                           value: 10, },
                           DatabaseEntry { key:   "bar".to_string(),
                                           value: 20, },
                           DatabaseEntry { key:   "baz".to_string(),
                                           value: 15, },]);
    view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}
