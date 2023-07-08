fn serve_order() {}

mod back_of_house {
    // 모듈 `back_of_house` 를 선언하고, 모듈 콘텐츠를 가져오기
    fn fix_incorrect_order() {
        cook_order();

        // use relative path by `super` keyword
        super::serve_order();
    }
    fn cook_order() {}
}
