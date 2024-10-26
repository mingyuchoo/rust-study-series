enum List
{
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons,
           Nil};

pub fn call1()
{
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("a를 생성한 뒤 카운터 = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));

    println!("b를 생성한 뒤 카운터 = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("c를 생성한 뒤 카운터 = {}", Rc::strong_count(&a));
    }

    println!("c가 범위를 벗어난 뒤 카운터 = {}", Rc::strong_count(&a));
}
