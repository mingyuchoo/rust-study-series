use leptos::*;

#[component]
pub fn Basic() -> impl IntoView {
    let (a, set_a) = create_signal(0);
    let (b, set_b) = create_signal(0);

    create_effect(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        log::debug!("Value: {}", a());
    });

    view! {
        <main>
            <h1>"create_effect"</h1>
            <p>"a: " {a}</p>
            <p>"b: " {b}</p>
        </main>
    }
}
#[component]
pub fn ZeroCostishAbstraction() -> impl IntoView {
    let (first, set_first) = create_signal(String::new());
    let (last, set_last) = create_signal(String::new());
    let (use_last, set_use_last) = create_signal(true);

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(move |_| {
        if use_last() {
            log::debug!("{} {}", first(), last())
        }
        else {
            log::debug!("{}", first())
        }
    });

    view! {
        <main>
            <h2>"Effects as Zero-Cost-ish Abstraction"</h2>
            <p>"First name: " {first}</p>
            <p>"Last name: " {last}</p>
            <input type="text" on:input=move |ev| set_first(event_target_value(&ev)) prop:value=first/>
            <input type="text" on:input=move |ev| set_last(event_target_value(&ev))  prop:value=last/>
            <button on:click=move |_ev| set_use_last.update(|value| *value = !*value)>
                "Toggle"
            </button>
        </main>
    }
}

#[component]
pub fn CancelableTrackingWithWatch() -> impl IntoView {
    let (num, set_num) = create_signal(0);
    let stop = watch(move || num.get(),
                     move |num, prev_num, _| {
                         log::debug!("Number: {}; Prev: {:?}", num, prev_num);
                     },
                     false);

    view! {
        <main>
            <h2>"Explicit, Cancelable Tracking with watch"</h2>
            <p>"Number: " {num.get()}</p>
            <button
                on:click=move |_ev| {
                    // > "Number:1; Prev: Some(0)"
                    set_num.set(1)
                }
            >
                "Set to 1"
            </button>
            <button
                on:click=move |_ev| {
                    // stop watching
                    stop()
                }
            >
                "Stop watching"
            </button>
            <button
                on:click=move |_ev| {
                    // (nothing happens)
                    set_num.set(2)
                }
            >
                "Set to 2"
            </button>
        </main>
    }
}
