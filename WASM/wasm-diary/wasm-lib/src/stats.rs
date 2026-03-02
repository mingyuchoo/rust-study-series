use crate::model::DiaryEntry;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, PartialEq)]
pub struct DiaryStatistics {
    pub total_entries: usize,
    pub total_characters: usize,
    pub total_words: usize,
    pub mood_distribution: HashMap<String, usize>,
    pub weather_distribution: HashMap<String, usize>,
}

/// 일기 목록에서 통계를 계산한다.
/// 어절 수는 공백 기준 분리로 산출한다 (한국어 형태소 분석 없이 실용적 수준).
pub fn calculate_statistics(entries: &[DiaryEntry]) -> DiaryStatistics {
    let total_entries = entries.len();
    let mut total_characters = 0usize;
    let mut total_words = 0usize;
    let mut mood_counts: HashMap<String, usize> = HashMap::new();
    let mut weather_counts: HashMap<String, usize> = HashMap::new();

    for entry in entries {
        total_characters += entry.content.chars().count();
        total_words += entry.content.split_whitespace().count();

        let mood_key = format!("{:?}", entry.mood);
        *mood_counts.entry(mood_key).or_insert(0) += 1;

        let weather_key = format!("{:?}", entry.weather);
        *weather_counts.entry(weather_key).or_insert(0) += 1;
    }

    DiaryStatistics {
        total_entries,
        total_characters,
        total_words,
        mood_distribution: mood_counts,
        weather_distribution: weather_counts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Mood, Weather};

    fn make_entry(content: &str, mood: Mood, weather: Weather) -> DiaryEntry {
        DiaryEntry {
            id: "test".to_string(),
            owner_id: "owner-1".to_string(),
            title: "테스트".to_string(),
            content: content.to_string(),
            mood,
            weather,
            created_at: "2026-03-02T00:00:00Z".to_string(),
            updated_at: "2026-03-02T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn 빈_목록이면_모든_통계가_0이다() {
        let stats = calculate_statistics(&[]);
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.total_characters, 0);
        assert_eq!(stats.total_words, 0);
        assert!(stats.mood_distribution.is_empty());
    }

    #[test]
    fn 일기_하나의_통계를_계산한다() {
        let entries = vec![make_entry("오늘은 좋은 하루였다", Mood::Happy, Weather::Sunny)];
        let stats = calculate_statistics(&entries);

        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.total_words, 3); // "오늘은", "좋은", "하루였다"
        assert_eq!(stats.mood_distribution.get("Happy"), Some(&1));
        assert_eq!(stats.weather_distribution.get("Sunny"), Some(&1));
    }

    #[test]
    fn 여러_일기의_감정_분포를_계산한다() {
        let entries = vec![
            make_entry("내용1", Mood::Happy, Weather::Sunny),
            make_entry("내용2", Mood::Happy, Weather::Rainy),
            make_entry("내용3", Mood::Sad, Weather::Rainy),
        ];
        let stats = calculate_statistics(&entries);

        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.mood_distribution.get("Happy"), Some(&2));
        assert_eq!(stats.mood_distribution.get("Sad"), Some(&1));
        assert_eq!(stats.weather_distribution.get("Sunny"), Some(&1));
        assert_eq!(stats.weather_distribution.get("Rainy"), Some(&2));
    }

    #[test]
    fn 한국어_글자수를_정확히_센다() {
        let entries = vec![make_entry("가나다라마", Mood::Calm, Weather::Cloudy)];
        let stats = calculate_statistics(&entries);
        assert_eq!(stats.total_characters, 5);
    }
}
