/**
 * `$ cargo test one_hundred`
 * `$ cargo test add`
 */
#[cfg(test)]
mod tests {
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
