mod data_types;
mod flow_control;
mod functions;
mod variables;

fn main() {
    variables::mutation();
    variables::shadowing();

    data_types::floating();
    data_types::binary_operations();
    data_types::boolean();
    data_types::characters();
    data_types::tuples();
    data_types::arrays();

    functions::other_function();
    functions::function_body();
    functions::return_value();

    flow_control::if_conditions();
    flow_control::let_and_if();
    flow_control::loops();
    flow_control::labeled_loops();
    flow_control::whiles();
    flow_control::for_element();
    flow_control::for_rev();
}
