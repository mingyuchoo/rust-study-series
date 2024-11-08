#[cfg(test)]
mod tests {
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]), }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self,
                message: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages
                                 .borrow()
                                 .len(),
                   1);
    }
}

pub trait Messenger {
    fn send(&self,
            msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value:     usize,
    max:       usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger,
{
    pub fn new(messenger: &T,
               max: usize)
               -> LimitTracker<T> {
        LimitTracker { messenger,
                       value: 0,
                       max }
    }

    pub fn set_value(&mut self,
                     value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("경고: 최댓값의 75%를 사용했습니다.");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("긴급 경고: 최대값의 90%를 사용했습니다.");
        } else if percentage_of_max >= 1.0 {
            self.messenger
                .send("에러: 최대값을 초과했습니다.");
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::{cell::RefCell,
          rc::Rc};
use List::{Cons,
           Nil};

pub fn call1() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a 수정 후 = {a:?}");
    println!("b 수정 후 = {b:?}");
    println!("c 수정 후 = {c:?}");
}
