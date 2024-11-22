use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="home-container">
            <h1 class="home-title">"Welcome to Leptos!"</h1>
            <button class="counter-button" on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
