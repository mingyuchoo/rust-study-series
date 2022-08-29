pub fn floating() {
  let x = 2.0;      // f64
  let y: f32 = 3.0; // f32

  println!("x: {}, y: {}", x, y);
}

pub fn binary_operations() {
  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let remainder = 43 % 5;

  print!("sum: {}, ", sum);
  print!("difference: {}, ", difference);
  print!("product: {}, ", product);
  print!("quotient: {}, ", quotient);
  println!("remainder: {} ", remainder);
}

pub fn boolean() {
  let t = true;
  let f: bool = false;

  if t {
    println!("t is true");
  }

  if f {
    println!("t is true");
  } else {
    println!("f is false");
  }
}

pub fn characters() {
  let c = 'z';
  let z = 'Z';
  let apple = '🍎';

  println!("c: {}", c);
  println!("z: {}", z);
  println!("apple: {}", apple);
}


pub fn tuples() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("x: {}, y: {}, z: {}", x, y, z);

  let w: (i32, f64, u8) = (500, 6.4 , 1);
  let five_hundred = w.0;
  let six_point_four = w.1;
  let one = w.2;

  println!("x: {}, y: {}, z: {}", five_hundred, six_point_four, one);
}

pub fn arrays() {
  let a           = [1,2,3,4,5];
  let b: [i32; 5] = [1,2,3,4,5];
  let c           = [3; 5];

  let first = a[0];
  println!("an element of a: {}", first);
  println!("an element of b: {}", b[1]);
  println!("an element of c: {}", c[2]);
}
