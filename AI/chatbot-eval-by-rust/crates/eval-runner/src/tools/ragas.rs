//! RAGAS 평가 모듈
//!
//! RAG 파이프라인 평가 메트릭 계산:
//! - `context_utilization`: 문맥 활용도 (네이티브 계산)
//! - `response_quality`: 종합 품질 점수 (가중 평균)
//! - LLM 기반 메트릭 (faithfulness, `answer_relevancy` 등)

use crate::utils::safe_float;
use models::{EvalSample,
             RagasResult};
use rag_core::{LlmClient,
               RagConfig,
               create_demo_chatbot};
use regex::Regex;
use std::{collections::HashSet,
          sync::LazyLock};

/// 한글/영문 단어 추출 정규표현식 (한 번만 컴파일)
static WORD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[가-힣]+|[a-zA-Z]+").expect("유효한 정규표현식"));

/// 한글/영문 불용어
const STOPWORDS: &[&str] = &[
    "the",
    "is",
    "are",
    "was",
    "were",
    "be",
    "been",
    "have",
    "has",
    "had",
    "do",
    "does",
    "did",
    "will",
    "would",
    "could",
    "should",
    "may",
    "might",
    "must",
    "can",
    "and",
    "or",
    "but",
    "if",
    "then",
    "else",
    "when",
    "where",
    "which",
    "who",
    "whom",
    "this",
    "that",
    "these",
    "those",
    "있다",
    "없다",
    "하다",
    "되다",
    "이다",
    "있는",
    "없는",
    "하는",
    "되는",
    "것이",
    "수가",
    "등을",
    "위한",
    "통해",
    "대한",
    "으로",
    "에서",
    "까지",
    "부터",
    "그리고",
    "또한",
    "하지만",
    "그러나",
];

/// RAGAS 평가 가중치
const WEIGHTS: &[(&str, f64)] = &[
    ("faithfulness", 0.20),
    ("answer_relevancy", 0.20),
    ("context_precision", 0.15),
    ("context_recall", 0.15),
    ("answer_correctness", 0.20),
    ("context_utilization", 0.10),
];

/// 문맥 활용도를 계산한다 (Context Utilization).
///
/// 응답에서 문맥의 핵심 단어가 얼마나 활용되었는지 측정한다.
#[must_use]
pub fn calculate_context_utilization(answer: &str, contexts: &[String]) -> f64 {
    if contexts.is_empty() || answer.is_empty() {
        return 0.0;
    }

    let stopwords: HashSet<&str> = STOPWORDS.iter().copied().collect();

    // 문맥에서 중요 단어 추출 (2자 이상)
    let context_text = contexts.join(" ");
    let context_words: HashSet<String> = WORD_RE
        .find_iter(&context_text)
        .map(|m| m.as_str().to_lowercase())
        .filter(|w| w.chars().count() >= 2 && !stopwords.contains(w.as_str()))
        .collect();

    if context_words.is_empty() {
        return 0.0;
    }

    // 응답에서 단어 추출
    let answer_words: HashSet<String> = WORD_RE
        .find_iter(answer)
        .map(|m| m.as_str().to_lowercase())
        .filter(|w| w.chars().count() >= 2)
        .collect();

    // 교집합 비율 계산
    let overlap = context_words.intersection(&answer_words).count();
    let utilization = overlap as f64 / context_words.len() as f64;

    utilization.min(1.0)
}

/// 응답 품질 종합 점수를 계산한다 (Response Quality).
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn calculate_response_quality(scores: &std::collections::HashMap<String, f64>) -> f64 {
    let mut total_weight = 0.0;
    let mut weighted_sum = 0.0;

    for (metric, weight) in WEIGHTS {
        if let Some(&value) = scores.get(*metric) {
            weighted_sum += value * weight;
            total_weight += weight;
        }
    }

    if total_weight == 0.0 {
        return 0.0;
    }

    weighted_sum / total_weight
}

/// LLM 기반 RAGAS 메트릭을 평가한다.
///
/// # Errors
///
/// 챗봇 초기화 실패 또는 LLM 호출 실패 시 에러를 반환한다.
pub async fn run_ragas_evaluation(samples: &[EvalSample]) -> anyhow::Result<RagasResult> {
    let config = RagConfig::from_env();

    println!("챗봇 초기화 중...");
    let chatbot = create_demo_chatbot().await?;

    let judge_llm = LlmClient::new(&config)?;
    let backend = if config.use_azure { "Azure OpenAI" } else { "OpenAI API" };
    println!("[RAGAS] 백엔드: {backend}");

    let mut answers = Vec::new();
    let mut all_contexts = Vec::new();

    println!("\n{}개 샘플 평가 중...", samples.len());
    for sample in samples {
        let response = chatbot.query(&sample.question).await?;
        answers.push(response.answer.clone());
        all_contexts.push(response.contexts.clone());
    }

    // 문맥 활용도 계산
    let context_utilization_scores: Vec<f64> = answers
        .iter()
        .zip(all_contexts.iter())
        .map(|(answer, contexts)| calculate_context_utilization(answer, contexts))
        .collect();

    let avg_context_utilization = if context_utilization_scores.is_empty() {
        0.0
    } else {
        context_utilization_scores.iter().sum::<f64>() / context_utilization_scores.len() as f64
    };

    // LLM 기반 RAGAS 메트릭 평가
    println!("\nRAGAS LLM 평가 실행 중...");
    let mut faithfulness_scores = Vec::new();
    let mut relevancy_scores = Vec::new();
    let mut correctness_scores = Vec::new();
    let mut precision_scores = Vec::new();
    let mut recall_scores = Vec::new();

    for (i, (sample, (answer, contexts))) in samples.iter().zip(answers.iter().zip(all_contexts.iter())).enumerate() {
        let context_text = contexts.join("\n");

        let eval_prompt = format!(
            "다음 RAG 시스템의 응답을 평가하세요. 각 메트릭을 0.0~1.0 범위로 채점하세요.

질문: {}
문맥: {}
응답: {}
기대 답변: {}

다음 JSON 형식으로만 응답하세요:
{{\"faithfulness\": 점수, \"answer_relevancy\": 점수, \"context_precision\": 점수, \"context_recall\": 점수, \"answer_correctness\": 점수}}

메트릭 설명:
- faithfulness: 응답이 문맥에 기반하는지 (충실성)
- answer_relevancy: 응답이 질문에 관련있는지 (답변 관련성)
- context_precision: 검색된 문맥이 질문과 관련있는지 (문맥 정밀도)
- context_recall: 문맥이 기대 답변을 커버하는지 (문맥 재현율)
- answer_correctness: 응답이 기대 답변과 일치하는지 (답변 정확성)",
            sample.question, context_text, answer, sample.ground_truth
        );

        match judge_llm.chat("당신은 RAG 시스템 평가자입니다. JSON으로만 응답하세요.", &eval_prompt).await {
            | Ok(response) =>
                if let Some(scores) = parse_ragas_scores(&response) {
                    faithfulness_scores.push(scores.0);
                    relevancy_scores.push(scores.1);
                    precision_scores.push(scores.2);
                    recall_scores.push(scores.3);
                    correctness_scores.push(scores.4);
                },
            | Err(e) => {
                tracing::warn!("RAGAS LLM 평가 실패 (샘플 {i}): {e}");
            },
        }
    }

    let avg = |scores: &[f64]| -> Option<f64> {
        if scores.is_empty() {
            None
        } else {
            safe_float(scores.iter().sum::<f64>() / scores.len() as f64)
        }
    };

    let faithfulness = avg(&faithfulness_scores);
    let answer_relevancy = avg(&relevancy_scores);
    let context_precision = avg(&precision_scores);
    let context_recall = avg(&recall_scores);
    let answer_correctness = avg(&correctness_scores);
    let context_utilization = safe_float(avg_context_utilization);

    // 종합 점수 계산
    let mut score_map = std::collections::HashMap::new();
    if let Some(v) = faithfulness {
        score_map.insert("faithfulness".to_string(), v);
    }
    if let Some(v) = answer_relevancy {
        score_map.insert("answer_relevancy".to_string(), v);
    }
    if let Some(v) = context_precision {
        score_map.insert("context_precision".to_string(), v);
    }
    if let Some(v) = context_recall {
        score_map.insert("context_recall".to_string(), v);
    }
    if let Some(v) = answer_correctness {
        score_map.insert("answer_correctness".to_string(), v);
    }
    if let Some(v) = context_utilization {
        score_map.insert("context_utilization".to_string(), v);
    }

    let response_quality = safe_float(calculate_response_quality(&score_map));

    println!("\n=== RAGAS 평가 결과 ===");
    println!("faithfulness: {faithfulness:?}");
    println!("answer_relevancy: {answer_relevancy:?}");
    println!("context_precision: {context_precision:?}");
    println!("context_recall: {context_recall:?}");
    println!("answer_correctness: {answer_correctness:?}");
    println!("context_utilization: {context_utilization:?}");
    println!("response_quality: {response_quality:?}");

    Ok(RagasResult {
        context_precision,
        context_recall,
        faithfulness,
        answer_relevancy,
        answer_correctness,
        context_utilization,
        response_quality,
    })
}

/// RAGAS LLM 응답을 파싱한다.
///
/// Returns: (faithfulness, `answer_relevancy`, `context_precision`,
/// `context_recall`, `answer_correctness`)
fn parse_ragas_scores(text: &str) -> Option<(f64, f64, f64, f64, f64)> {
    let start = text.find('{')?;
    let end = text.rfind('}')? + 1;
    let value: serde_json::Value = serde_json::from_str(&text[start .. end]).ok()?;

    let faithfulness = value.get("faithfulness")?.as_f64()?;
    let answer_relevancy = value.get("answer_relevancy")?.as_f64()?;
    let context_precision = value.get("context_precision")?.as_f64()?;
    let context_recall = value.get("context_recall")?.as_f64()?;
    let answer_correctness = value.get("answer_correctness")?.as_f64()?;

    Some((faithfulness, answer_relevancy, context_precision, context_recall, answer_correctness))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 문맥_활용도가_올바르게_계산된다() {
        let answer = "손오공은 사이어인 종족입니다";
        let contexts = vec!["손오공은 사이어인 소년이다".to_string()];
        let score = calculate_context_utilization(answer, &contexts);
        assert!(score > 0.0);
    }

    #[test]
    fn 빈_문맥의_활용도는_0이다() {
        let score = calculate_context_utilization("답변", &[]);
        assert!(score.abs() < f64::EPSILON);
    }

    #[test]
    fn 빈_답변의_활용도는_0이다() {
        let score = calculate_context_utilization("", &["문맥".to_string()]);
        assert!(score.abs() < f64::EPSILON);
    }

    #[test]
    fn 종합_점수가_가중_평균으로_계산된다() {
        let mut scores = std::collections::HashMap::new();
        scores.insert("faithfulness".to_string(), 0.8);
        scores.insert("answer_relevancy".to_string(), 0.9);
        scores.insert("context_precision".to_string(), 0.7);
        scores.insert("context_recall".to_string(), 0.6);
        scores.insert("answer_correctness".to_string(), 0.85);
        scores.insert("context_utilization".to_string(), 0.5);

        let quality = calculate_response_quality(&scores);
        assert!(quality > 0.0 && quality <= 1.0);
    }

    #[test]
    fn ragas_json_응답을_파싱한다() {
        let response = r#"{"faithfulness": 0.8, "answer_relevancy": 0.9, "context_precision": 0.7, "context_recall": 0.6, "answer_correctness": 0.85}"#;
        let result = parse_ragas_scores(response);
        assert!(result.is_some());
        let (f, ar, cp, cr, ac) = result.unwrap();
        assert!((f - 0.8).abs() < f64::EPSILON);
        assert!((ar - 0.9).abs() < f64::EPSILON);
        assert!((cp - 0.7).abs() < f64::EPSILON);
        assert!((cr - 0.6).abs() < f64::EPSILON);
        assert!((ac - 0.85).abs() < f64::EPSILON);
    }
}
