use serde::Serialize;

const MAX_TITLE_LENGTH: usize = 100;
const MAX_CONTENT_LENGTH: usize = 5000;

/// 유효성 검사 결과. `valid`가 false이면 `errors`에 세부 내용이 담긴다.
#[derive(Debug, Serialize, PartialEq)]
pub struct ValidationResult {
    /// 모든 검사를 통과하면 true
    pub valid: bool,
    /// 실패한 검사 목록
    pub errors: Vec<ValidationError>,
}

/// 개별 유효성 검사 오류.
#[derive(Debug, Serialize, PartialEq)]
pub struct ValidationError {
    /// 오류가 발생한 필드명 (예: "title", "content")
    pub field: String,
    /// 사용자에게 표시할 오류 메시지
    pub message: String,
}

pub fn validate_entry(title: &str, content: &str) -> ValidationResult {
    let mut errors = Vec::new();

    let trimmed_title = title.trim();
    if trimmed_title.is_empty() {
        errors.push(ValidationError {
            field: "title".to_string(),
            message: "제목을 입력해주세요.".to_string(),
        });
    } else if trimmed_title.chars().count() > MAX_TITLE_LENGTH {
        errors.push(ValidationError {
            field: "title".to_string(),
            message: format!("제목은 {MAX_TITLE_LENGTH}자 이내로 입력해주세요."),
        });
    }

    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        errors.push(ValidationError {
            field: "content".to_string(),
            message: "내용을 입력해주세요.".to_string(),
        });
    } else if trimmed_content.chars().count() > MAX_CONTENT_LENGTH {
        errors.push(ValidationError {
            field: "content".to_string(),
            message: format!("내용은 {MAX_CONTENT_LENGTH}자 이내로 입력해주세요."),
        });
    }

    ValidationResult {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 빈_제목이면_유효성_오류를_반환한다() {
        let result = validate_entry("", "내용이 있음");
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].field, "title");
    }

    #[test]
    fn 공백만_있는_제목이면_유효성_오류를_반환한다() {
        let result = validate_entry("   ", "내용이 있음");
        assert!(!result.valid);
        assert_eq!(result.errors[0].field, "title");
    }

    #[test]
    fn 빈_내용이면_유효성_오류를_반환한다() {
        let result = validate_entry("제목", "");
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].field, "content");
    }

    #[test]
    fn 제목과_내용_모두_비면_오류_두_개를_반환한다() {
        let result = validate_entry("", "");
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 2);
    }

    #[test]
    fn 유효한_입력이면_통과한다() {
        let result = validate_entry("오늘의 일기", "좋은 하루였다.");
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn 제목이_100자를_초과하면_오류를_반환한다() {
        let long_title: String = "가".repeat(101);
        let result = validate_entry(&long_title, "내용");
        assert!(!result.valid);
        assert_eq!(result.errors[0].field, "title");
    }

    #[test]
    fn 내용이_5000자를_초과하면_오류를_반환한다() {
        let long_content: String = "나".repeat(5001);
        let result = validate_entry("제목", &long_content);
        assert!(!result.valid);
        assert_eq!(result.errors[0].field, "content");
    }
}
