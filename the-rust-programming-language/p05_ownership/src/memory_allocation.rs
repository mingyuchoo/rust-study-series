pub fn move_scalar() {
  println!("-- move_scalar()");

  let x = 5;  // stack memory
  let y = x;

  println!("{}", x);
  println!("{}", y);
}

pub fn move_complex() {
  println!("-- move_complex()");

  let s1 = String::from("hello"); // heap memory
  let s2 = s1.clone();

  println!("{}", s1);
  println!("{}", s2);

}

pub fn relationship() {
  println!("-- relationship()");

  let s = String::from("hello");
  take_ownership(s);

  let x = 5;
  makes_copy(x);

  let s1 = gives_ownership();
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2);

  println!("{}, {}", s1, s3);

  fn take_ownership(some_string: String) {
    println!("{}", some_string);
  }

  fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
  }

  fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
  }

  fn takes_and_gives_back(a_string: String) -> String {
    a_string
  }
}

pub fn return_ownership() {
  println!("-- return_ownership()");

  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  println!("'{}'의 길이는 {}입니다.", s2, len);

  fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
  }
}