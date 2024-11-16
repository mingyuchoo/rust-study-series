use leptos::*;

#[component]
pub fn GoodOptions() -> impl IntoView {
    view! {
        <main>
            <h1>"Good Options"</h1>
            <BIsAFunctionOfA/>
            <CIsAFunctionOfAAndSomeOtherThingB/>
            <AAndBAreIndependentSignalsButSometimesUpdatedAtTheSameTime/>
        </main>
    }
}

// B is a function of A
#[component]
fn BIsAFunctionOfA() -> impl IntoView {
    // A
    let (count, set_count) = create_signal(1);
    // B is a function of A
    let derived_signal_double_count = move || {
        logging::log!("derived_signal_double_count: {}", count());
        count() * 2
    };
    // B is a function of A
    let memoized_double_count = create_memo(move |_| {
        logging::log!("memoized_double_count: {}", count());
        count() * 2
    });

    view! {
        <div>
            <h2>"B is a function of A"</h2>
            <p>"count: " {count}</p>
            <p>"derived_signal_double_count: " {derived_signal_double_count}</p>
            <p>"memoized_double_count: " {memoized_double_count}</p>
            <button on:click=move |_ev| { set_count.update(|n| *n += 1) }>"Increase"</button>
        </div>

    }
}

// C is a function of A and some other thing B
#[component]
fn CIsAFunctionOfAAndSomeOtherThingB() -> impl IntoView {
    // A
    let (first_name, set_first_name) = create_signal("Bridget".to_string());
    // B
    let (last_name, set_last_name) = create_signal("Jones".to_string());
    // C is a function of A and B
    let full_name = move || with!(|first_name, last_name| format!("{first_name} {last_name}"));
    view! {
        <div>
            <h2>"C is as function of A and some other thing B"</h2>
            <p>"First name: " {first_name}</p>
            <p>"Last name: " {last_name}</p>
            <p>"Full name: " {full_name}</p>
            <button on:click=move |_ev| { set_first_name.set("John".to_string()) }>"Change First Name"</button>
            <button on:click=move |_ev| { set_last_name.set("Doe".to_string()) }>"Change Last Name"</button>
        </div>
    }
}

// A and B are independent signals, but sometimes updated at the same time.
#[component]
fn AAndBAreIndependentSignalsButSometimesUpdatedAtTheSameTime() -> impl IntoView {
    // A
    let (age, set_age) = create_signal(32);
    // B
    let (favorite_number, set_favorite_number) = create_signal(42);
    // use this to handle a click on a `Clear` button
    let clear_handle = move |_| {
        // update both A and B
        set_age(0);
        set_favorite_number(0);
    };
    view! {
        <div>
            <h2>"A and B are independent signals, but sometimes updated at the same time"</h2>
            <p>{age}</p>
            <p>{favorite_number}</p>
            <button on:click=clear_handle>
                "Clear"
            </button>
        </div>
    }
}
