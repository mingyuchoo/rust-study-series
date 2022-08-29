pub fn scope() {
  println!("-- scope()");

  let s = "hello";
  {
    {
      let s = "world";
      println!("{}", s);
    }
    println!("{}", s);
  }

  println!("{}", s);
}
