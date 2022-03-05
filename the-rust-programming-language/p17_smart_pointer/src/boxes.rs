enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::boxes::List::{Cons, Nil};

pub fn call1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

pub fn call2() {
    let list = Cons(1,
                   Box::new(Cons(2,
                       Box::new(Cons(3,
                           Box::new(Nil))))));
}