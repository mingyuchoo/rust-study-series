use create_collection::add;

fn main() {
    let a: u64 = 2;
    let b: u64 = 3;
    let sum = add(a, b);
    println!("add({}, {}) = {}", a, b, sum);
}