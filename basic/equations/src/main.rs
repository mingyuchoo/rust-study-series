fn main() -> Result<(), Box<dyn std::error::Error>>
{
    what_value_of_s_balances_the_scale();

    Ok(())
}

/// 2s + 10 = s + 15
fn what_value_of_s_balances_the_scale() -> ()
{
    let max = 100;

    // FIXED: NOT idiomatic code
    //
    // for s in 1..max {
    //    if left(s) == right(s) {
    //        println!("I found the number s: {}", s);
    //}

    match (1 .. max).find(|&x| left(x) == right(x)) {
        | Some(x) => println!("I found the number x: {}", x),
        | None => println!("No number found that balances the scale."),
    }
}

fn left(s: i32) -> i32
{
    2 * s + 10
}

fn right(s: i32) -> i32
{
    s + 15
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_left()
    {
        assert_eq!(left(-1), 8);
        assert_eq!(left(0), 10);
        assert_eq!(left(1), 12);
    }

    #[test]
    fn test_right()
    {
        assert_eq!(right(-1), 14);
        assert_eq!(right(0), 15);
        assert_eq!(right(1), 16);
    }
}
