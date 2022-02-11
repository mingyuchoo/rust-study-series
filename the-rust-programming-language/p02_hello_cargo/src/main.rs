//! A Simple Hello World Crate

mod basic; // Import basic module

const PI: f64 = 3.14159265359;

/// This is main function for starting
fn main() {
    println!("π value is {}", PI);
    basic::vars::main();
    basic::funs::main();
}
