mod front_of_house {
    // 모듈 front_of_house` 를 선언하고, 모듈 콘텐츠를 가져오기
    pub mod hosting {
        // 모듈 `hosting` 를 공개로 선언하고, 모듈 콘텐츠를 가져오기
        pub fn add_to_waitlist() {}
    }
}

pub use crate::pub_use_keyword::front_of_house::hosting; // 모듈 경로를 현재 범위 안으로 가져오고 공개하기

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
