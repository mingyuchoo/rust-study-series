#[cfg(test)]
mod tests {
    // `tests` 모듈을 선언하기
    use super::*; // 상대경로 `super`로 상위 모듈 경로를 현재 범위 안으로 가져오기

    // `$ cargo test -- --ignored`
    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("캐롤");
        assert!(result.contains("캐롤"),
                "Greeting 함수의 결과에 이름이 없어요. 결괏값: '{}'",
                result);
    }
}

pub fn greeting(name: &str) -> String {
    // format!("안녕하세요 {}!", name)
    format!("안녕하세요!")
}
