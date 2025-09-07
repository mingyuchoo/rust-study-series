//! 그래프 생성 모듈
//! - 정규식 기반 간단 엔티티 추출
//! - 엔티티 간 관계(주어-술어-목적어 유사) 추정

use regex::Regex;
use crate::types::{Chunk, Entity, Relation};
use crate::ner::Ner;

/// 간단한 정규식 룰 기반 NER
pub fn extract_entities(chunks: &[Chunk]) -> Vec<Entity> {
    let mut entities = Vec::new();

    let re_date = Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap();
    let re_org = Regex::new(r"(?i)(주식회사|회사|조직|기관|Group|Corp|Inc|LLC|Ltd)").unwrap();
    let re_place = Regex::new(r"(?i)(서울|부산|대구|인천|Korea|Seoul)").unwrap();

    for ch in chunks {
        let text = ch.content.as_str();
        // 날짜
        for m in re_date.find_iter(text) {
            entities.push(Entity { name: m.as_str().to_string(), r#type: "DATE".into() });
        }
        // 조직(간단 키워드)
        if re_org.is_match(text) {
            // 실제 이름 추출은 어려우므로 전체 문장 축약
            let name = truncate(text, 40);
            entities.push(Entity { name, r#type: "ORG".into() });
        }
        // 장소
        for m in re_place.find_iter(text) {
            entities.push(Entity { name: m.as_str().to_string(), r#type: "LOC".into() });
        }
        // 인명(아주 단순: 홍길동 등 3자 한글명 패턴)
        let re_person = Regex::new(r"[가-힣]{3}").unwrap();
        for m in re_person.find_iter(text) {
            let s = m.as_str();
            if s.chars().count() == 3 {
                entities.push(Entity { name: s.to_string(), r#type: "PERSON".into() });
            }
        }
    }

    dedup_entities(entities)
}

/// 외부 NER 구현을 사용하여 엔티티를 추출한다.
pub fn extract_entities_with<N: Ner>(ner: &N, chunks: &[Chunk]) -> Vec<Entity> {
    let mut entities = Vec::new();
    for ch in chunks {
        let text = ch.content.as_str();
        entities.extend(ner.extract(text));
    }
    dedup_entities(entities)
}

/// 간단 관계 추론: 같은 문장 내에 존재하는 엔티티 쌍을 연결
pub fn infer_relations(chunks: &[Chunk], entities: &[Entity]) -> Vec<Relation> {
    let mut rels = Vec::new();
    for ch in chunks {
        let text = ch.content.as_str();
        // 문장 분리
        for sent in text.split(['.', '!', '?', '\n']) {
            let mut found: Vec<&Entity> = Vec::new();
            for e in entities {
                if sent.contains(&e.name) {
                    found.push(e);
                }
            }
            // 모든 쌍을 관계로 연결(간단)
            for i in 0..found.len() {
                for j in (i+1)..found.len() {
                    rels.push(Relation {
                        subject: found[i].name.clone(),
                        predicate: "CO_OCCURS".into(),
                        object: found[j].name.clone(),
                        weight: 1.0,
                    });
                }
            }
        }
    }
    rels
}

fn dedup_entities(mut v: Vec<Entity>) -> Vec<Entity> {
    v.sort_by(|a, b| a.name.cmp(&b.name).then(a.r#type.cmp(&b.r#type)));
    v.dedup_by(|a, b| a.name == b.name && a.r#type == b.r#type);
    v
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}…", &s[..max]) }
}
