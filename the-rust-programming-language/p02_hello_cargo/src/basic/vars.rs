pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    variables();
    consts();
    statics();
    shadowing();

    Ok(())
}

/// This function is an example for variable bindings
pub fn variables() {
    let a; // Declaration; without data type
    a = 1; // Assignment
    println!("{0}", a);

    let b: i8; // Declaration; with data type
    b = 2;
    println!("{b}", b = b);

    let t = true; // Declaration + assignment; without data type
    println!("{}", t);

    let f: bool = false; // Declaration + aasignment; with data type
    println!("{}", f);

    let (x, y) = (1, 2); // x = 1 and y = 2
    println!("{:?}", (x, y));
    println!("{:#?}", (x, y));

    let mut z = 5;
    println!("{0}", z);

    z = 6;
    println!("{0}", z);

    let z = { x + y }; // z = 3
    println!("{0}", z);

    let z = {
        let x = 3;
        let y = 4;

        x + y
    }; // z = 7
    println!("{0}", z);
}

/// This function is an example for consts
pub fn consts() {
    const N: i32 = 5;
    println!("{}", N);
}

/// This function is an example for statics
pub fn statics() {
    static N: i32 = 5;
    println!("{}", N);
}

/// This function is an example for variable shadowing
pub fn shadowing() {
    let x: f64 = -20.48; // float
    let x: i64 = x.floor() as i64; // int
    println!("{}", x); // -21

    let s: &str = "hello"; // &str
    let s: String = s.to_uppercase(); // String
    println!("{}", s); // HELLO
}
