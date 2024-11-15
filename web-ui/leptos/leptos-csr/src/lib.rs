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
            <part1::forms_and_inputs::ControlledInputs/>
            <part1::forms_and_inputs::UncontrolledInputs/>
            <part1::forms_and_inputs::SpecialCasesTextarea/>
            <part1::forms_and_inputs::SpecialCasesSelect/>
            <part1::control_flow::SoWhat/>
            <part1::control_flow::PreventingOverRendering/>
            <part1::control_flow::TypeConversions/>
            <part1::error_handling::NumericInput/>
            <part1::parent_child_communication::PassAWriteSignal/>
            <part1::parent_child_communication::UseACallback/>
            <part1::parent_child_communication::ProvidingAContext/>
            <part2::typicode::Api/>
        }
    })
}
