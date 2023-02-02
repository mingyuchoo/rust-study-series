use p12_automated_testing::internal_fn;
mod common;

// `$ cargo test --test integration`
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(internal_fn::add_two(2), 4);
}
