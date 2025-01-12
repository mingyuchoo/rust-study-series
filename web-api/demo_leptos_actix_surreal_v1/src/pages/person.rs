use leptos::*;
use leptos_router::*;

#[component]
pub fn PersonPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params.with(|params| params.get("id").cloned().unwrap_or_default())
    };

    view! {
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <h2 class="text-xl font-bold mb-4">"Person Details"</h2>
            <p>"ID: " {id}</p>
        </div>
    }
}
