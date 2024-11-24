use leptos::*;

#[component]
pub fn PeoplePage() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <h2 class="text-xl font-bold mb-4">"People List"</h2>
            <ul>
                <li>"People 1"</li>
                <li>"People 2"</li>
                <li>"People 3"</li>
          </ul>
        </div>
    }
}
