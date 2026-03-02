use sha2::{Digest,
          Sha256};

/// WASM 환경에서 `js_sys::Math::random()`을 사용하여 랜덤 salt를 생성한다.
pub fn generate_salt() -> String {
    let mut salt = String::new();
    for _ in 0..4 {
        let random = (js_sys::Math::random() * u32::MAX as f64) as u32;
        salt.push_str(&format!("{random:08x}"));
    }
    salt
}

/// salt와 비밀번호를 결합하여 SHA-256 해시를 생성한다.
pub fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{b:02x}")).collect()
}

/// 비밀번호가 저장된 해시와 일치하는지 검증한다.
pub fn verify_password(password: &str, salt: &str, stored_hash: &str) -> bool {
    hash_password(password, salt) == stored_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 같은_입력이면_같은_해시를_생성한다() {
        let hash1 = hash_password("password123", "salt-abc");
        let hash2 = hash_password("password123", "salt-abc");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn 다른_salt이면_다른_해시를_생성한다() {
        let hash1 = hash_password("password123", "salt-abc");
        let hash2 = hash_password("password123", "salt-xyz");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn 다른_비밀번호이면_다른_해시를_생성한다() {
        let hash1 = hash_password("password123", "salt-abc");
        let hash2 = hash_password("password456", "salt-abc");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn 해시는_64자_hex_문자열이다() {
        let hash = hash_password("test", "salt");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn 올바른_비밀번호를_검증한다() {
        let salt = "test-salt";
        let hash = hash_password("mypassword", salt);
        assert!(verify_password("mypassword", salt, &hash));
    }

    #[test]
    fn 잘못된_비밀번호를_거부한다() {
        let salt = "test-salt";
        let hash = hash_password("mypassword", salt);
        assert!(!verify_password("wrongpassword", salt, &hash));
    }
}
