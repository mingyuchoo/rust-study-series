pub fn string_type() {
  println!("-- string_type()");

  let s = String::from("hello");
  println!("{}", s);

  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);
}
