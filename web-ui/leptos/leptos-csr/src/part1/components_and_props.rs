use leptos::*;

#[component]
pub fn ComponentsAndProps() -> impl IntoView
{
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <h1>Components and props</h1>
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Click me"
        </button>
        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(/// The maximum value of the progress bar.
               #[prop(default = 100)]
               max: u16,
               /// How much progress should be displayed.
               #[prop(into)]
               progress: Signal<i32>)
               -> impl IntoView
{
    view! {
        <progress max=max value=progress />

    }
}
