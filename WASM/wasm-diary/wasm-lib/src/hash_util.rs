use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// `getrandom`(WASM: Web Crypto API)을 통해 암호학적으로 안전한 salt를 생성한다.
/// 반환값은 base64url 인코딩된 SaltString 문자열이다.
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

/// argon2id로 비밀번호를 해싱한다.
/// `salt`는 base64url 인코딩된 SaltString이어야 한다.
/// 반환값은 PHC 문자열 형식(`$argon2id$...`)이며, salt가 내장된다.
pub fn hash_password(password: &str, salt: &str) -> String {
    let salt_string = match SaltString::from_b64(salt) {
        Ok(s) => s,
        Err(_) => return String::new(),
    };
    Argon2::default()
        .hash_password(password.as_bytes(), &salt_string)
        .map(|h| h.to_string())
        .unwrap_or_default()
}

/// 비밀번호가 저장된 PHC 해시와 일치하는지 검증한다.
/// salt는 PHC 문자열에 내장되어 있으므로 `_salt` 파라미터는 무시된다.
pub fn verify_password(password: &str, _salt: &str, stored_hash: &str) -> bool {
    match PasswordHash::new(stored_hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 테스트용 고정 salt (base64url 인코딩된 16바이트 값)
    const TEST_SALT: &str = "AAAAAAAAAAAAAAAAAAAAAA";
    const TEST_SALT_2: &str = "BBBBBBBBBBBBBBBBBBBBBA";

    #[test]
    fn 같은_입력이면_같은_해시를_생성한다() {
        let hash1 = hash_password("password123", TEST_SALT);
        let hash2 = hash_password("password123", TEST_SALT);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn 다른_salt이면_다른_해시를_생성한다() {
        let hash1 = hash_password("password123", TEST_SALT);
        let hash2 = hash_password("password123", TEST_SALT_2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn 다른_비밀번호이면_다른_해시를_생성한다() {
        let hash1 = hash_password("password123", TEST_SALT);
        let hash2 = hash_password("password456", TEST_SALT);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn 해시는_argon2id_phc_형식이다() {
        let hash = hash_password("test", TEST_SALT);
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn 올바른_비밀번호를_검증한다() {
        let hash = hash_password("mypassword", TEST_SALT);
        assert!(verify_password("mypassword", TEST_SALT, &hash));
    }

    #[test]
    fn 잘못된_비밀번호를_거부한다() {
        let hash = hash_password("mypassword", TEST_SALT);
        assert!(!verify_password("wrongpassword", TEST_SALT, &hash));
    }
}
