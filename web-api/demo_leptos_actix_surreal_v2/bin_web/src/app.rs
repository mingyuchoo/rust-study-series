use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use lib_adder::add_left_right;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link
            rel="preload"
            href="https://fonts.googleapis.com/css?family=IBM+Plex+Sans:400,600,700"
            as_="font"
            type_="font/woff2"
            crossorigin="anonymous"
        />
        <Stylesheet id="leptos" href="/pkg/bin_web.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <nav>
                <div>
                    <A href="">"Home"</A>
                    <A href="/people">"People"</A>
                </div>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/people" view=PeoplePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let add_resource = create_resource(move || count.get(),
                                       |current_count| async move {
                                           add_left_right(current_count,
                                                          current_count + 1)
                                       });
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <h2>"Counter"</h2>
        <button on:click=on_click>"Click Me: " {count}</button>
        <h3>"The result value"</h3>
        <p>
            {move || add_resource.get()
                               .map(|result| result.to_string())
                               .unwrap_or_else(|| "Loading...".to_string())
            }
        </p>
    }
}

#[component]
fn PeoplePage() -> impl IntoView {
    let people_resource = create_resource(|| (), |_| get_people());
    let people = move || match people_resource.get() {
        | Some(Ok(people)) => people,
        | Some(Err(_)) => vec![],
        | None => vec![],
    };

    view! {
        <h1>"People"</h1>
        <h2>"People List"</h2>
        <Suspense fallback=move || view! { <p>"Loading ..."</p>}>
            <ErrorBoundary fallback=|_errors| {view! {<p>"Something went wrong."</p>}}>
                <ul>
                    <For each=people key=|person| person.id.clone() let:person>
                        <li>
                            {format!("{} - {}", person.id, person.name)}
                        </li>
                    </For>
                </ul>
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server]
pub async fn get_people() -> Result<Vec<Person>, ServerFnError> {
    use lib_repo::DB;

    let people = DB.select("person")
                   .await?;

    println!("{:?}", people);

    Ok(people)
}

use lib_repo::Id;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    id:   Id,
    name: String,
}
