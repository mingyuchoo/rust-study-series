/// WASM 환경에서 현재 시각을 ISO 8601 형식으로 반환한다.
/// `chrono`를 사용하지 않고 `js_sys::Date`로 직접 포맷팅한다.
pub fn now_iso() -> String {
    let date = js_sys::Date::new_0();
    date.to_iso_string().as_string().unwrap_or_default()
}

/// ISO 8601 날짜 문자열에서 날짜 부분(YYYY-MM-DD)만 추출한다.
pub fn extract_date(iso_string: &str) -> &str { iso_string.get(.. 10).unwrap_or(iso_string) }

/// 주어진 날짜가 범위(from ~ to) 안에 있는지 판정한다.
/// 비교는 ISO 문자열의 사전식 비교로 수행한다 (YYYY-MM-DD 형식이므로 정상
/// 동작).
pub fn is_in_range(date: &str, from: &str, to: &str) -> bool {
    let d = extract_date(date);
    let f = extract_date(from);
    let t = extract_date(to);
    d >= f && d <= t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 날짜_부분을_추출한다() {
        assert_eq!(extract_date("2026-03-02T10:30:00.000Z"), "2026-03-02");
    }

    #[test]
    fn 짧은_문자열은_그대로_반환한다() {
        assert_eq!(extract_date("2026"), "2026");
    }

    #[test]
    fn 범위_안의_날짜를_판정한다() {
        assert!(is_in_range("2026-03-02T10:00:00Z", "2026-03-01", "2026-03-03"));
        assert!(is_in_range("2026-03-01T00:00:00Z", "2026-03-01", "2026-03-01"));
    }

    #[test]
    fn 범위_밖의_날짜를_판정한다() {
        assert!(!is_in_range("2026-02-28T23:59:59Z", "2026-03-01", "2026-03-03"));
        assert!(!is_in_range("2026-03-04T00:00:00Z", "2026-03-01", "2026-03-03"));
    }
}
