use leptos::*;
use leptos_router::*;

#[component]
pub fn SrdbRoutesPage() -> impl IntoView {
    view! {
        <div class="container mx-auto p-4">
            <h2 class="text-xl font-bold mb-4">"SurrealDB Routes"</h2>
            <ul class="space-y-2">
                <li>
                    <A href="/srdb/people" class="text-yellow-300 hover:underline">
                        "View All People"
                    </A>
                </li>
            </ul>
        </div>
    }
}
