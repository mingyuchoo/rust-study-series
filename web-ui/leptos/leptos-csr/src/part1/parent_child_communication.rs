use leptos::{ev::MouseEvent,
             *};

// Pass a WriteSignal
#[component]
pub fn PassAWriteSignal() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <div>
            <h1>
                "Parent-Child Communication"
            </h1>
            <p>
                "Toggled? " {toggled}
            </p>
            <ButtonA setter=set_toggled/>
        </div>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_ev| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}

// User a Callback
#[component]
pub fn UseACallback() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <div>
            <p>
                "Toggled? " {toggled}
            </p>
            <ButtonB1 on_click=move |_ev| set_toggled.update(|value| *value = !*value)/>
            <ButtonB2 on_click=move |_ev| set_toggled.update(|value| *value = !*value)/>
        </div>
    }
}

// Callback Version
#[component]
pub fn ButtonB1(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

// Closure version
#[component]
pub fn ButtonB2<F>(on_click: F) -> impl IntoView
    where F: Fn(MouseEvent) + 'static,
{
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

// User an EventListener
#[component]
pub fn UseAnEventListener() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <div>
            <p>"Toggled? " {toggled}</p>
            // note the on:click instead of on_click
            // this is the same syntax as an HTML element event listener
            <ButtonC on:click=move |_ev| set_toggled.update(|value| *value = !*value)/>
        </div>
    }
}

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle"
        </button>
    }
}

// Providing a Context
#[component]
pub fn ProvidingAContext() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    // share `set_toggled` with all children of this component
    provide_context(set_toggled);
    view! {
        <main>
            <h1>
                "Provding a Context"
            </h1>
            <p>"Toggled? " {toggled}</p>
            <Layout/>
        </main>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h2>
                "My Page"
            </h2>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    // use_context searches up the context tree,
    // hoping to find a `WriteSignal<bool>`
    // in the case, I .expect() because I know I provided it
    let setter = use_context::<WriteSignal<bool>>().expect("to have found \
                                                            the setter \
                                                            provided.");
    view! {
        <button
            on:click=move |_ev| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
