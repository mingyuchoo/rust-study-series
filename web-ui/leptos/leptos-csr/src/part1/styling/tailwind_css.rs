use leptos::*;

#[component]
pub fn TailwindCSS() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                {move || match count() == 0 {
                     | true => "Click me!".to_string(),
                     | false => count().to_string(),
                }}
            </button>
        </main>
    }
}
