fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!"); // Hello, world!

    print!("Hello, world!"); // Without new line
    println!(); // A new line

    println!("Hello, world!\n"); // With new line

    println!("{}, {}!", "Hello", "world"); // Hello, world!

    println!("{0}, {1}!", "Hello", "world"); // Hello, world!

    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!

    println!("{:?}", [1, 2, 3]); // [1, 2, 3]

    println!("{:#?}", [1, 2, 3]);
    // [
    // 1,
    // 2,
    // 3
    // ]

    Ok(())
}
