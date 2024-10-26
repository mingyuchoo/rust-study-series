mod ex1;
mod ex2;
mod ex3;
mod ex4;
mod ex5;
mod ex6;

fn main() -> Result<(), std::io::Error>
{
    ex1::call1().expect("panicked!");
    ex2::call1().expect("panicked!");
    ex3::call1().expect("panicked!");
    ex4::call1().expect("panicked!");
    ex5::call1().expect("panicked!");
    ex6::call1()
}
