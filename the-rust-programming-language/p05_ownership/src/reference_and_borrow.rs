pub fn return_value() {
  println!("-- return_value()");

  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("'{}'의 길이는 {}입니다.", s1, len);

  fn calculate_length(s: &String) -> usize {
    s.len()
  }

  let mut s2 = String::from("hello");
  println!("address of     s2: {:p}", &s2);
  println!("address of mut s2: {:p}", &mut s2);

  change(&mut s2);
  println!("{}", s2);

  fn change(some_string: &mut String) {
    println!("some_string: {}, address of some_string: {:p}", some_string, some_string);
    some_string.push_str(", world");
  }
}
