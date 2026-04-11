// =============================================================================
// @trace SPEC-020
// @trace PRD: PRD-020
// @trace FR: PRD-020/FR-3
// @trace file-type: impl
// =============================================================================
//
// 도메인 pre-filter. `task_description` 의 키워드를 집계해 가장 관련이 높은
// 상위 K 개 도메인을 선택한다. 임베딩 기반 라우터를 붙이기 전 1차 근사치이며,
// 매칭이 전혀 없으면 빈 Vec 를 반환하여 상위 호출자가 "모든 도메인 유지" 로
// 폴백하도록 한다.

/// `(domain, keyword)` 페어. 소문자 부분 문자열 매칭.
const DOMAIN_KEYWORDS: &[(&str, &[&str])] = &[
    (
        "financial",
        &[
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
        ],
    ),
    (
        "customer_service",
        &[
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
        ],
    ),
];

/// task 설명에서 각 도메인의 키워드 매칭 건수를 센 뒤 상위 `top_k` 개만
/// 반환한다. 동률은 `DOMAIN_KEYWORDS` 의 등록 순서를 유지한다. 매칭 0건이면
/// 빈 Vec.
pub fn select_domains(task_description: &str, top_k: usize) -> Vec<String> {
    if top_k == 0 {
        return Vec::new();
    }
    let task = task_description.to_lowercase();
    let mut scored: Vec<(&str, usize, usize)> = DOMAIN_KEYWORDS
        .iter()
        .enumerate()
        .map(|(idx, (domain, kws))| {
            let hits = kws.iter().filter(|kw| task.contains(&kw.to_lowercase())).count();
            (*domain, hits, idx)
        })
        .filter(|(_, hits, _)| *hits > 0)
        .collect();
    // hits 내림차순, 동률이면 등록 순서 오름차순
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
    scored.into_iter().take(top_k).map(|(d, _, _)| d.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @trace TC: SPEC-020/TC-1
    #[test]
    fn financial_task_selects_financial() {
        let r = select_domains("연 이자율 5% 로 1000만원을 예금했을 때 복리 계산해줘", 1);
        assert_eq!(r, vec!["financial".to_string()]);
    }

    /// @trace TC: SPEC-020/TC-2
    #[test]
    fn cs_task_selects_cs() {
        let r = select_domains("고객이 주문한 상품 환불을 요청했습니다", 1);
        assert_eq!(r, vec!["customer_service".to_string()]);
    }

    /// @trace TC: SPEC-020/TC-3
    #[test]
    fn mixed_task_top2_returns_both() {
        let r = select_domains("대출 이자 계산 중 고객 문의가 들어왔습니다", 2);
        assert!(r.contains(&"financial".to_string()));
        assert!(r.contains(&"customer_service".to_string()));
    }

    /// @trace TC: SPEC-020/TC-4
    #[test]
    fn no_match_returns_empty() {
        let r = select_domains("오늘 날씨가 좋네요", 3);
        assert!(r.is_empty(), "매칭 없는 task 는 빈 Vec");
    }

    /// @trace TC: SPEC-020/TC-5
    #[test]
    fn top_k_zero_returns_empty() {
        let r = select_domains("이자 계산", 0);
        assert!(r.is_empty());
    }
}
