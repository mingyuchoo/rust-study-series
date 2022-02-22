mod generic_functions;
mod generic_data_types;
mod traits;

fn main() {
    generic_functions::call1();
    generic_functions::call2();

    generic_data_types::call1();
    generic_data_types::call3();
    generic_data_types::call4();
    generic_data_types::call5();
    generic_data_types::call6();

    traits::call1();
    traits::call2();
    traits::call3();
    traits::call4();
}
