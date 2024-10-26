pub fn other_function()
{
    println!("다른 함수");
    another_function(5, 6);
    print_labeled_measurement(5, 'h');
}

fn another_function(x: i32,
                    y: i32)
{
    println!("또 다른 함수");
    println!("x의 값: {x}");
    println!("y의 값: {y}");
}

fn print_labeled_measurement(value: i32,
                             unit_label: char)
{
    println!("The measurement is: {value}{unit_label}");
}

/// Statements and Expressions
/// - Statements are instructions that perform some action and do not return a
///   value.
/// - Expressions evaluate to a resultant value. Let’s look at some examples.
pub fn function_body()
{
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("x의 값: {x}");
    println!("y의 값: {y}");
}

/// Functions with Return Values
pub fn return_value()
{
    let x = five();
    let y = plus_one(x);
    println!("x의 값: {x}");
    println!("y의 값: {y}");
}

fn five() -> i32
{
    5
}

fn plus_one(x: i32) -> i32
{
    x + 1
}
