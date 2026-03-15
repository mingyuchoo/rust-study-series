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

    fn get_version() -> String {
        String::from("blog-component v0.3.0 (WASI 0.2 + RBAC)")
    }
}

export!(BlogComponent);
