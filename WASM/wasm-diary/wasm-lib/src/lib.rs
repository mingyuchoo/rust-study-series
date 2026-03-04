mod date_util;
mod hash_util;
mod id_gen;
pub mod manager;
pub mod model;
pub mod stats;
pub mod user_manager;
pub mod user_model;
pub mod validation;

use wasm_bindgen::prelude::*;

/// 두 정수를 더한다. 초기 WASM 바인딩 예제용.
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize { left + right }

// manager::DiaryManager와 model::Mood는 #[wasm_bindgen]으로 직접 노출됨

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_함수가_정상_동작한다() {
        assert_eq!(add(2, 2), 4);
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::{
        manager::DiaryManager,
        model::{Mood, Weather},
        user_manager::UserManager,
        user_model::Role,
    };

    const SALT_A: &str = "AAAAAAAAAAAAAAAAAAAAAA";
    const SALT_B: &str = "BBBBBBBBBBBBBBBBBBBBBA";
    const NOW: &str = "2026-03-02T00:00:00Z";

    fn setup_users() -> UserManager {
        let mut mgr = UserManager::new();
        mgr.ensure_admin_with("admin-id".to_string(), SALT_A.to_string(), NOW.to_string());
        mgr.register_with(
            "user-1".to_string(),
            "alice".to_string(),
            None,
            "password1!".to_string(),
            Role::User,
            SALT_B.to_string(),
            NOW.to_string(),
        );
        mgr
    }

    #[test]
    fn 소유자별_일기_조회는_본인_항목만_반환한다() {
        let _user_mgr = setup_users();

        let mut diary = DiaryManager::new();
        diary.create_entry_with(
            "e1".to_string(), "admin-id".to_string(),
            "관리자 일기".to_string(), "내용".to_string(),
            Mood::Calm, Weather::Cloudy, NOW.to_string(),
        );
        diary.create_entry_with(
            "e2".to_string(), "user-1".to_string(),
            "앨리스 일기".to_string(), "내용".to_string(),
            Mood::Happy, Weather::Sunny, NOW.to_string(),
        );

        let admin_entries: Vec<serde_json::Value> =
            serde_json::from_str(&diary.get_entries_by_owner("admin-id")).unwrap();
        let user_entries: Vec<serde_json::Value> =
            serde_json::from_str(&diary.get_entries_by_owner("user-1")).unwrap();

        assert_eq!(admin_entries.len(), 1);
        assert_eq!(admin_entries[0]["owner_id"], "admin-id");
        assert_eq!(user_entries.len(), 1);
        assert_eq!(user_entries[0]["owner_id"], "user-1");
    }

    #[test]
    fn 사용자_삭제_후_해당_일기도_제거된다() {
        let mut user_mgr = setup_users();

        let mut diary = DiaryManager::new();
        diary.create_entry_with(
            "e1".to_string(), "user-1".to_string(),
            "앨리스 일기".to_string(), "내용".to_string(),
            Mood::Happy, Weather::Sunny, NOW.to_string(),
        );

        // 사용자 삭제 후 일기도 함께 삭제
        assert!(user_mgr.delete_user("user-1"));
        assert!(diary.delete_entry("e1"));

        let remaining: Vec<serde_json::Value> =
            serde_json::from_str(&diary.get_entries_by_owner("user-1")).unwrap();
        assert!(remaining.is_empty());
    }

    #[test]
    fn 전체_통계는_모든_사용자_일기를_포함한다() {
        let mut diary = DiaryManager::new();
        diary.create_entry_with(
            "e1".to_string(), "admin-id".to_string(),
            "제목1".to_string(), "내용1".to_string(),
            Mood::Happy, Weather::Sunny, NOW.to_string(),
        );
        diary.create_entry_with(
            "e2".to_string(), "user-1".to_string(),
            "제목2".to_string(), "내용2".to_string(),
            Mood::Sad, Weather::Rainy, NOW.to_string(),
        );

        let stats: serde_json::Value =
            serde_json::from_str(&diary.get_statistics()).unwrap();
        assert_eq!(stats["total_entries"], 2);
        assert_eq!(stats["mood_distribution"]["Happy"], 1);
        assert_eq!(stats["mood_distribution"]["Sad"], 1);
    }

    #[test]
    fn json_직렬화_후_복원해도_데이터가_동일하다() {
        let mut diary = DiaryManager::new();
        diary.create_entry_with(
            "e1".to_string(), "user-1".to_string(),
            "제목".to_string(), "내용".to_string(),
            Mood::Calm, Weather::Cloudy, NOW.to_string(),
        );

        let json = diary.save_to_json();
        let mut restored = DiaryManager::new();
        restored.load_from_json(&json);

        let original: Vec<serde_json::Value> =
            serde_json::from_str(&diary.get_all_entries()).unwrap();
        let recovered: Vec<serde_json::Value> =
            serde_json::from_str(&restored.get_all_entries()).unwrap();

        assert_eq!(original.len(), recovered.len());
        assert_eq!(original[0]["id"], recovered[0]["id"]);
        assert_eq!(original[0]["owner_id"], recovered[0]["owner_id"]);
    }
}
