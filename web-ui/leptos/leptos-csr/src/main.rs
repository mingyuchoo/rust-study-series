use leptos::*;

fn main()
{
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView
{
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        <p>
            <strong>"Reactive: " </strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive: " </strong>
            {move || count()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            {count()}
        </p>
    }
}
