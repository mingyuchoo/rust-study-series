use leptos::*;
use leptos_router::*;

mod part1;
mod part2;

#[component]
pub fn App() -> impl IntoView {
    mount_to_body(|| {
        view! {
            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/part1/basic_component/BasicComponent" view=part1::basic_component::BasicComponent/>
                    <Route path="/part1/dynamic_attributes/DynamicAttributes" view=part1::dynamic_attributes::DynamicAttributes/>
                    <Route path="/part1/components_and_props/ComponentsAndProps" view=part1::components_and_props::ComponentsAndProps/>
                    <Route path="/part1/iteration/Iteration" view=part1::iteration::Iteration/>
                    <Route path="/part1/iteration_with_for/IterationWithFor" view=part1::iteration_with_for::IterationWithFor/>
                    <Route path="/part1/forms_and_inputs/ControlledInputs" view=part1::forms_and_inputs::ControlledInputs/>
                    <Route path="/part1/forms_and_inputs/UncontrolledInputs" view=part1::forms_and_inputs::UncontrolledInputs/>
                    <Route path="/part1/forms_and_inputs/SpecialCasesTextarea" view=part1::forms_and_inputs::SpecialCasesTextarea/>
                    <Route path="/part1/forms_and_inputs/SpecialCasesSelect" view=part1::forms_and_inputs::SpecialCasesSelect/>
                    <Route path="/part1/control_flow/SoWhat" view=part1::control_flow::SoWhat/>
                    <Route path="/part1/control_flow/PreventingOverRendering" view=part1::control_flow::PreventingOverRendering/>
                    <Route path="/part1/control_flow/TypeConversions" view=part1::control_flow::TypeConversions/>
                    <Route path="/part1/error_handling/NumbericInput" view=part1::error_handling::NumericInput/>
                    <Route path="/part1/parent_child_communication/PassAWriteSignal" view=part1::parent_child_communication::PassAWriteSignal/>
                    <Route path="/part1/parent_child_communication/UseACallback" view=part1::parent_child_communication::UseACallback/>
                    <Route path="/part1/parent_child_communication/ProvidingAContext" view=part1::parent_child_communication::ProvidingAContext/>
                    <Route path="/part1/passing_children_to_components/ComponentChildren" view=part1::passing_children_to_components::ComponentChildren/>
                    <Route path="/part1/passing_children_to_components/ManipulatingChildren" view=part1::passing_children_to_components::ManipulatingChildren/>
                    <Route path="/part1/reactivity/working_with_signals/GettingAndSetting" view=part1::reactivity::working_with_signals::GettingAndSetting/>
                    <Route path="/part1/reactivity/making_signals_depend_on/GoodOptions" view=part1::reactivity::making_signals_depend_on::GoodOptions/>
                    <Route path="/part1/reactivity/responding_to_changes_with_create_effect/Basic" view=part1::reactivity::responding_to_changes_with_create_effect::Basic/>
                    <Route path="/part1/reactivity/responding_to_changes_with_create_effect/ZeroCostishAbstraction" view=part1::reactivity::responding_to_changes_with_create_effect::ZeroCostishAbstraction/>
                    <Route path="/part1/reactivity/responding_to_changes_with_create_effect/CancelableTrackingWithWatch" view=part1::reactivity::responding_to_changes_with_create_effect::CancelableTrackingWithWatch/>
                    <Route path="/part1/reactivity/interlude/ReactivityAndFunctions" view=part1::reactivity::interlude::ReactivityAndFunctions/>
                    <Route path="/part1/testing/test_business_logic/HardToTest" view=part1::testing::test_business_logic::HardToTest/>
                    <Route path="/part1/testing/test_business_logic/EasyToTest" view=part1::testing::test_business_logic::EasyToTest/>
                    <Route path="/part1/testing/end_to_end_testing/WasmBindgenTest" view=part1::testing::end_to_end_testing::WasmBindgenTest/>
                    <Route path="/part1/asynchronous/loading_data_with_resources/CreateResource" view=part1::asynchronous::loading_data_with_resources::CreateResource/>
                    <Route path="/part1/asynchronous/suspense_component/SuspenseComponent" view=part1::asynchronous::suspense_component::SuspenseComponent/>
                    <Route path="/part1/asynchronous/transition_component/TransitionComponent" view=part1::asynchronous::transition_component::TransitionComponent/>
                    <Route path="/part1/asynchronous/create_action/CreateAction" view=part1::asynchronous::create_action::CreateAction/>
                    <Route path="/part1/interlude/projecting_children/ProjectingChildren" view=part1::interlude::projecting_children::ProjectingChildren/>
                    <Route path="/part1/globalstatemanagement/global_state_management/GlobalStateManagement" view=part1::globalstatemanagement::global_state_management::GlobalStateManagement/>
                    <Route path="/part1/routing/defining_routes/DefiningRoutes" view=part1::routing::defining_routes::DefiningRoutes/>
                    <Route path="/part2/typicode/Api" view=part2::typicode::Api/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </Router>
        }
    })
}

#[component]
fn Home() -> impl IntoView {
    let links = vec![("/", "Home"),
                     ("/part1/basic_component/BasicComponent", "Basic Component"),
                     ("/part1/dynamic_attributes/DynamicAttributes", "DynamicAttributes"),
                     ("/part1/components_and_props/ComponentsAndProps", "ComponentsAndProps"),
                     ("/part1/iteration/Iteration", "Iteration"),
                     ("/part1/iteration_with_for/IterationWithFor", "IterationWithFor"),
                     ("/part1/forms_and_inputs/ControlledInputs", "ControlledInputs"),
                     ("/part1/forms_and_inputs/UncontrolledInputs", "UncontrolledInputs"),
                     ("/part1/forms_and_inputs/SpecialCasesTextarea", "SpecialCasesTextarea"),
                     ("/part1/forms_and_inputs/SpecialCasesSelect", "SpecialCasesSelect"),
                     ("/part1/control_flow/SoWhat", "SoWhat"),
                     ("/part1/control_flow/PreventingOverRendering", "PreventingOverRendering"),
                     ("/part1/control_flow/TypeConversions", "TypeConversions"),
                     ("/part1/error_handling/NumbericInput", "NumericInput"),
                     ("/part1/parent_child_communication/PassAWriteSignal", "PassAWriteSignal"),
                     ("/part1/parent_child_communication/UseACallback", "UseACallback"),
                     ("/part1/parent_child_communication/ProvidingAContext", "ProvidingAContext"),
                     ("/part1/passing_children_to_components/ComponentChildren", "ComponentChildren"),
                     ("/part1/passing_children_to_components/ManipulatingChildren", "ManipulatingChildren"),
                     ("/part1/reactivity/working_with_signals/GettingAndSetting", "GettingAndSetting"),
                     ("/part1/reactivity/making_signals_depend_on/GoodOptions", "GoodOptions"),
                     ("/part1/reactivity/responding_to_changes_with_create_effect/Basic", "Basic"),
                     ("/part1/reactivity/responding_to_changes_with_create_effect/ZeroCostishAbstraction",
                      "ZeroCostishAbstraction"),
                     ("/part1/reactivity/responding_to_changes_with_create_effect/CancelableTrackingWithWatch",
                      "CancelableTrackingWithWatch"),
                     ("/part1/reactivity/interlude/ReactivityAndFunctions", "ReactivityAndFunctions"),
                     ("/part1/testing/test_business_logic/HardToTest", "HardToTest"),
                     ("/part1/testing/test_business_logic/EasyToTest", "EasyToTest"),
                     ("/part1/testing/end_to_end_testing/WasmBindgenTest", "WasmBindgenTest"),
                     ("/part1/asynchronous/loading_data_with_resources/CreateResource", "CreateResource"),
                     ("/part1/asynchronous/suspense_component/SuspenseComponent", "SuspenseComponent"),
                     ("/part1/asynchronous/transition_component/TransitionComponent", "TransitionComponent"),
                     ("/part1/asynchronous/create_action/CreateAction", "CreateAction"),
                     ("/part1/interlude/projecting_children/ProjectingChildren", "ProjectingChildren"),
                     ("/part1/globalstatemanagement/global_state_management/GlobalStateManagement",
                      "GlobalStateManagement"),
                     ("/part1/routing/defining_routes/DefiningRoutes", "DefiningRoutes"),
                     ("/part2/typicode/Api", "Api"),];

    view! {
        <main>
            <h1>"Home"</h1>
            <ul>
                {links.into_iter().map(|(href, text)| view! {
                    <li><a href={href}>{text}</a></li>
                }).collect::<Vec<_>>()}
            </ul>
        </main>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <main>
            <h1>"Page Not Found."</h1>
        </main>
    }
}
