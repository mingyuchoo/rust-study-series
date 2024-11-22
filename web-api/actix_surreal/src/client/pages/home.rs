use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <h1 class="text-2xl font-bold mb-4">"Welcome to Leptos!"</h1>
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button
                        on:click=move |_| set_count.update(|count| *count += 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-1-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {count}
                    </button>
                    <button
                        on:click=move |_| set_count.update(|value| *value -= 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg lb-blue-700 border-blue-800 text-white"
                        class:invisible=move || { count.get() < 1}
                    >
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}
