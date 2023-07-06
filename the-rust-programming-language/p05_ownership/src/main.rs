mod complex_data_type; // 모듈 경로를 현재 범위 안으로 가져오기
mod memory_allocation; // 모듈 경로를 현재 범위 안으로 가져오기
mod reference_and_borrow; // 모듈 경로를 현재 범위 안으로 가져오기
mod slice_type; // 모듈 경로를 현재 범위 안으로 가져오기
mod variable_scope; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    variable_scope::scope();

    complex_data_type::string_type();

    memory_allocation::move_scalar();
    memory_allocation::move_complex();
    memory_allocation::relationship();
    memory_allocation::return_ownership();

    reference_and_borrow::return_value();
    reference_and_borrow::mutable_reference();
    reference_and_borrow::dead_reference();

    slice_type::get_first_word();
    slice_type::get_element_of_array();
}
