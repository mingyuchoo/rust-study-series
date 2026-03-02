use crate::{date_util,
            id_gen,
            model::{DiaryEntry,
                    Mood,
                    Weather},
            stats,
            validation};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DiaryManager {
    entries: Vec<DiaryEntry>,
}

// 내부 헬퍼 (테스트 시 ID/타임스탬프 주입용)
impl DiaryManager {
    fn create_entry_with(&mut self, id: String, title: String, content: String, mood: Mood, weather: Weather, now: String) -> String {
        let entry = DiaryEntry {
            id,
            title: title.trim().to_string(),
            content: content.trim().to_string(),
            mood,
            weather,
            created_at: now.clone(),
            updated_at: now,
        };
        let json = serde_json::to_string(&entry).unwrap_or_default();
        self.entries.push(entry);
        json
    }

    fn update_entry_with(&mut self, id: &str, title: &str, content: &str, mood: Mood, weather: Weather, now: String) -> String {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.title = title.trim().to_string();
            entry.content = content.trim().to_string();
            entry.mood = mood;
            entry.weather = weather;
            entry.updated_at = now;
            serde_json::to_string(entry).unwrap_or_default()
        } else {
            String::new()
        }
    }
}

#[wasm_bindgen]
impl DiaryManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// localStorage에서 읽어온 JSON 문자열로 상태를 복원한다.
    pub fn load_from_json(&mut self, json: &str) {
        if let Ok(entries) = serde_json::from_str::<Vec<DiaryEntry>>(json) {
            self.entries = entries;
        }
    }

    /// 현재 상태를 JSON 문자열로 직렬화한다 (localStorage 저장용).
    pub fn save_to_json(&self) -> String { serde_json::to_string(&self.entries).unwrap_or_else(|_| "[]".to_string()) }

    /// 새 일기를 생성하고 생성된 항목의 JSON을 반환한다.
    pub fn create_entry(&mut self, title: &str, content: &str, mood: Mood, weather: Weather) -> String {
        let id = id_gen::generate_id();
        let now = date_util::now_iso();
        self.create_entry_with(id, title.to_string(), content.to_string(), mood, weather, now)
    }

    /// 일기를 수정하고 수정된 항목의 JSON을 반환한다. 존재하지 않으면 빈
    /// 문자열.
    pub fn update_entry(&mut self, id: &str, title: &str, content: &str, mood: Mood, weather: Weather) -> String {
        let now = date_util::now_iso();
        self.update_entry_with(id, title, content, mood, weather, now)
    }

    /// 일기를 삭제한다. 성공 시 true.
    pub fn delete_entry(&mut self, id: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }

    /// 단건 조회. 존재하지 않으면 빈 문자열.
    pub fn get_entry(&self, id: &str) -> String {
        self.entries
            .iter()
            .find(|e| e.id == id)
            .map(|e| serde_json::to_string(e).unwrap_or_default())
            .unwrap_or_default()
    }

    /// 전체 목록 JSON (최신순 정렬).
    pub fn get_all_entries(&self) -> String {
        let mut sorted = self.entries.clone();
        sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        serde_json::to_string(&sorted).unwrap_or_else(|_| "[]".to_string())
    }

    /// 제목 또는 내용에 키워드가 포함된 일기를 검색한다 (대소문자 무시).
    pub fn search_by_keyword(&self, keyword: &str) -> String {
        let kw = keyword.to_lowercase();
        let results: Vec<&DiaryEntry> = self
            .entries
            .iter()
            .filter(|e| e.title.to_lowercase().contains(&kw) || e.content.to_lowercase().contains(&kw))
            .collect();
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }

    /// 특정 감정의 일기만 필터링한다.
    pub fn filter_by_mood(&self, mood: Mood) -> String {
        let results: Vec<&DiaryEntry> = self.entries.iter().filter(|e| e.mood == mood).collect();
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }

    /// 특정 날씨의 일기만 필터링한다.
    pub fn filter_by_weather(&self, weather: Weather) -> String {
        let results: Vec<&DiaryEntry> = self.entries.iter().filter(|e| e.weather == weather).collect();
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }

    /// 날짜 범위(from ~ to)로 일기를 필터링한다.
    pub fn filter_by_date_range(&self, from: &str, to: &str) -> String {
        let results: Vec<&DiaryEntry> = self.entries.iter().filter(|e| date_util::is_in_range(&e.created_at, from, to)).collect();
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }

    /// 통계 JSON을 반환한다.
    pub fn get_statistics(&self) -> String { serde_json::to_string(&stats::calculate_statistics(&self.entries)).unwrap_or_else(|_| "{}".to_string()) }

    /// 제목과 내용의 유효성을 검사한다.
    pub fn validate(title: &str, content: &str) -> String {
        serde_json::to_string(&validation::validate_entry(title, content)).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manager_with_entries() -> DiaryManager {
        let mut mgr = DiaryManager::new();
        let now = "2026-03-02T10:00:00Z".to_string();
        mgr.create_entry_with(
            "id-1".to_string(),
            "행복한 날".to_string(),
            "오늘은 정말 좋은 날이었다".to_string(),
            Mood::Happy,
            Weather::Sunny,
            now.clone(),
        );
        mgr.create_entry_with(
            "id-2".to_string(),
            "슬픈 하루".to_string(),
            "비가 와서 우울했다".to_string(),
            Mood::Sad,
            Weather::Rainy,
            "2026-03-01T10:00:00Z".to_string(),
        );
        mgr.create_entry_with(
            "id-3".to_string(),
            "평온한 오후".to_string(),
            "차를 마시며 휴식했다".to_string(),
            Mood::Calm,
            Weather::Cloudy,
            now,
        );
        mgr
    }

    #[test]
    fn 새_매니저는_비어있다() {
        let mgr = DiaryManager::new();
        let all: Vec<serde_json::Value> = serde_json::from_str(&mgr.get_all_entries()).unwrap();
        assert!(all.is_empty());
    }

    #[test]
    fn 일기를_생성하고_조회한다() {
        let mut mgr = DiaryManager::new();
        let json = mgr.create_entry_with(
            "test-1".to_string(),
            "제목".to_string(),
            "내용".to_string(),
            Mood::Happy,
            Weather::Sunny,
            "2026-03-02T00:00:00Z".to_string(),
        );

        let entry: DiaryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry.id, "test-1");
        assert_eq!(entry.title, "제목");
        assert_eq!(entry.weather, Weather::Sunny);

        let fetched = mgr.get_entry("test-1");
        assert!(!fetched.is_empty());
    }

    #[test]
    fn 존재하지_않는_일기를_조회하면_빈_문자열이다() {
        let mgr = DiaryManager::new();
        assert!(mgr.get_entry("nonexistent").is_empty());
    }

    #[test]
    fn 일기를_수정한다() {
        let mut mgr = make_manager_with_entries();
        let updated = mgr.update_entry_with("id-1", "수정된 제목", "수정된 내용", Mood::Excited, Weather::Windy, "2026-03-02T12:00:00Z".to_string());

        let entry: DiaryEntry = serde_json::from_str(&updated).unwrap();
        assert_eq!(entry.title, "수정된 제목");
        assert_eq!(entry.mood, Mood::Excited);
        assert_eq!(entry.weather, Weather::Windy);
        assert_eq!(entry.updated_at, "2026-03-02T12:00:00Z");
    }

    #[test]
    fn 존재하지_않는_일기를_수정하면_빈_문자열이다() {
        let mut mgr = DiaryManager::new();
        assert!(
            mgr.update_entry_with("nonexistent", "t", "c", Mood::Happy, Weather::Sunny, "2026-03-02T00:00:00Z".to_string())
                .is_empty()
        );
    }

    #[test]
    fn 일기를_삭제한다() {
        let mut mgr = make_manager_with_entries();
        assert!(mgr.delete_entry("id-1"));
        assert!(mgr.get_entry("id-1").is_empty());
    }

    #[test]
    fn 존재하지_않는_일기를_삭제하면_false이다() {
        let mut mgr = DiaryManager::new();
        assert!(!mgr.delete_entry("nonexistent"));
    }

    #[test]
    fn 키워드로_검색한다() {
        let mgr = make_manager_with_entries();
        let results: Vec<DiaryEntry> = serde_json::from_str(&mgr.search_by_keyword("좋은")).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "id-1");
    }

    #[test]
    fn 키워드_검색은_대소문자를_무시한다() {
        let mut mgr = DiaryManager::new();
        mgr.create_entry_with(
            "id-1".to_string(),
            "Hello World".to_string(),
            "content".to_string(),
            Mood::Happy,
            Weather::Sunny,
            "2026-03-02T00:00:00Z".to_string(),
        );
        let results: Vec<DiaryEntry> = serde_json::from_str(&mgr.search_by_keyword("hello")).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn 날씨로_필터링한다() {
        let mgr = make_manager_with_entries();
        let results: Vec<DiaryEntry> = serde_json::from_str(&mgr.filter_by_weather(Weather::Rainy)).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "id-2");
    }

    #[test]
    fn 감정으로_필터링한다() {
        let mgr = make_manager_with_entries();
        let results: Vec<DiaryEntry> = serde_json::from_str(&mgr.filter_by_mood(Mood::Happy)).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "id-1");
    }

    #[test]
    fn 날짜_범위로_필터링한다() {
        let mgr = make_manager_with_entries();
        let results: Vec<DiaryEntry> = serde_json::from_str(&mgr.filter_by_date_range("2026-03-02", "2026-03-02")).unwrap();
        assert_eq!(results.len(), 2); // id-1, id-3
    }

    #[test]
    fn json으로_저장하고_복원한다() {
        let mgr = make_manager_with_entries();
        let json = mgr.save_to_json();

        let mut mgr2 = DiaryManager::new();
        mgr2.load_from_json(&json);

        let all_original: Vec<DiaryEntry> = serde_json::from_str(&mgr.get_all_entries()).unwrap();
        let all_restored: Vec<DiaryEntry> = serde_json::from_str(&mgr2.get_all_entries()).unwrap();
        assert_eq!(all_original.len(), all_restored.len());
    }

    #[test]
    fn 잘못된_json을_로드하면_기존_상태를_유지한다() {
        let mut mgr = make_manager_with_entries();
        mgr.load_from_json("invalid json{{{");
        // 로드 실패 시 기존 entries가 그대로여야 하지만, 현재 구현은 entries를 덮어쓰지
        // 않음 (serde_json::from_str이 Err이면 아무것도 안 함)
        let all: Vec<DiaryEntry> = serde_json::from_str(&mgr.get_all_entries()).unwrap();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn 통계를_계산한다() {
        let mgr = make_manager_with_entries();
        let stats_json = mgr.get_statistics();
        let stats: serde_json::Value = serde_json::from_str(&stats_json).unwrap();
        assert_eq!(stats["total_entries"], 3);
    }

    #[test]
    fn 유효성_검사를_수행한다() {
        let result = DiaryManager::validate("", "내용");
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["valid"], false);
    }

    #[test]
    fn 전체_목록은_최신순으로_정렬된다() {
        let mgr = make_manager_with_entries();
        let all: Vec<DiaryEntry> = serde_json::from_str(&mgr.get_all_entries()).unwrap();
        // id-1, id-3는 2026-03-02, id-2는 2026-03-01이므로 id-2가 마지막
        assert_eq!(all.last().unwrap().id, "id-2");
    }
}
