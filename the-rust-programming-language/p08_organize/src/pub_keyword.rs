mod back_of_house {
    // 모듈 `back_of_house` 를 선언하고, 모듈 콘텐츠를 가져오기
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 여름에 아침 식사로 호밀빵을 주문한다.
    let mut meal = back_of_house::Breakfast::summer("호밀빵");

    // 마음이 변해 빵 종류를 바꾼다.
    meal.toast = String::from("밀빵");

    println!("{} 토스트로 주세요", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("{:?}, {:?}", order1, order2);
}
