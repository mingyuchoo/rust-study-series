use leptos::*;

pub fn SoWhat() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // T by if statements
    let is_odd = move || value() & 1 == 1;

    // T by match statements
    let message1 = move || match value() {
        | 0 => "Zero",
        | 1 => "One",
        | n if is_odd() => "Odd",
        | _ => "Even",
    };

    // Option<T>
    let message2 = move || {
        if is_odd() {
            Some("Ding ding ding")
        } else {
            None
        }
    };

    // Option<T>
    let message3 = move || is_odd().then(|| "Ding ding ding!");

    view! {
        <h1>
            "Control Flow"
        </h1>
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                    "Even"
            }}
        </p>
        <p>
            {message1}
        </p>
        <p>
            {message2}
        </p>
        <p>
            {message3}
        </p>
    }
}

pub fn NotPreventingOverRendering() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let message = move || {
        if value() > 5 {
            logging::log!("{}: rendering Big", value());
            "Big"
        } else {
            logging::log!("{}: rendering Small", value());
            "Small"
        }
    };

    view! {
        <button
            on:click=move |_| {
                set_value.update(|n| *n += 1);
            }
        >
            "Click me: " { move || value()}
        </button>
        <p>
            {message}
        </p>
    }
}

pub fn PreventingOverRendering() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                set_value.update(|n| *n += 1);
            }
        >
            "Click me: " { move || value()}
        </button>
        <Show
            when=move || { value() > 5 }
            fallback=|| view! {
                <p>
                    "Small"
                </p>
            }
        >
            <p>
                "Big"
            </p>
        </Show>
    }
}

pub fn TypeConversions() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    view! {
        <main>
            {move || match is_odd() {
                | true  if value() == 1 => { view! {<pre>"One"</pre>}.into_any()},
                | false if value() == 2 => { view! {<p>"Two"</p>}.into_any()},
                | _                     => { view! {<textarea>{value()}</textarea>}.into_any()}
            }}
        </main>
    }
}
