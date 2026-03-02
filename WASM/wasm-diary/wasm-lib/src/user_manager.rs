use crate::{date_util,
            hash_util,
            id_gen,
            user_model::{Role,
                         UserAccount}};
use serde::{Deserialize,
            Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct RegistrationValidation {
    valid: bool,
    username_error: Option<String>,
    password_error: Option<String>,
}

/// 로그인 결과: 성공 시 사용자 정보, 실패 시 에러 메시지
#[derive(Debug, Serialize, Deserialize)]
struct LoginResult {
    success: bool,
    user_id: Option<String>,
    username: Option<String>,
    nickname: Option<String>,
    role: Option<Role>,
    error: Option<String>,
}

/// JS에 공개할 사용자 정보 (비밀번호 해시/salt 제외)
#[derive(Debug, Serialize, Deserialize)]
struct UserView {
    id: String,
    username: String,
    nickname: Option<String>,
    role: Role,
    created_at: String,
}

impl From<&UserAccount> for UserView {
    fn from(account: &UserAccount) -> Self {
        Self {
            id: account.id.clone(),
            username: account.username.clone(),
            nickname: account.nickname.clone(),
            role: account.role,
            created_at: account.created_at.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct UserManager {
    users: Vec<UserAccount>,
}

// 내부 헬퍼 (테스트 시 ID/타임스탬프/salt 주입용)
impl UserManager {
    fn register_with(
        &mut self,
        id: String,
        username: String,
        nickname: Option<String>,
        password: String,
        role: Role,
        salt: String,
        now: String,
    ) -> String {
        let password_hash = hash_util::hash_password(&password, &salt);
        let account = UserAccount {
            id,
            username,
            nickname,
            password_hash,
            salt,
            role,
            created_at: now,
        };
        let result = LoginResult {
            success: true,
            user_id: Some(account.id.clone()),
            username: Some(account.username.clone()),
            nickname: account.nickname.clone(),
            role: Some(account.role),
            error: None,
        };
        let json = serde_json::to_string(&result).unwrap_or_default();
        self.users.push(account);
        json
    }

    #[cfg(test)]
    fn ensure_admin_with(&mut self, id: String, salt: String, now: String) {
        if !self.users.iter().any(|u| u.role == Role::Admin) {
            let password_hash = hash_util::hash_password("admin123", &salt);
            self.users.push(UserAccount {
                id,
                username: "admin".to_string(),
                nickname: None,
                password_hash,
                salt,
                role: Role::Admin,
                created_at: now,
            });
        }
    }
}

#[wasm_bindgen]
impl UserManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut mgr = Self { users: Vec::new() };
        mgr.ensure_admin();
        mgr
    }

    /// 관리자 계정이 없으면 기본 admin/admin123 계정을 생성한다.
    fn ensure_admin(&mut self) {
        if !self.users.iter().any(|u| u.role == Role::Admin) {
            let id = id_gen::generate_id();
            let salt = hash_util::generate_salt();
            let now = date_util::now_iso();
            let password_hash = hash_util::hash_password("admin123", &salt);
            self.users.push(UserAccount {
                id,
                username: "admin".to_string(),
                nickname: None,
                password_hash,
                salt,
                role: Role::Admin,
                created_at: now,
            });
        }
    }

    /// localStorage에서 읽어온 JSON으로 상태를 복원한다.
    pub fn load_from_json(&mut self, json: &str) {
        if let Ok(users) = serde_json::from_str::<Vec<UserAccount>>(json) {
            self.users = users;
        }
        self.ensure_admin();
    }

    /// 현재 상태를 JSON으로 직렬화한다.
    pub fn save_to_json(&self) -> String {
        serde_json::to_string(&self.users).unwrap_or_else(|_| "[]".to_string())
    }

    /// 회원가입. 유효성 검사 후 성공/실패 JSON을 반환한다.
    pub fn register(
        &mut self,
        username: &str,
        password: &str,
        role: Role,
        nickname: Option<String>,
    ) -> String {
        let validation = self.validate_registration(username, password);
        let parsed: RegistrationValidation =
            serde_json::from_str(&validation).unwrap_or(RegistrationValidation {
                valid: false,
                username_error: Some("검증 오류".to_string()),
                password_error: None,
            });

        if !parsed.valid {
            return serde_json::to_string(&LoginResult {
                success: false,
                user_id: None,
                username: None,
                nickname: None,
                role: None,
                error: parsed
                    .username_error
                    .or(parsed.password_error)
                    .unwrap_or_else(|| "유효성 검사 실패".to_string())
                    .into(),
            })
            .unwrap_or_default();
        }

        let nickname = nickname.filter(|n| !n.trim().is_empty());
        let id = id_gen::generate_id();
        let salt = hash_util::generate_salt();
        let now = date_util::now_iso();
        self.register_with(
            id,
            username.trim().to_string(),
            nickname,
            password.to_string(),
            role,
            salt,
            now,
        )
    }

    /// 로그인. 성공 시 사용자 정보, 실패 시 에러 메시지 JSON을 반환한다.
    pub fn login(&self, username: &str, password: &str) -> String {
        let username_trimmed = username.trim();
        if let Some(user) = self
            .users
            .iter()
            .find(|u| u.username == username_trimmed)
        {
            if hash_util::verify_password(password, &user.salt, &user.password_hash) {
                return serde_json::to_string(&LoginResult {
                    success: true,
                    user_id: Some(user.id.clone()),
                    username: Some(user.username.clone()),
                    nickname: user.nickname.clone(),
                    role: Some(user.role),
                    error: None,
                })
                .unwrap_or_default();
            }
        }

        serde_json::to_string(&LoginResult {
            success: false,
            user_id: None,
            username: None,
            nickname: None,
            role: None,
            error: Some("아이디 또는 비밀번호가 올바르지 않습니다".to_string()),
        })
        .unwrap_or_default()
    }

    /// 전체 사용자 목록을 반환한다 (비밀번호 정보 제외).
    pub fn get_all_users(&self) -> String {
        let views: Vec<UserView> = self.users.iter().map(UserView::from).collect();
        serde_json::to_string(&views).unwrap_or_else(|_| "[]".to_string())
    }

    /// 사용자를 삭제한다. 성공 시 true.
    pub fn delete_user(&mut self, id: &str) -> bool {
        let before = self.users.len();
        self.users.retain(|u| u.id != id);
        self.users.len() < before
    }

    /// 사용자의 역할을 변경한다. 성공 시 true.
    pub fn change_role(&mut self, id: &str, role: Role) -> bool {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == id) {
            user.role = role;
            true
        } else {
            false
        }
    }

    /// 회원가입 유효성 검사. JSON으로 결과를 반환한다.
    pub fn validate_registration(&self, username: &str, password: &str) -> String {
        let username_trimmed = username.trim();
        let mut valid = true;
        let mut username_error: Option<String> = None;
        let mut password_error: Option<String> = None;

        if username_trimmed.is_empty() {
            valid = false;
            username_error = Some("사용자 ID를 입력해주세요".to_string());
        } else if !username_trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            valid = false;
            username_error =
                Some("사용자 ID는 영문, 숫자, 언더스코어(_)만 사용할 수 있습니다".to_string());
        } else if username_trimmed.len() < 3 {
            valid = false;
            username_error = Some("사용자 ID는 3자 이상이어야 합니다".to_string());
        } else if username_trimmed.len() > 20 {
            valid = false;
            username_error = Some("사용자 ID는 20자 이하여야 합니다".to_string());
        } else if self
            .users
            .iter()
            .any(|u| u.username == username_trimmed)
        {
            valid = false;
            username_error = Some("이미 사용 중인 사용자 ID입니다".to_string());
        }

        if password.is_empty() {
            valid = false;
            password_error = Some("비밀번호를 입력해주세요".to_string());
        } else if password.len() < 6 {
            valid = false;
            password_error = Some("비밀번호는 6자 이상이어야 합니다".to_string());
        } else if password.len() > 100 {
            valid = false;
            password_error = Some("비밀번호는 100자 이하여야 합니다".to_string());
        }

        serde_json::to_string(&RegistrationValidation {
            valid,
            username_error,
            password_error,
        })
        .unwrap_or_else(|_| r#"{"valid":false}"#.to_string())
    }

    /// 사용자 ID로 역할을 조회한다.
    pub fn get_user_role(&self, user_id: &str) -> String {
        self.users
            .iter()
            .find(|u| u.id == user_id)
            .map(|u| serde_json::to_string(&u.role).unwrap_or_default())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manager() -> UserManager {
        let mut mgr = UserManager { users: Vec::new() };
        mgr.ensure_admin_with(
            "admin-id".to_string(),
            "admin-salt".to_string(),
            "2026-03-02T00:00:00Z".to_string(),
        );
        mgr
    }

    #[test]
    fn 새_매니저는_관리자_계정을_가진다() {
        let mgr = make_manager();
        assert_eq!(mgr.users.len(), 1);
        assert_eq!(mgr.users[0].username, "admin");
        assert_eq!(mgr.users[0].role, Role::Admin);
    }

    #[test]
    fn 회원가입이_성공한다() {
        let mut mgr = make_manager();
        let result = mgr.register_with(
            "user-1".to_string(),
            "testuser".to_string(),
            None,
            "password123".to_string(),
            Role::User,
            "test-salt".to_string(),
            "2026-03-02T10:00:00Z".to_string(),
        );

        let parsed: LoginResult = serde_json::from_str(&result).unwrap();
        assert!(parsed.success);
        assert_eq!(parsed.username.unwrap(), "testuser");
        assert_eq!(mgr.users.len(), 2);
    }

    #[test]
    fn 로그인이_성공한다() {
        let mut mgr = make_manager();
        mgr.register_with(
            "user-1".to_string(),
            "testuser".to_string(),
            None,
            "mypassword".to_string(),
            Role::User,
            "salt-1".to_string(),
            "2026-03-02T10:00:00Z".to_string(),
        );

        let result = mgr.login("testuser", "mypassword");
        let parsed: LoginResult = serde_json::from_str(&result).unwrap();
        assert!(parsed.success);
        assert_eq!(parsed.user_id.unwrap(), "user-1");
    }

    #[test]
    fn 잘못된_비밀번호로_로그인하면_실패한다() {
        let mut mgr = make_manager();
        mgr.register_with(
            "user-1".to_string(),
            "testuser".to_string(),
            None,
            "mypassword".to_string(),
            Role::User,
            "salt-1".to_string(),
            "2026-03-02T10:00:00Z".to_string(),
        );

        let result = mgr.login("testuser", "wrongpassword");
        let parsed: LoginResult = serde_json::from_str(&result).unwrap();
        assert!(!parsed.success);
        assert!(parsed.error.is_some());
    }

    #[test]
    fn 존재하지_않는_사용자로_로그인하면_실패한다() {
        let mgr = make_manager();
        let result = mgr.login("nonexistent", "password");
        let parsed: LoginResult = serde_json::from_str(&result).unwrap();
        assert!(!parsed.success);
    }

    #[test]
    fn 관리자_기본_계정으로_로그인한다() {
        let mgr = make_manager();
        let result = mgr.login("admin", "admin123");
        let parsed: LoginResult = serde_json::from_str(&result).unwrap();
        assert!(parsed.success);
        assert_eq!(parsed.role.unwrap(), Role::Admin);
    }

    #[test]
    fn 전체_사용자_목록을_조회한다() {
        let mut mgr = make_manager();
        mgr.register_with(
            "user-1".to_string(),
            "user1".to_string(),
            None,
            "pass".to_string(),
            Role::User,
            "s".to_string(),
            "2026-03-02T00:00:00Z".to_string(),
        );

        let all = mgr.get_all_users();
        let views: Vec<UserView> = serde_json::from_str(&all).unwrap();
        assert_eq!(views.len(), 2);
        // password_hash, salt가 없어야 함
        let json_str = &all;
        assert!(!json_str.contains("password_hash"));
        assert!(!json_str.contains("\"salt\""));
    }

    #[test]
    fn 사용자를_삭제한다() {
        let mut mgr = make_manager();
        mgr.register_with(
            "user-1".to_string(),
            "user1".to_string(),
            None,
            "pass".to_string(),
            Role::User,
            "s".to_string(),
            "2026-03-02T00:00:00Z".to_string(),
        );

        assert!(mgr.delete_user("user-1"));
        assert_eq!(mgr.users.len(), 1);
    }

    #[test]
    fn 역할을_변경한다() {
        let mut mgr = make_manager();
        mgr.register_with(
            "user-1".to_string(),
            "user1".to_string(),
            None,
            "pass".to_string(),
            Role::User,
            "s".to_string(),
            "2026-03-02T00:00:00Z".to_string(),
        );

        assert!(mgr.change_role("user-1", Role::Admin));
        assert_eq!(mgr.users[1].role, Role::Admin);
    }

    #[test]
    fn 중복_사용자명을_거부한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("admin", "password123");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(!parsed.valid);
        assert!(parsed.username_error.unwrap().contains("이미 사용 중"));
    }

    #[test]
    fn 짧은_사용자명을_거부한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("ab", "password123");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(!parsed.valid);
        assert!(parsed.username_error.unwrap().contains("3자 이상"));
    }

    #[test]
    fn 짧은_비밀번호를_거부한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("newuser", "12345");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(!parsed.valid);
        assert!(parsed.password_error.unwrap().contains("6자 이상"));
    }

    #[test]
    fn 빈_사용자명을_거부한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("", "password123");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(!parsed.valid);
    }

    #[test]
    fn 빈_비밀번호를_거부한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("newuser", "");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(!parsed.valid);
    }

    #[test]
    fn json으로_저장하고_복원한다() {
        let mut mgr = make_manager();
        mgr.register_with(
            "user-1".to_string(),
            "user1".to_string(),
            None,
            "pass".to_string(),
            Role::User,
            "s".to_string(),
            "2026-03-02T00:00:00Z".to_string(),
        );

        let json = mgr.save_to_json();
        let mut mgr2 = UserManager { users: Vec::new() };
        mgr2.load_from_json(&json);

        assert_eq!(mgr2.users.len(), 2);
        assert_eq!(mgr2.users[0].username, "admin");
        assert_eq!(mgr2.users[1].username, "user1");
    }

    #[test]
    fn 사용자_역할을_조회한다() {
        let mgr = make_manager();
        let role = mgr.get_user_role("admin-id");
        assert_eq!(role, "\"Admin\"");
    }

    #[test]
    fn 존재하지_않는_사용자의_역할_조회는_빈_문자열이다() {
        let mgr = make_manager();
        let role = mgr.get_user_role("nonexistent");
        assert!(role.is_empty());
    }

    #[test]
    fn 유효한_회원가입_검증이_통과한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("newuser", "password123");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(parsed.valid);
    }

    #[test]
    fn 영문이_아닌_사용자_id를_거부한다() {
        let mgr = make_manager();
        let result = mgr.validate_registration("한글유저", "password123");
        let parsed: RegistrationValidation = serde_json::from_str(&result).unwrap();
        assert!(!parsed.valid);
        assert!(parsed.username_error.unwrap().contains("영문"));
    }
}
