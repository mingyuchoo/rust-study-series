mod variable_scope;
mod complex_data_type;
mod memory_allocation;
mod reference_and_borrow;
mod slice_type;

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
}
