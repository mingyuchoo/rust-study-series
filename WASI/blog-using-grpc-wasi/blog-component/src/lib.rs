wit_bindgen::generate!({
    world: "blog-world",
    path: "wit",
});

struct BlogComponent;

impl exports::component::blog::blogger::Guest for BlogComponent {
    fn validate_title(title: String) -> String {
        if title.trim().is_empty() {
            return "제목을 입력해주세요.".to_string();
        }
        if title.len() > 200 {
            return "제목은 200자를 초과할 수 없습니다.".to_string();
        }
        String::new()
    }

    fn validate_content(content: String) -> String {
        if content.trim().is_empty() {
            return "내용을 입력해주세요.".to_string();
        }
        if content.len() > 50_000 {
            return "내용은 50,000자를 초과할 수 없습니다.".to_string();
        }
        String::new()
    }

    fn validate_comment(content: String) -> String {
        if content.trim().is_empty() {
            return "댓글 내용을 입력해주세요.".to_string();
        }
        if content.len() > 5_000 {
            return "댓글은 5,000자를 초과할 수 없습니다.".to_string();
        }
        String::new()
    }

    fn validate_role(role: String) -> String {
        match role.as_str() {
            "admin" | "user" => String::new(),
            _ => "역할은 'admin' 또는 'user'만 가능합니다.".to_string(),
        }
    }

    fn validate_visibility(visibility: String) -> String {
        match visibility.as_str() {
            "public" | "private" => String::new(),
            _ => "공개범위는 'public' 또는 'private'만 가능합니다.".to_string(),
        }
    }

    fn validate_email(email: String) -> String {
        let email = email.trim();
        if email.is_empty() {
            return "이메일을 입력해주세요.".to_string();
        }
        if email.len() > 254 {
            return "이메일은 254자를 초과할 수 없습니다.".to_string();
        }

        let parts: Vec<&str> = email.splitn(2, '@').collect();
        if parts.len() != 2 {
            return "이메일 형식이 올바르지 않습니다. '@'가 필요합니다.".to_string();
        }

        let local = parts[0];
        let domain = parts[1];

        if local.is_empty() {
            return "이메일의 로컬 부분이 비어있습니다.".to_string();
        }
        if domain.is_empty() || !domain.contains('.') {
            return "이메일 도메인 형식이 올바르지 않습니다.".to_string();
        }

        let domain_parts: Vec<&str> = domain.split('.').collect();
        if domain_parts.iter().any(|p| p.is_empty()) {
            return "이메일 도메인에 빈 부분이 있습니다.".to_string();
        }

        let tld = domain_parts.last().unwrap();
        if tld.len() < 2 {
            return "이메일 도메인의 최상위 도메인이 너무 짧습니다.".to_string();
        }

        String::new()
    }

    fn validate_username(username: String) -> String {
        let username = username.trim();
        if username.is_empty() {
            return "사용자명을 입력해주세요.".to_string();
        }
        if username.len() < 2 {
            return "사용자명은 2자 이상이어야 합니다.".to_string();
        }
        if username.len() > 30 {
            return "사용자명은 30자를 초과할 수 없습니다.".to_string();
        }

        // 영문, 숫자, 밑줄, 하이픈만 허용
        if !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return "사용자명은 영문, 숫자, 밑줄(_), 하이픈(-)만 사용할 수 있습니다.".to_string();
        }

        // 첫 글자는 영문이어야 함
        if !username.starts_with(|c: char| c.is_ascii_alphabetic()) {
            return "사용자명은 영문자로 시작해야 합니다.".to_string();
        }

        // 예약어 차단
        let reserved = [
            "admin",
            "root",
            "system",
            "superuser",
            "administrator",
            "moderator",
            "mod",
            "null",
            "undefined",
            "anonymous",
        ];
        if reserved.contains(&username.to_ascii_lowercase().as_str()) {
            return "사용할 수 없는 사용자명입니다.".to_string();
        }

        String::new()
    }

    fn validate_password_strength(password: String) -> String {
        if password.len() < 8 {
            return "비밀번호는 8자 이상이어야 합니다.".to_string();
        }
        if password.len() > 128 {
            return "비밀번호는 128자를 초과할 수 없습니다.".to_string();
        }

        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| c.is_ascii_punctuation());

        let kinds = [has_upper, has_lower, has_digit, has_special]
            .iter()
            .filter(|&&v| v)
            .count();

        if kinds < 2 {
            return "비밀번호는 대문자, 소문자, 숫자, 특수문자 중 2가지 이상을 포함해야 합니다."
                .to_string();
        }

        String::new()
    }

    fn sanitize_content(content: String) -> String {
        replace_case_insensitive(
            &replace_case_insensitive(
                &replace_case_insensitive(
                    &replace_case_insensitive(
                        &replace_case_insensitive(
                            &replace_case_insensitive(
                                &replace_case_insensitive(
                                    &replace_case_insensitive(
                                        &replace_case_insensitive(
                                            &content,
                                            "<script",
                                            "&lt;script",
                                        ),
                                        "</script",
                                        "&lt;/script",
                                    ),
                                    "javascript:",
                                    "",
                                ),
                                "onerror=",
                                "",
                            ),
                            "onload=",
                            "",
                        ),
                        "onclick=",
                        "",
                    ),
                    "onmouseover=",
                    "",
                ),
                "onfocus=",
                "",
            ),
            "oninput=",
            "",
        )
    }

    fn validate_bio(bio: String) -> String {
        if bio.len() > 500 {
            return "자기소개는 500자를 초과할 수 없습니다.".to_string();
        }
        String::new()
    }

    fn validate_website(website: String) -> String {
        let website = website.trim();
        if website.is_empty() {
            return String::new();
        }
        if website.len() > 200 {
            return "웹사이트 주소는 200자를 초과할 수 없습니다.".to_string();
        }
        if !website.starts_with("https://") && !website.starts_with("http://") {
            return "웹사이트 주소는 http:// 또는 https://로 시작해야 합니다.".to_string();
        }
        if !website[8..].contains('.') {
            return "웹사이트 주소 형식이 올바르지 않습니다.".to_string();
        }
        String::new()
    }

    fn validate_theme(theme: String) -> String {
        match theme.as_str() {
            "dark" | "light" => String::new(),
            _ => "테마는 'dark' 또는 'light'만 가능합니다.".to_string(),
        }
    }

    fn get_version() -> String {
        String::from("blog-component v0.5.0 (WASI 0.2 + RBAC + Full Validation)")
    }
}

/// 대소문자를 구분하지 않고 문자열을 치환합니다.
fn replace_case_insensitive(input: &str, pattern: &str, replacement: &str) -> String {
    let lower_input = input.to_ascii_lowercase();
    let lower_pattern = pattern.to_ascii_lowercase();
    let mut result = String::with_capacity(input.len());
    let mut start = 0;

    while let Some(pos) = lower_input[start..].find(&lower_pattern) {
        result.push_str(&input[start..start + pos]);
        result.push_str(replacement);
        start += pos + pattern.len();
    }
    result.push_str(&input[start..]);
    result
}

export!(BlogComponent);
