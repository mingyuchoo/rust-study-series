#[cfg(test)]
mod tests {
    use super::*;
    // `$ cargo test -- --ignored`
    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("캐롤");
        assert!(
            result.contains("캐롤"),
            "Greeting 함수의 결과에 이름이 없어요. 결괏값: '{}'",
            result
        );
    }
}

pub fn greeting(name: &str) -> String {
    // format!("안녕하세요 {}!", name)
    format!("안녕하세요!")
}
