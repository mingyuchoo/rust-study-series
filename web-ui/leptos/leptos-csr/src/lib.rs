use leptos::*;

mod part1;
mod part2;

pub fn App() -> impl IntoView {
    mount_to_body(|| {
        view! {
            <part1::basic_component::BasicComponent/>
            <part1::dynamic_attributes::DynamicAttributes/>
            <part1::components_and_props::ComponentsAndProps/>
            <part1::iteration::Iteration/>
            <part1::iteration_with_for::IterationWithFor/>
            <part2::typicode::Api/>
        }
    })
}
