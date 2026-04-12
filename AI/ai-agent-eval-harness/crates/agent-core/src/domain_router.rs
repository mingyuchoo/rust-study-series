// =============================================================================
// @trace SPEC-020
// @trace SPEC-022
// @trace PRD: PRD-020, PRD-022
// @trace FR: PRD-020/FR-3, PRD-022/FR-3
// @trace file-type: impl
// =============================================================================
//
// 도메인 pre-filter. `task_description` 의 키워드를 집계해 가장 관련이 높은
// 상위 K 개 도메인을 선택한다. 임베딩 기반 라우터를 붙이기 전 1차 근사치이며,
// 매칭이 전혀 없으면 빈 Vec 를 반환하여 상위 호출자가 "모든 도메인 유지" 로
// 폴백하도록 한다.
//
// SPEC-022: 키워드는 이제 SQLite `domain_keywords` 테이블에 저장된다. 본 모듈은
// `RwLock<Option<HashMap<String, Vec<String>>>>` 캐시를 들고 있으며, 첫 호출 시
// `try_installed_store()` → `list_all_domain_keywords()` 로 채운다. CRUD
// 핸들러가 도메인/키워드를 변경하면 `invalidate_cache()` 를 호출해 다음 select
// 시 재로드.
//
// 단위 테스트나 store 가 install 되지 않은 환경에서는 `default_keywords()` 가
// 폴백으로 사용된다.

use std::{collections::HashMap,
          sync::RwLock};

/// 부트스트랩 키워드. v5 마이그레이션 시 이 값들이 DB 에 시드되며, store 가
/// 없는 환경(테스트 등)에서는 폴백으로 사용된다.
///
/// 새 부트스트랩 도메인을 Rust 코드에 추가할 때 이 함수도 같이 갱신.
pub fn default_keywords() -> Vec<(String, Vec<String>)> {
    vec![
        (
            "financial".to_string(),
            vec![
                "이자",
                "이율",
                "금리",
                "복리",
                "단리",
                "원금",
                "대출",
                "예금",
                "적금",
                "적립",
                "계좌",
                "거래",
                "송금",
                "interest",
                "loan",
                "deposit",
                "account",
                "transaction",
                "premium",
                "compound",
                "principal",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        ),
        (
            "customer_service".to_string(),
            vec![
                "환불",
                "반품",
                "교환",
                "배송",
                "주문",
                "고객",
                "불만",
                "클레임",
                "상담",
                "문의",
                "에스컬레이션",
                "refund",
                "return",
                "shipping",
                "order",
                "customer",
                "complaint",
                "inquiry",
                "escalate",
                "ticket",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        ),
    ]
}

/// 부트스트랩 키워드를 `(domain, keyword)` 페어 리스트로 평면화. SqliteStore
/// 의 `seed_domain_keywords()` 가 받는 형태.
pub fn default_keywords_flat() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (d, kws) in default_keywords() {
        for k in kws {
            out.push((d.clone(), k));
        }
    }
    out
}

/// 캐시. None 이면 다음 호출 시 lazy load.
static KEYWORD_CACHE: RwLock<Option<HashMap<String, Vec<String>>>> = RwLock::new(None);

/// 캐시를 비운다. 도메인/키워드 CRUD 직후 호출.
///
/// @trace SPEC: SPEC-022
/// @trace FR: PRD-022/FR-3
pub fn invalidate_cache() {
    if let Ok(mut guard) = KEYWORD_CACHE.write() {
        *guard = None;
    }
}

/// 외부에서 캐시를 직접 주입(테스트·기동 시점 prime). 일반 코드는 lazy load 에
/// 의존.
pub fn prime_cache(map: HashMap<String, Vec<String>>) {
    if let Ok(mut guard) = KEYWORD_CACHE.write() {
        *guard = Some(map);
    }
}

/// DB → HashMap 로드. 실패하면 default 로 폴백.
fn load_into_cache() -> HashMap<String, Vec<String>> {
    if let Some(store) = data_scenarios::loader::try_installed_store() {
        // SqliteStore 메서드는 async 이므로 동기 컨텍스트에서 block_on.
        let result = match tokio::runtime::Handle::try_current() {
            | Ok(handle) => {
                let store = store.clone();
                tokio::task::block_in_place(|| handle.block_on(async move { store.list_all_domain_keywords().await }))
            },
            | Err(_) => tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()
                .map(|rt| rt.block_on(async move { store.list_all_domain_keywords().await }))
                .unwrap_or_else(|| Ok(HashMap::new())),
        };
        if let Ok(map) = result {
            if !map.is_empty() {
                return map;
            }
        }
    }
    // 폴백: 컴파일 시 부트스트랩 값
    default_keywords().into_iter().collect()
}

/// task 설명에서 각 도메인의 키워드 매칭 건수를 센 뒤 상위 `top_k` 개만
/// 반환한다. 동률은 도메인 이름 정렬(stable). 매칭 0건이면 빈 Vec.
pub fn select_domains(task_description: &str, top_k: usize) -> Vec<String> {
    if top_k == 0 {
        return Vec::new();
    }
    // 캐시 조회 (read lock)
    let cached: Option<HashMap<String, Vec<String>>> = KEYWORD_CACHE.read().ok().and_then(|g| g.clone());
    let map = match cached {
        | Some(m) => m,
        | None => {
            // lazy load + write lock
            let loaded = load_into_cache();
            if let Ok(mut guard) = KEYWORD_CACHE.write() {
                *guard = Some(loaded.clone());
            }
            loaded
        },
    };

    let task = task_description.to_lowercase();
    let mut scored: Vec<(String, usize)> = map
        .into_iter()
        .map(|(domain, kws)| {
            let hits = kws.iter().filter(|kw| task.contains(&kw.to_lowercase())).count();
            (domain, hits)
        })
        .filter(|(_, hits)| *hits > 0)
        .collect();
    // hits 내림차순, 동률이면 도메인 이름 사전순(결정론적)
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    scored.into_iter().take(top_k).map(|(d, _)| d).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// 전역 `KEYWORD_CACHE` 를 사용하는 테스트들이 병렬 실행 시 서로 캐시를
    /// 오염시키는 것을 방지하기 위한 직렬화 가드.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn prime_with_defaults() {
        invalidate_cache();
        let mut map = HashMap::new();
        for (d, kws) in default_keywords() {
            map.insert(d, kws);
        }
        prime_cache(map);
    }

    /// @trace TC: SPEC-020/TC-1
    #[test]
    fn financial_task_selects_financial() {
        let _g = TEST_LOCK.lock().unwrap();
        prime_with_defaults();
        let r = select_domains("연 이자율 5% 로 1000만원을 예금했을 때 복리 계산해줘", 1);
        assert_eq!(r, vec!["financial".to_string()]);
    }

    /// @trace TC: SPEC-020/TC-2
    #[test]
    fn cs_task_selects_cs() {
        let _g = TEST_LOCK.lock().unwrap();
        prime_with_defaults();
        let r = select_domains("고객이 주문한 상품 환불을 요청했습니다", 1);
        assert_eq!(r, vec!["customer_service".to_string()]);
    }

    /// @trace TC: SPEC-020/TC-3
    #[test]
    fn mixed_task_top2_returns_both() {
        let _g = TEST_LOCK.lock().unwrap();
        prime_with_defaults();
        let r = select_domains("대출 이자 계산 중 고객 문의가 들어왔습니다", 2);
        assert!(r.contains(&"financial".to_string()));
        assert!(r.contains(&"customer_service".to_string()));
    }

    /// @trace TC: SPEC-020/TC-4
    #[test]
    fn no_match_returns_empty() {
        let _g = TEST_LOCK.lock().unwrap();
        prime_with_defaults();
        let r = select_domains("오늘 날씨가 좋네요", 3);
        assert!(r.is_empty(), "매칭 없는 task 는 빈 Vec");
    }

    /// @trace TC: SPEC-020/TC-5
    #[test]
    fn top_k_zero_returns_empty() {
        let _g = TEST_LOCK.lock().unwrap();
        prime_with_defaults();
        let r = select_domains("이자 계산", 0);
        assert!(r.is_empty());
    }

    /// @trace TC: SPEC-022/TC-10
    #[test]
    fn invalidate_then_reprime_picks_up_new_keywords() {
        let _g = TEST_LOCK.lock().unwrap();
        // 새 도메인 healthcare 를 캐시에 직접 주입
        let mut map = HashMap::new();
        map.insert("healthcare".to_string(), vec!["환자".to_string(), "처방".to_string()]);
        prime_cache(map);
        let r = select_domains("환자 처방전 분석", 1);
        assert_eq!(r, vec!["healthcare".to_string()]);

        // invalidate 후 default 가 lazy load 되지 않도록 다시 prime
        invalidate_cache();
        let mut map2 = HashMap::new();
        map2.insert("healthcare".to_string(), vec!["수술".to_string()]);
        prime_cache(map2);
        // 환자/처방 은 매칭 안 됨
        let r2 = select_domains("환자 처방전", 1);
        assert!(r2.is_empty(), "갱신된 키워드 반영");
    }
}
