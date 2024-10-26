#[cfg(test)]
mod tests {
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기

    #[test]
    fn internal() {
        assert_eq!(4, internal_addr(2, 2));
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_addr(a, 2)
}

fn internal_addr(a: i32,
                 b: i32)
                 -> i32 {
    a + b
}
