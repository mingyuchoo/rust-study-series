mod front_of_house
{
    // 모듈 `front_of_house` 를 선언하고, 모듈 콘텐츠를 가져오기
    pub mod hosting
    {
        // 모듈 `hosting` 를 선언하고, 모듈 콘텐츠를 가져오기
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving
    {
        // 모듈 `serving` 를 선언하고, 모듈 콘텐츠를 가져오기
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant()
{
    // absolute path
    crate::mod_keyword::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
