fn left(s: i32) -> i32 {
    2 * s + 10
}

fn right(s: i32) -> i32 {
    s + 15
}

fn what_value_of_s_balances_the_scale() -> () {

    let max: i32 = 100;

    for s in 1..max {
        if left(s) == right(s) {
            println!("I found the number s: {}", s);
        }
    }
}

fn main() {
    what_value_of_s_balances_the_scale();
}
