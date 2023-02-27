pub fn main() {
    hello();

    // 01. Without type declarations.
    let p1 = plus_one;
    let x = p1(5);
    println!("{}", x); // 6

    // 02. With type declarations.
    // and using variable shadowing
    let p1: fn(i32) -> i32 = plus_one;
    let x = p1(5);
    println!("{}", x); // 6

    // 03. Call other function
    let p2: fn(i32) -> i32 = plus_two;
    let y = p2(5);
    println!("{}", y); // 7

    let x = 2;
    println!("{}", get_square_value(x));

    // Input parameters are passed inside | | and expression body is wrapped within { }
    let square = |i: i32| -> i32 { i * i };
    println!("{}", square(x)); // 4
}

/// This function returns the greeting; Hello, world!
pub fn hello() -> String {
    println!("{}", "Hello, world!");
    ("Hello, world!").to_string()
}

/// 01. Without the return keyword. Only the last expression returns
pub fn plus_one(a: i32) -> i32 {
    a + 1
    // There is no ending ; in the above line.
    // It means this is an expression which equals to `return a + 1;`.
}


/// 02. With the return keyword.
pub fn plus_two(a: i32) -> i32 {
    return a + 2;
    // Should use return keyword only on conditional / early returns.
    // Using return keyword in the last expression is a bad practice.
}

/// Closures
pub fn get_square_value(i: i32) -> i32 {
    i * i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(-1), 0);
        assert_eq!(plus_one(0), 1);
        assert_eq!(plus_one(1), 2);
    }

    #[test]
    fn test_plus_two() {
        assert_eq!(plus_two(-1), 1);
        assert_eq!(plus_two(0), 2);
        assert_eq!(plus_two(1), 3);
    }

    #[test]
    fn test_get_square_value() {
        assert_eq!(get_square_value(0), 0);
        assert_eq!(get_square_value(1), 1);
        assert_eq!(get_square_value(2), 4);
    }
}
