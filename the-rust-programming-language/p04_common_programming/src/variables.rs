pub fn mutation()
{
    let mut x = 5;
    println!("x의 값: {x}");
    x = 6;
    println!("x의 값: {x}");
}

pub fn shadowing()
{
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner scope에서 x의 값: {x}");
    }
    println!("x의 값: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces의 값: {spaces}");
}
