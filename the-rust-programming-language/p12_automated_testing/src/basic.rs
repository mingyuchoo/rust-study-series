#[cfg(test)]
mod tests {
    // `tests` 모듈을 선언하기

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // `$ cargo test -- --ignored`
    #[test]
    #[ignore]
    fn another() {
        panic!("테스트 실패!");
    }
}
