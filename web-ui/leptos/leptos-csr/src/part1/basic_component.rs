use leptos::*;

#[component]
pub fn BasicComponent() -> impl IntoView
{
    let (count, set_count) = create_signal(0);

    view! {
        <h1>Baisc Component</h1>
        <button
            on:click=move |_| set_count.update(|n| *n += 1)
        >
            "Click me"
        </button>
        <p>
            // old fashion
            <strong>"Reactive: " </strong>
            {move || count.get()}
        </p>
        <p>
            // modern fashion
            <strong>"Reactive: " </strong>
            {move || count()}
        </p>
        <p>
            // working correctly
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            // not working
            <strong>"Not reactive: "</strong>
            {count()}
        </p>
    }
}
