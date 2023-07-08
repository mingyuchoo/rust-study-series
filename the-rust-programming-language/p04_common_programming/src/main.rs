mod data_types; // 모듈 경로를 현재 범위 안으로 가져오기
mod flow_control; // 모듈 경로를 현재 범위 안으로 가져오기
mod functions; // 모듈 경로를 현재 범위 안으로 가져오기
mod variables; // 모듈 경로를 현재 범위 안으로 가져오기

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
