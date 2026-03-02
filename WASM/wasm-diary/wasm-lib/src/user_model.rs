use serde::{Deserialize,
            Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAccount {
    pub id: String,
    pub username: String,
    pub nickname: Option<String>,
    pub password_hash: String,
    pub salt: String,
    pub role: Role,
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
