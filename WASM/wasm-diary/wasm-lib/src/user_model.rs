use serde::{Deserialize,
            Serialize};
use wasm_bindgen::prelude::*;

/// 사용자 역할. 권한 수준을 결정한다.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// 전체 사용자 관리 및 모든 일기 열람 권한
    Admin,
    /// 본인 일기 작성/수정/삭제 권한
    User,
}

/// 사용자 계정 정보. `password_hash`와 `salt`는 JS 레이어에 노출하지 않는다.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAccount {
    /// 고유 식별자 (UUID v4)
    pub id: String,
    /// 로그인 ID (영문·숫자·언더스코어, 3~20자)
    pub username: String,
    /// 표시 이름 (선택)
    pub nickname: Option<String>,
    /// argon2id PHC 형식 비밀번호 해시
    pub password_hash: String,
    /// base64url 인코딩 salt (password_hash 내에도 내장됨)
    pub salt: String,
    /// 계정 역할
    pub role: Role,
    /// 계정 생성 일시 (ISO 8601)
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 역할_enum을_json으로_직렬화한다() {
        let role = Role::Admin;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, "\"Admin\"");

        let deserialized: Role = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Role::Admin);
    }

    #[test]
    fn 사용자_계정을_json으로_직렬화한다() {
        let account = UserAccount {
            id: "user-1".to_string(),
            username: "testuser".to_string(),
            nickname: Some("테스트유저".to_string()),
            password_hash: "hash123".to_string(),
            salt: "salt123".to_string(),
            role: Role::User,
            created_at: "2026-03-02T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&account).unwrap();
        let deserialized: UserAccount = serde_json::from_str(&json).unwrap();
        assert_eq!(account, deserialized);
    }

    #[test]
    fn 사용자_목록을_json으로_직렬화한다() {
        let accounts = vec![
            UserAccount {
                id: "1".to_string(),
                username: "admin".to_string(),
                nickname: None,
                password_hash: "h1".to_string(),
                salt: "s1".to_string(),
                role: Role::Admin,
                created_at: "2026-03-01T00:00:00Z".to_string(),
            },
            UserAccount {
                id: "2".to_string(),
                username: "user1".to_string(),
                nickname: Some("유저1".to_string()),
                password_hash: "h2".to_string(),
                salt: "s2".to_string(),
                role: Role::User,
                created_at: "2026-03-02T00:00:00Z".to_string(),
            },
        ];

        let json = serde_json::to_string(&accounts).unwrap();
        let deserialized: Vec<UserAccount> = serde_json::from_str(&json).unwrap();
        assert_eq!(accounts, deserialized);
    }
}
