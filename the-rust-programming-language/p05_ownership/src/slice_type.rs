pub fn get_first_word() {
  println!("-- get_first_word()");

  let s: String = String::from("hello world");
  let word = first_word_with_slices(&s);

  // s.clear();  // error!

  println!("the first word is: {}", word);

  fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return i;
      }
    }
    s.len()
  }

  fn first_word_with_slices(s: &str) -> &str { // &String == &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }
    &s[..]
  }
}