mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::use_keyword::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// idiomatic
use std::collections::HashMap;
pub fn call1() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
