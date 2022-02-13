mod variable_scope;
mod complex_data_type;
mod memory_allocation;
mod reference_and_borrow;

fn main() {
    variable_scope::scope();

    complex_data_type::string_type();

    memory_allocation::move_scalar();
    memory_allocation::move_complex();
    memory_allocation::relationship();
    memory_allocation::return_ownership();

    reference_and_borrow::return_value();
}
