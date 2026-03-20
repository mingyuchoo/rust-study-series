//! NER 모듈
//! - Trait 기반으로 교체 가능하도록 설계
//! - 기본 구현: 정규식 기반 간단 NER (RegexNER)

use regex::Regex;

use crate::types::Entity;

/// NER 공통 Trait
pub trait Ner {
    /// 입력 텍스트에서 엔티티를 추출한다.
    fn extract(&self, text: &str) -> Vec<Entity>;
}

/// 간단한 정규식 기반 NER 구현
pub struct RegexNer {
    re_date: Regex,
    re_org: Regex,
    re_place: Regex,
    re_person: Regex,
}

impl Default for RegexNer {
    fn default() -> Self {
        Self {
            re_date: Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap(),
            re_org: Regex::new(r"(?i)(주식회사|회사|조직|기관|Group|Corp|Inc|LLC|Ltd)").unwrap(),
            re_place: Regex::new(r"(?i)(서울|부산|대구|인천|Korea|Seoul)").unwrap(),
            re_person: Regex::new(r"[가-힣]{3}").unwrap(),
        }
    }
}

impl Ner for RegexNer {
    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        for m in self.re_date.find_iter(text) {
            entities.push(Entity {
                name: m.as_str().to_string(),
                r#type: "DATE".into(),
            });
        }
        if self.re_org.is_match(text) {
            // 문장에서 조직명이 명시되지 않을 수 있어 간단히 문장 일부를 사용
            let name = truncate(text, 40);
            entities.push(Entity { name, r#type: "ORG".into() });
        }
        for m in self.re_place.find_iter(text) {
            entities.push(Entity {
                name: m.as_str().to_string(),
                r#type: "LOC".into(),
            });
        }
        for m in self.re_person.find_iter(text) {
            let s = m.as_str();
            if s.chars().count() == 3 {
                entities.push(Entity {
                    name: s.to_string(),
                    r#type: "PERSON".into(),
                });
            }
        }
        dedup_entities(entities)
    }
}

/// 멀티바이트(UTF-8) 안전 잘라내기: 바이트 인덱스가 아닌 문자 기준으로 자른다.
fn truncate(s: &str, max: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max {
        return s.to_string();
    }
    let mut out = String::with_capacity(max + 3);
    out.extend(s.chars().take(max));
    out.push('…');
    out
}

fn dedup_entities(mut v: Vec<Entity>) -> Vec<Entity> {
    v.sort_by(|a, b| a.name.cmp(&b.name).then(a.r#type.cmp(&b.r#type)));
    v.dedup_by(|a, b| a.name == b.name && a.r#type == b.r#type);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ner() -> RegexNer {
        RegexNer::default()
    }

    fn find_entity<'a>(entities: &'a [Entity], name: &str, etype: &str) -> bool {
        entities.iter().any(|e| e.name == name && e.r#type == etype)
    }

    #[test]
    fn test_extract_date() {
        let entities = ner().extract("프로젝트는 2024-01-15 에 시작했습니다.");
        assert!(find_entity(&entities, "2024-01-15", "DATE"));
    }

    #[test]
    fn test_extract_multiple_dates() {
        let entities = ner().extract("2024-01-01 부터 2024-12-31 까지 진행합니다.");
        assert!(find_entity(&entities, "2024-01-01", "DATE"));
        assert!(find_entity(&entities, "2024-12-31", "DATE"));
    }

    #[test]
    fn test_extract_org() {
        let entities = ner().extract("주식회사 샘플이 참여했습니다.");
        assert!(entities.iter().any(|e| e.r#type == "ORG"));
    }

    #[test]
    fn test_extract_place() {
        let entities = ner().extract("장소는 서울입니다.");
        assert!(find_entity(&entities, "서울", "LOC"));
    }

    #[test]
    fn test_extract_person() {
        let entities = ner().extract("홍길동이 프로젝트를 이끌었습니다.");
        assert!(find_entity(&entities, "홍길동", "PERSON"));
    }

    #[test]
    fn test_no_entities_in_plain_text() {
        let entities = ner().extract("hello world 123");
        assert!(entities.is_empty());
    }

    #[test]
    fn test_dedup_removes_duplicates() {
        let entities = ner().extract("서울에서 서울로 이동합니다.");
        let seoul_count = entities.iter().filter(|e| e.name == "서울").count();
        assert_eq!(seoul_count, 1);
    }

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate("짧은", 10), "짧은");
    }

    #[test]
    fn test_truncate_long_string() {
        let result = truncate("이것은 매우 긴 문자열입니다 테스트용", 5);
        assert_eq!(result.chars().count(), 6); // 5자 + '…'
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_truncate_multibyte_safe() {
        // UTF-8 멀티바이트 문자에서도 패닉 없이 동작해야 한다
        let result = truncate("가나다라마바사아자차", 3);
        assert_eq!(result, "가나다…");
    }
}
