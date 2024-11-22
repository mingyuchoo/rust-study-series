use leptos::*;
use leptos_router::*;

#[component]
pub fn SrdbIndexPage() -> impl IntoView {
    view! {
        <div class="container mx-auto p-4">
            <h2 class="text-xl font-bold mb-4">"SurrealDB Routes"</h2>
            <ul class="space-y-2">
                <li>
                    <A href="/srdb/people" class="text-blue-600 hover:underline">
                        "View All People"
                    </A>
                </li>
                // 필요한 경우 더 많은 링크를 여기에 추가할 수 있습니다
            </ul>
        </div>
    }
}
