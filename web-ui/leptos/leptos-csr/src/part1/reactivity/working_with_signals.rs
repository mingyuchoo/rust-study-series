use leptos::*;

#[component]
pub fn GettingAndSetting() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    set_count(1);
    set_count.set(2);
    set_count.update(|n| *n = 3);

    logging::log!("{}", count());

    let (first, _) = create_signal("Bob".to_string());
    let (middle, _) = create_signal("J.".to_string());
    let (last, _) = create_signal("Smith".to_string());

    let name1 = move || first.with(|first| middle.with(|middle| last.with(|last| format!("{first} {middle} {last}"))));
    let name2 = move || with!(|first, middle, last| format!("{first} {middle} {last}"));

    view! {
        <div>
            <h1>"Getting and Setting"</h1>
            <p>"Count: " {count}</p>
            <p>"Count: " {count.get()}</p>
            <p>"Count: " {count.with(|n| n.clone())}</p>
            <p>{name1}</p>
            <p>{name2}</p>
        </div>
    }
}
