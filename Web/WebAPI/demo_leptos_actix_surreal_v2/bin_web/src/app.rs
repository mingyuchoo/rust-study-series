use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use lib_adder::add_left_right;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    uuid: String,
    name: String,
}

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
    let add_resource = create_resource(
        move || count.get(),
        |current_count| async move {
            add_left_right(current_count, current_count + 1)
        },
    );
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div>
            <h2>"Counter"</h2>
            <div>
                <button on:click=on_click>"Click Me: " {count}</button>
                <h3>"The result value"</h3>
                <div>
                    <p>
                        {move || add_resource.get()
                                           .map(|result| result.to_string())
                                           .unwrap_or_else(|| "Loading...".to_string())
                        }
                    </p>
                </div>
            </div>
        </div>
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

    let (selected_person, set_selected_person) = create_signal(None::<Person>);
    let (new_name, set_new_name) = create_signal(String::new());
    let add_person_action = create_server_action::<AddPerson>();
    let refresh_people = Callback::new(move |_: String| {
        people_resource.refetch();
        set_selected_person.set(None);
    });
    create_effect(move |_| {
        if add_person_action.version().get() > 0 {
            people_resource.refetch();
        }
    });

    view! {
        <h1>"People"</h1>
        <div>
            <Suspense fallback=move || view! { <p>"Loading ..."</p>}>
                <p>
                    {move || {
                        let count = people().len();
                        format!("There are {} people.", count)
                    }}
                </p>
            </Suspense>
            <h2>"Add a person"</h2>
            <form on:submit=move |event| {
              event.prevent_default();
              add_person_action.dispatch(AddPerson { name: new_name.get() });
              set_new_name.set(String::new());
            }>
                <input
                    type="text"
                    placeholder="Enter name"
                    prop:value=new_name
                    on:input=move |event| set_new_name.set(event_target_value(&event))
                />
                <button type="submit">
                    "Add Person"
                </button>
            </form>
            <h2>"People List"</h2>
            <div>
                <Suspense fallback=move || view! { <p>"Loading ..."</p>}>
                    <ErrorBoundary fallback=|_errors| {view! {<p>"Something went wrong."</p>}}>
                        <ul>
                            <For each=people key=|person| person.uuid.clone() let:person>
                                <li>
                                {let person_clone = person.clone();
                                    view! {
                                        <a on:click=move |_| set_selected_person(Some(person_clone.clone()))
                                            href="#"
                                        >
                                            {format!("{} - {}", person.uuid, person.name)}
                                        </a>
                                    }}
                                </li>
                            </For>
                        </ul>
                    </ErrorBoundary>
                </Suspense>
                <PersonDetails
                    person=selected_person
                    on_delete=refresh_people
                />
            </div>
        </div>
    }
}

#[component]
fn PersonDetails(
    person: ReadSignal<Option<Person>>,
    on_delete: Callback<String>,
) -> impl IntoView {
    let delete_person_action = create_server_action::<DeletePerson>();
    let handle_delete = move |uuid: String| {
        delete_person_action.dispatch(DeletePerson {
            uuid: uuid.clone()
        });
        on_delete.call(uuid);
    };
    view! {
        <Show
            when=move || person.get().is_some()
            fallback=|| view! {<div><p>"Select a person to view details"</p></div>}
        >
            {move || {
                let person = person.get().unwrap();
                view! {
                    <h3>"Person Details"</h3>
                    <div>
                        <p>"UUID: " {person.uuid.to_string()}</p>
                        <p>"Name: " {person.name}</p>
                    </div>
                    <div>
                        <button on:click=move |_| {
                            delete_person_action.dispatch(DeletePerson { uuid: person.uuid.clone(), });
                            handle_delete(person.uuid.to_string())
                        }>
                            "Delete Person"
                        </button>
                    </div>
                }
            }}
        </Show>
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

    let people = DB.select("person").await?;
    Ok(people)
}

#[server(AddPerson, "/api")]
pub async fn add_person(name: String) -> Result<Option<Person>, ServerFnError> {
    use lib_repo::DB;
    use uuid::Uuid;

    let new_person = Person {
        uuid: Uuid::new_v4().to_string(),
        name,
    };

    let created: Option<Person> = DB
        .create(("person", new_person.uuid.clone()))
        .content(new_person)
        .await?;

    Ok(created)
}

#[server(DeletePerson, "/api")]
pub async fn delete_person(
    uuid: String,
) -> Result<Option<Person>, ServerFnError> {
    use lib_repo::DB;
    let deleted: Option<Person> = DB.delete(("person", uuid)).await?;

    Ok(deleted)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use lib_repo::DB;
        use uuid::Uuid;
    }
}
