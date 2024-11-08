use leptos::*;

#[component]
pub fn Iteration() -> impl IntoView
{
    let values = vec![0, 1, 2];
    let length = 5;
    let counters = (1 ..= length).map(|idx| create_signal(idx));
    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters.map(|(count, set_count)| {
                              view! {
                                  <li>
                                      <button on:click=move |_| set_count.update(|n| *n += 1)>
                                          {count}
                                      </button>
                                  </li>
                              }
                          })
                          .collect_view();

    view! {
            <h1>Iteration</h1>
            // this will just render "012"
            <p>{values.clone()}</p>
            // or we can wrap them in <li>
            <ul>
                {values
                    .into_iter()
                    .map(|n| view! { <li>{n}</li>})
                    .collect_view()
                }
            </ul>
            <ul>
                {counter_buttons}
            </ul>
    }
}
