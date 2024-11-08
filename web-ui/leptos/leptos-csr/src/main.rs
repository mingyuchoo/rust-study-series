use leptos::*;

mod part1;

fn main()
{
    mount_to_body(|| {
        view! {
            <part1::basic_component::BasicComponent/>
            <part1::dynamic_attributes::DynamicAttributes/>
            <part1::components_and_props::ComponentsAndProps/>
            <part1::iteration::Iteration/>
        }
    })
}