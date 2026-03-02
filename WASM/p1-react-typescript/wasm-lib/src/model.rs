use serde::{Deserialize,
            Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mood {
    Happy,
    Sad,
    Angry,
    Anxious,
    Calm,
    Excited,
    Tired,
    Grateful,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiaryEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub mood: Mood,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 일기_항목을_json으로_직렬화한다() {
        let entry = DiaryEntry {
            id: "test-1".to_string(),
            title: "오늘의 일기".to_string(),
            content: "좋은 하루였다".to_string(),
            mood: Mood::Happy,
            created_at: "2026-03-02T10:00:00Z".to_string(),
            updated_at: "2026-03-02T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: DiaryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn 감정_enum을_json으로_직렬화한다() {
        let mood = Mood::Grateful;
        let json = serde_json::to_string(&mood).unwrap();
        assert_eq!(json, "\"Grateful\"");

        let deserialized: Mood = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Mood::Grateful);
    }

    #[test]
    fn 일기_목록을_json으로_직렬화한다() {
        let entries = vec![
            DiaryEntry {
                id: "1".to_string(),
                title: "첫째 날".to_string(),
                content: "내용1".to_string(),
                mood: Mood::Happy,
                created_at: "2026-03-01T00:00:00Z".to_string(),
                updated_at: "2026-03-01T00:00:00Z".to_string(),
            },
            DiaryEntry {
                id: "2".to_string(),
                title: "둘째 날".to_string(),
                content: "내용2".to_string(),
                mood: Mood::Calm,
                created_at: "2026-03-02T00:00:00Z".to_string(),
                updated_at: "2026-03-02T00:00:00Z".to_string(),
            },
        ];

        let json = serde_json::to_string(&entries).unwrap();
        let deserialized: Vec<DiaryEntry> = serde_json::from_str(&json).unwrap();
        assert_eq!(entries, deserialized);
    }
}
