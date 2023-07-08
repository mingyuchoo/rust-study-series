use p12_automated_testing::internal_fn; // 모듈 경로를 현재 범위 안으로 가져오기
mod common; // 모듈을 선언하고, 모듈 콘텐츠를 가져오기

// `$ cargo test --test integration`
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(internal_fn::add_two(2), 4);
}
