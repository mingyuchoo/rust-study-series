use leptos::*;
use gloo_timers::future::TimeoutFuture;

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn Api() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let async_data = create_resource(
        count,
        |value| async move { load_data(value).await },
    );
    let stable = create_resource(|| (), |_| async move { load_data(1).await });
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading...".into())
    };
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}
