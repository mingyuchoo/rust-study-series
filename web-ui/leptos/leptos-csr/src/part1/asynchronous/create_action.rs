use gloo_timers::future::TimeoutFuture;
use leptos::html::Input;
use leptos::*;
use uuid::Uuid;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
    _ = text;
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    // pretend this is a post ID or something
    Uuid::new_v4()
}

#[component]
pub fn CreateAction() -> impl IntoView {
    // an action takes an async function with single argument
    // it can be a simple type, a struct, or ()
    let add_todo = create_action(|input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });
    // action provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>();

    view! {
        <main>
            <h1>"Create Action"</h1>
            <form
                on:submit=move |ev| {
                    ev.prevent_default(); // don't reload the page...
                    let input = input_ref.get().expect("input to exist");
                    add_todo.dispatch(input.value());
                }
            >
                <label>"What do you need to do?"<input type="text" node_ref=input_ref/></label>
                <button type="submit">"Add Todo"</button>
            </form>
            <p>{move || pending().then(|| "Loading...")}</p>
            <p>"Submitted: "<code>{move || format!("{:?}", submitted())}</code></p>
            <p>"Pending: "<code>{move || format!("{:?}", pending())}</code></p>
            <p>"Todo ID: "<code>{move || format!("{:?}", todo_id())}</code></p>
        </main>
    }
}
