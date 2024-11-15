/// floating
pub fn floating() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {x}, y: {y}");
}

pub fn binary_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // subtraction
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    print!("sum: {sum}, ");
    print!("difference: {difference}, ");
    print!("product: {product}, ");
    print!("quotient: {quotient}, ");
    print!("truncated: {truncated}, ");
    println!("remainder: {remainder} ");
}

/// boolean
pub fn boolean() {
    let t = true;
    let f: bool = false;

    if t {
        println!("t is true");
    }

    if f {
        println!("t is true");
    }
    else {
        println!("f is false");
    }
}

/// characters
pub fn characters() {
    let c = 'z';
    let z = 'Z';
    let apple = '🍎';

    println!("c: {c}");
    println!("z: {z}");
    println!("apple: {apple}");
}

/// tuples
pub fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let w: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = w.0;
    let six_point_four = w.1;
    let one = w.2;

    println!("x: {x}, y: {y}, z: {z}");
    println!("x: {five_hundred}, y: {six_point_four}, z: {one}");
}

/// arrays
pub fn arrays() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    let first = a[0];

    println!("an element of a: {first}");
    println!("an element of b: {}", b[1]);
    println!("an element of c: {}", c[2]);
}
