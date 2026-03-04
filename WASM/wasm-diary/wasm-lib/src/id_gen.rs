use uuid::Uuid;

/// 암호학적으로 안전한 UUID v4를 생성한다.
/// WASM 환경에서는 `getrandom`이 Web Crypto API를 사용한다.
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_는_uuid_v4_형식이다() {
        let id = generate_id();
        // UUID v4: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx (36자)
        assert_eq!(id.len(), 36);
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
        // 버전 비트: 세 번째 그룹이 '4'로 시작
        assert!(parts[2].starts_with('4'));
    }

    #[test]
    fn 연속_생성_id는_서로_다르다() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }
}
