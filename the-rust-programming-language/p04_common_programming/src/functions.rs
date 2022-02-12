pub fn other_function() {
  println!("다른 함수");
  another_function(5, 6);
}

pub fn another_function(x: i32, y: i32) {
  println!("또 다른 함수");
  println!("x의 값: {}", x);
  println!("y의 값: {}", y);
}

pub fn function_body() {
  let x = 5;
  let y = { let x = 3; x + 1 };

  println!("x의 값: {}", x);
  println!("y의 값: {}", y);
}

fn five() -> i32 {
  5
}
fn plus_one(x: i32) -> i32 {
  x + 1
}
pub fn return_value() {
  let x = five();
  let y = plus_one(x);
  println!("x의 값: {}", x);
  println!("y의 값: {}", y);
}
