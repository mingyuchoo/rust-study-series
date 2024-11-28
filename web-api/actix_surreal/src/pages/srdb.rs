use leptos::*;
use leptos_router::*;

#[component]
pub fn SrdbPage() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <h1 class="text-2xl font-bold mb-4">"SurrealDB API"</h1>
            <Outlet/>
        </div>
    }
}
