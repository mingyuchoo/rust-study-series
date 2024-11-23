use leptos::*;

// So far, we've only been working with local state in components
// We've only seen how to communicate between parent and child components
// Be there are also more general ways to manage global state
//
// The three best approaches to global state are
// 1. Using the router to drive global state via the URL
// 2. Passing signals through context
// 3. Creating a global state struct and creating lenses into it with
//    `create_slice`
//
// Option #1: URL as Global State
// The next few ections of the tutorial will be about the router.
// So for now, we'll look at options #2 and #3.

// Option #2: Pass Signals through Context
//
// In virtual DOM libraries like React, using the Context API to manage global
// state is a bad idea: because the entire app exists in a tree, changing
// some value provided high up in the tree can cause the whole app to render.
//
// In fine-grained reactive libraries like Leptos, this is simply not the case.
// You can create a signal in the root of your app and pass it down to other
// omponents using provide_context(). Changing it will only cause rerendering
// in th specific place it is actually used, not the whole app.

#[component]
pub fn GlobalStateManagement() -> impl IntoView {
    view! {
        <main>
            <h1>"Gobal State Management"</h1>
            <Option2/>
            <Option3/>
        </main>
    }
}

#[component]
pub fn Option2() -> impl IntoView {
    // here we create a signal in the root that can be consumed
    // anywhere in the app.
    let (count, set_count) = create_signal(0);
    // we will pass the setter to specific components,
    // but provide the count itself to the whole app via context
    provide_context(count);

    view! {
        <main>
            <h2>"Option 2: Passing Signals"</h2>
            // SetterButton is allowed to modify the count
            <SetterButton set_count/>
            // These consumers can only read from it
            // But we could give them write access by passing `set_count` if we wanted
            <div style="display: flex">
                <FancyMath/>
            </div>
        </main>
    }
}

/// A button that increment our global counter.
#[component]
fn SetterButton(set_count: WriteSignal<u32>) -> impl IntoView {
    view! {
        <div class="provide red">
            <button on:click=move |_| set_count.update(|count| *count += 1)>
                "Increment Global Count"
            </button>
        </div>
    }
}

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
    // here we consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>()
    // we know we just provided this in the parent component
        .expect("there to be a `count` signal provided");
    let is_even = move || count() & 1 == 0;

    view! {
        <div class="consumer blue">
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                    " is not"
            }}
            " even."
        </div>
    }
}

/// A component that shows a list of items generated form the global count.
#[component]
fn ListItems() -> impl IntoView {
    // again, consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>().expect("there to be a `count` signal provided");
    let squares = move || {
        (0 .. count()).map(|n| view! {<li>{n}<sup>"2"</sup> " is " {n * n}</li>})
                      .collect::<Vec<_>>()
    };
    view! {
            <div class="consumer green">
                <ul>{squares}</ul>
            </div>
    :    }
}

// Option #3: Create a Global State Struct
//
// You can use this approach to build a signal global data structure
// that holds the state for your whole app, and then access it by
// taking fine-grained slices using `create_slice` or `create_memo`,
// so that changing one part of the state doesn't cause parts of your
// app that depend on other parts of the state to change.

#[derive(Default, Clone, Debug)]
struct GlobalState {
    count: u32,
    name:  String,
}

#[component]
fn Option3() -> impl IntoView {
    // we will provide a single signal that holds the whole state
    // each component will be responsible for creating its own "lens" into it
    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        <main>
            <h2>"Option 3: Passing Signals"</h2>
            <div class="red consumer" style="width: 100%">
                <h3>"Current Global State"</h3>
                <pre>
                    {move || {
                        format!("{:#?}", state.get())
                    }}
                </pre>
            </div>
            <div style="display: flex">
                <GlobalStateCounter/>
                <GlobalStateInput/>
            </div>
        </main>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to hae been provided");

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) =
        create_slice(// we take a slice *from* `state`
                     state,
                     // our getter returns a "slice" of the data
                     |state| state.count,
                     // our setter describes how to mutate that slice, given a
                     // new value
                     |state, n| state.count = n);
    view! {
        <div class="consumer blue">
            <button on:click=move |_| {
                set_count(count() + 1);
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateInput() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided.");

    // this slice is completely independent of the `count` slice
    // that we created in the other component
    // neither of them will cause the other to rerun
    let (name, set_name) = create_slice(// we take a slice *from* `state`
                                        state,
                                        // our getter returns a "slice" of the
                                        // data
                                        |state| {
                                            state.name
                                                 .clone()
                                        },
                                        // our setter describes how to mutate
                                        // that slice, given a new vlaue
                                        |state, n| state.name = n);

    view! {
        <div class="consumer green">
            <input type="text" prop:value=name on:input=move |ev| { set_name(event_target_value(&ev))}/>
            <br/>
            <span>"Name is: "{name}</span>
        </div>
    }
}
