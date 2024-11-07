use leptos::*;

#[component]
pub fn DynamicAttributes() -> impl IntoView
{
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let double_count = move || count() * 2;
    let html = "<p>This HTML will be injected.<p>";

    // 1. create event listeners
    // 2. create dynamic text by passing a signal into the view
    view! {
        <h1>Dynamic Attributes</h1>
        <button
            // Dynamic Classes
            on:click=move |_| set_count.update(|n| *n += 1)
            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>

        <button
            // Dyanmic Styles
            on:click=move |_| set_x.update(|n| *n += 10)
            style="position: absolute"
            style:left=move || format!("{}px", x() + 10)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            style=("--coloumns", x)
        >
            "Click to move"
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

        <p>
            // Derived signals
            <progress
                max="50"
                // signals are functions,
                // so    `value=move || count.get()`
                // ,     `value=move || count()`
                // , and `value=count` are interchaneable.
                value=double_count
            />
            "Double Count: " {double_count}
        </p>
        <div inner_html=html />

    }
}
