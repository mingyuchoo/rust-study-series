fn main() {
    let num = 10;
    println!("{num} plus one is {}!", library_one::add_one(num));
    println!("{num} plus two is {}!", library_two::add_two(num));
    println!("{num} plus three is {}!", library_three::add_three(num));
}
