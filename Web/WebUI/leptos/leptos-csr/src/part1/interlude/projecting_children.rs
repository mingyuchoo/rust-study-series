use leptos::*;

#[component]
pub fn ProjectingChildren() -> impl IntoView {
    let name = "Alice".to_string();
    view! {
        <main>
            <h1>"Projecting Children"</h1>
            <Outer clone:name>
                <Inner clone:name>
                    <Inmost name=name.clone()/>
                </Inner>
            </Outer>
        </main>
    }
}

#[component]
pub fn Outer(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
pub fn Inner(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
pub fn Inmost(name: String) -> impl IntoView {
    view! {
        <p>{name}</p>
    }
}
