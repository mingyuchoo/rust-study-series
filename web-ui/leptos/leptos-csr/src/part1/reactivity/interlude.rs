use leptos::*;

#[component]
pub fn ReactivityAndFunctions() -> impl IntoView {
    // a signal holds a value, and can be updated
    let (count, set_count) = create_signal(0);

    // a derived signal is a function that accesses other signals
    let double_count = move || count() * 2;
    let count_is_odd = move || count() & 1 == 1;
    let text = move || if count_is_odd() { "odd" } else { "even" };

    // an effect automatically tracks the signals it depends on
    // and reruns when they change
    create_effect(move |_| {
        logging::log!("text = {}", text());
    });

    view! {
        <main>
            <h2>"Reactivity and Functions"</h2>
            <p>{move || text().to_uppercase()}</p>
            <SimpleCount/>
        </main>
    }
}

#[component]
pub fn SimpleCount() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        <button on:click=increment>
            {value}
        </button>
    }
}
