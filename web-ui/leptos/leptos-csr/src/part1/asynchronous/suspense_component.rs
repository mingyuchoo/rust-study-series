use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name.to_ascii_uppercase()
}

#[component]
pub fn SuspenseComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Bill".to_string());

    // this will reload every time `name` changes
    let async_data =
        create_resource(name,
                        |name| async move { important_api_call(name).await });

    view! {
        <main>
            <h1>"Suspense Component"</h1>
            <input on:input=move |ev| set_name(event_target_value(&ev)) prop:value=name/>
            <p><code>"name: "</code>{name}</p>
            <Suspense
                // the fallback will show whenever a response
                // read "under" the suspense is loading
                fallback=move || view! { <p>"Loading..."</p>}
            >
            // the children will be rendered once initially,
            // and then whenever any resource has been resolved
            <p>
                "Your shouting name is " {move || async_data.get()}
            </p>
            </Suspense>
        </main>
    }
}
