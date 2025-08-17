use std::ops::Deref;

#[test]
pub fn test1() {
    let x = 5;
    let y = &x;

    println!("{}", x);
    println!("{}", &x);
    println!("{}", y);
    println!("{}", &y);
    println!("{}", *y);

    assert_eq!(5, x);
    // assert_eq!(5, &x);   // ERROR
    // assert_eq!(5, y);    // ERROR
    // assert_eq!(5, &y);   // ERROR
    assert_eq!(5, *y);
}

#[test]
pub fn test2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, &x);   // ERROR
    // assert_eq!(5, y);    // ERROR
    // assert_eq!(5, &y);   // ERROR
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
pub fn test3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, &x);   // ERROR
    // assert_eq!(5, y);    // ERROR
    // assert_eq!(5, &y);   // ERROR
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("안녕하세요 {name}!");
}

pub fn call3() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
