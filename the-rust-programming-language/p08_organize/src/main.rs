mod as_keyword; // 모듈 경로를 현재 범위 안으로 가져오기
mod mod_keyword; // 모듈 경로를 현재 범위 안으로 가져오기
mod pub_keyword; // 모듈 경로를 현재 범위 안으로 가져오기
mod pub_use_keyword; // 모듈 경로를 현재 범위 안으로 가져오기
mod use_keyword; // 모듈 경로를 현재 범위 안으로 가져오기

fn main() {
    mod_keyword::eat_at_restaurant();

    pub_keyword::eat_at_restaurant();

    use_keyword::eat_at_restaurant();
    use_keyword::call1();

    // as_keyword::call1();
    // as_keyword::call2();

    pub_use_keyword::eat_at_restaurant();
}
