//! A Simple Hello World Crate

mod basic; // `basic` 모듈을 선언하고, 해당 모듈의 콘텐츠를 가져오기

const PI: f64 = 3.14159265359;

/// This is main function for starting
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Hello, world!");

    println!("π value is {}", PI);

    basic::vars::main();

    basic::funs::main();

    Ok(())
}
