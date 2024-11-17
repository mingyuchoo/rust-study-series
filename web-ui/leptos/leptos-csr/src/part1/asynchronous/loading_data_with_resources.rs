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
    // pub fn create_signal<T>(
    //     value: T
    // ) -> (ReadSignal<T>, WriteSignal<T>)

    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // pub fn create_resource<S, T, Fu>(
    //     source: impl Fn() -> S + 'static,
    //     fetcher: impl Fn(S) -> Fu + 'static,
    // ) -> Resource<S, T>
    // where
    //     S: PartialEq + Clone + 'static,
    //     T: Serializable + 'static,
    //     Fu: Future<Output = T> +'static,

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // create_resource takes two arguments after its scope
    let async_data = create_resource(// the first is the "source signal"
                                     count,
                                     // the second is the "fetcher function"
                                     // it takes the source signal's value as its argument
                                     // and does some async work
                                     |value| async move { load_data(value).await });
    // whenever the "source signal" changes, the "fetcher function" reloads

    // we can access the resource value with .get()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            // get asynchronous resource by get()
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a signal
    // to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || match loading() {
        | true => "Loading",
        | false => "Idle.",
    };

    view! {
        <main>
            <h1>"Create Resource"</h1>
            <button on:click=move |_ev| set_count.update(|n| *n += 1)>"Click me"</button>
            <p><code>"stable"</code>": " {move || stable.get()}</p>
            <p><code>"count"</code>": " {count}</p>
            <p><code>"async_value"</code>": " {async_result}<br/>{is_loading}</p>
        </main>
    }
}
