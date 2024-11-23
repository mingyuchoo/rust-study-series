use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            | Cons(_, ref item) => Some(item),
            | Nil => None,
        }
    }
}

pub fn call1() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a의 최초 rc 카운트 = {}", Rc::strong_count(&a));
    println!("a의 다음 아이템 = {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    println!("b를 생성한 뒤 a의 rc 카우트 = {}", Rc::strong_count(&a));
    println!("b의 최초 rc 카운트 = {}", Rc::strong_count(&b));
    println!("b의 다음 아이템 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a를 변경한 뒤 b의 rc 카운트 = {}", Rc::strong_count(&b));
    println!("a를 변경한 뒤 a의 rc 카운트 = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
struct Node {
    value:    i32,
    parent:   RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn call2() {
    let leaf = Rc::new(Node { value:    3,
                              parent:   RefCell::new(Weak::new()),
                              children: RefCell::new(vec![]), });

    println!("leaf strong= {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node { value:    5,
                                    parent:   RefCell::new(Weak::new()),
                                    children:
                                        RefCell::new(vec![Rc::clone(&leaf)]), });

        *leaf.parent
             .borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
        println!("leaf strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
    }

    println!("leaf parent = {:?}",
             leaf.parent
                 .borrow()
                 .upgrade());
    println!("leaf strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));
}
