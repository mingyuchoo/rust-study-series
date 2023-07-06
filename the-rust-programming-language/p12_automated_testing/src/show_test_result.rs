/**
 * `$ cargo test -- --nocapture`
 * `$ cargo test -- --nocapture --test-threads=1`
 */
#[cfg(test)]
mod tests {
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // `$ cargo test -- --ignored`
    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("입력값: {a}");
    10
}
