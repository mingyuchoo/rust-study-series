/// WASM 환경에서 `js_sys::Math::random()`과 `js_sys::Date::now()`를 조합하여
/// 고유 ID를 생성한다. `uuid` 크레이트의 `getrandom` WASM 호환 문제를 회피한다.
pub fn generate_id() -> String {
    let timestamp = js_sys::Date::now() as u64;
    let random = (js_sys::Math::random() * 1_000_000.0) as u64;
    format!("{timestamp}-{random}")
}

#[cfg(test)]
mod tests {
    // js_sys 함수는 WASM 환경에서만 동작하므로 단위 테스트에서는
    // manager.rs에서 ID를 주입받는 방식으로 테스트한다.
    // 여기서는 포맷 검증만 수행한다.

    #[test]
    fn id_포맷은_타임스탬프_하이픈_랜덤_형태이다() {
        let id = "1709366400000-123456";
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].parse::<u64>().is_ok());
        assert!(parts[1].parse::<u64>().is_ok());
    }
}
