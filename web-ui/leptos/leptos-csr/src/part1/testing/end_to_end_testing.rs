use leptos::*;

#[component]
pub fn WasmBindgenTest() -> impl IntoView {
    view! {
        <main>
            <h1>"wasm-bindgen-test"</h1>
            <p>
                <a
                    href="https://github.com/leptos-rs/leptos/blob/main/examples/counter/tests/web.rs"
                    target="_blank"
                >
                    "Leptos Examples Counter"
                </a>
            </p>
        </main>
    }
}
