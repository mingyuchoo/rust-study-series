use gloo_timers::future::TimeoutFuture;
use leptos::*;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn CreateResource() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(// the first is the "source signal"
                                     count,
                                     // the second is the loader
                                     // it takes the source signal's value as its argument
                                     // and does some async work
                                     |value| async move { load_data(value).await });

    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the soruce signal
    // that doesn't depend on anything: we jsut load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .get()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data.get().map(|value| format!("Server returned {value:?}"))
        // This loading state will only show before the first load
        .unwrap_or_else(|| "Loading...".into())
    };

    view! {
        <main>
            <h1>"Create resource"</h1>
            { move || match once.get() {
                | None => view! { <p>"Loading..."</p>}.into_view(),
                | Some(data) => view! { <ShowData data/>}.into_view(),
            }}
        </main>
    }
}
