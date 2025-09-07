//! 한국어 NER 모듈
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
