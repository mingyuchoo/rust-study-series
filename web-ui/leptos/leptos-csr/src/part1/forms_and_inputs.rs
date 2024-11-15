use leptos::*;

#[component]
pub fn ControlledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <h2>"ControlledInputs"</h2>
        <input
            type="text"
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value in JavaScript,
                // but smooths out some of the typecasting necessary to
                // make this work in Rust
                set_name(event_target_value(&ev));
            }
            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn UncontrolledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
                                   // event handler can only fire
                                   // after the view is mounted to the DOM,
                                   // so the `NodeRef` will be `Some`
                                   .expect("<input> should be mounted")
                                   // `leptoss::HtmlElement<html::Input>` implements
                                   // `Deref` to a `web_sys::HtmlInputElement`.
                                   // this means we can call `HtmlInputElement::value()`
                                   // to get the current value of the input
                                   .value();
        set_name(value);
    };

    view! {
        <h2>"UncontrolledInputs"</h2>
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn SpecialCasesTextarea() -> impl IntoView {
    let (name, set_name) = create_signal("SpecialCases".to_string());
    view! {
        <h2>"SpecialCasesTextarea"</h2>
        <textarea
            prop:value=move || name.get()
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        >
            {name.get_untracked()}
        </textarea>
    }
}

#[component]
pub fn SpecialCasesSelect() -> impl IntoView {
    let (value, set_value) = create_signal(0i32);
    view! {
        <h2>"SpecialCasesSelect"</h2>
        <select
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value(new_value.parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button
            on:click=move |_| set_value.update(|n| {
                match *n {
                    | 2 => *n = 0,
                    | _ => *n += 1,
                }
            })
        >
            "Next Option"
        </button>
    }
}
