//! 안전성 평가 모듈
//!
//! 적대적 프롬프트를 사용한 가드레일 테스트:
//! - 시스템 프롬프트 노출 방지
//! - 프롬프트 인젝션 방어
//! - 민감 정보 보호

use crate::{config::{EvalConfig,
                     default_violation_keywords},
            dataset::{legacy_adversarial_prompts,
                      load_adversarial_tests}};
use models::{AdversarialTest,
             SafetyDetail,
             SafetyReport,
             SafetyResult};
use rag_core::create_demo_chatbot;

/// 안전성 평가를 수행한다 (적대적 프롬프트).
///
/// # Errors
///
/// 챗봇 초기화 실패 또는 RAG 질의 실패 시 에러를 반환한다.
pub async fn run_safety_evaluation(adversarial_tests: Option<Vec<AdversarialTest>>, violation_keywords: Option<Vec<String>>) -> anyhow::Result<SafetyReport> {
    let eval_config = EvalConfig::from_cwd();

    // 테스트 데이터 로드
    let tests = match adversarial_tests {
        | Some(tests) => tests,
        | None => match load_adversarial_tests(&eval_config.golden_dataset_path) {
            | Ok(tests) if !tests.is_empty() => {
                println!("[Safety] golden_dataset.json에서 {}개 테스트 로드", tests.len());
                tests
            },
            | _ => {
                // 레거시 데이터 사용
                let prompts = legacy_adversarial_prompts();
                let legacy: Vec<AdversarialTest> = prompts
                    .into_iter()
                    .map(|p| AdversarialTest {
                        prompt: p,
                        expected_behavior: String::new(),
                        violation_keywords: Vec::new(),
                        id: None,
                        category: None,
                    })
                    .collect();
                println!("[Safety] 레거시 데이터셋 사용 ({}개)", legacy.len());
                legacy
            },
        },
    };

    let keywords: Vec<String> = violation_keywords.unwrap_or_else(|| default_violation_keywords().into_iter().map(String::from).collect());

    println!("챗봇 초기화 중...");
    let chatbot = create_demo_chatbot().await?;

    let mut results: Vec<SafetyResult> = Vec::new();
    let mut violations = 0;

    println!("\n{}개 적대적 프롬프트 테스트 중...", tests.len());

    for test in &tests {
        println!("\n[테스트] {}", test.prompt);
        let response = chatbot.query(&test.prompt).await?;
        let answer_lower = response.answer.to_lowercase();

        // 테스트별 위반 키워드와 기본 위반 키워드 결합
        let mut all_keywords: Vec<String> = keywords.clone();
        all_keywords.extend(test.violation_keywords.clone());
        all_keywords.sort();
        all_keywords.dedup();

        // 위반 체크
        let detected_violation = all_keywords.iter().find(|kw| answer_lower.contains(&kw.to_lowercase())).cloned();

        let is_violation = detected_violation.is_some();
        if is_violation {
            violations += 1;
            println!("  응답: {}...", truncate(&response.answer, 100));
            println!("  결과: X 위반");
        } else {
            println!("  응답: {}...", truncate(&response.answer, 100));
            println!("  결과: O 통과");
        }

        results.push(SafetyResult {
            prompt: test.prompt.clone(),
            response: response.answer.clone(),
            passed: !is_violation,
            violation_detected: detected_violation,
        });
    }

    // 결과 요약
    let total = tests.len();
    let passed = total - violations;
    let pass_rate = if total > 0 { passed as f64 / total as f64 } else { 0.0 };

    println!("\n=== 안전성 평가 결과 ===");
    println!("총 테스트: {total}");
    if total > 0 {
        println!("통과: {passed} ({:.1}%)", pass_rate * 100.0);
        println!("위반: {violations} ({:.1}%)", (1.0 - pass_rate) * 100.0);
    } else {
        println!("테스트 케이스가 없습니다.");
    }

    Ok(SafetyReport {
        total,
        passed,
        violations,
        pass_rate,
        details: results
            .into_iter()
            .map(|r| SafetyDetail {
                prompt: r.prompt,
                response: r.response,
                passed: r.passed,
            })
            .collect(),
    })
}

/// 문자열을 지정 길이로 자른다.
fn truncate(s: &str, max_chars: usize) -> String { s.chars().take(max_chars).collect() }

#[cfg(test)]
mod tests {
    #[test]
    fn 위반_키워드_대소문자_무시_확인() {
        let answer = "API_KEY는 노출하지 않습니다".to_lowercase();
        let keyword = "api_key";
        assert!(answer.contains(keyword));
    }
}
