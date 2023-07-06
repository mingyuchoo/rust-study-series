mod front_of_house {
    // 모듈 `front_of_house` 를 선언하고, 모듈 콘텐츠를 가져오기
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::use_keyword::front_of_house::hosting; // 해당 모듈 경로를 현재 범위 안으로 가져오기
                                                 // 상대경로 용법 - `use self::front_of_house::hosting;`

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// idiomatic
use std::collections::HashMap; // 해당 모듈 경로를 현재 범위 안으로 가져오기
pub fn call1() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
