use leptos::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[component]
pub fn PeoplePage() -> impl IntoView {
    let people_resource = Resource::new(|| (), |_| get_people());
    let people = move || people_resource.get();

    view! {
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <h2 class="text-xl font-bold mb-4">"People List"</h2>
            <Suspense fallback=move || view! { <p>"Loading ..."</p>}>
                <ErrorBoundary fallback=|_errors| {view! {<p>"Something went wrong."</p>}}>
                    <ul>
                        <For each=people key=|person| person.id let:person>
                            <li>
                                {person.name}
                            </li>
                        </For>
                    </ul>
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[server]
pub async fn get_people() -> Result<Vec<Person>, ServerFnError> {
    Ok(PEOPLE.iter()
             .cloned()
             .collect())
}

// Dummy API
static PEOPLE: LazyLock<[Person; 3]> = LazyLock::new(|| {
    [Person { id:   1,
              name: "Adam".to_string(), },
     Person { id:   2,
              name: "Bob".to_string(), },
     Person { id:   3,
              name: "Chris".to_string(), }]
});

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    id:   usize,
    name: String,
}
