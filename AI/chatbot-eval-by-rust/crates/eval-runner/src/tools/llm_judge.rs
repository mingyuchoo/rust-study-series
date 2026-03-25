//! LLM-as-a-Judge 평가 모듈
//!
//! GPT-4를 평가자로 사용하여 응답 품질 자동 채점.
//! Azure OpenAI 및 OpenAI API 모두 지원.

use models::{EvalResult,
             EvalSample};
use rag_core::{LlmClient,
               RagConfig,
               create_demo_chatbot};

/// LLM-as-a-Judge 평가 프롬프트
const EVAL_PROMPT_TEMPLATE: &str = "당신은 AI 응답 품질 평가자입니다. 다음 기준으로 응답을 0-10점으로 평가하세요.

## 평가 기준
1. 정확성 (Correctness): 응답이 질문에 정확히 답하는가?
2. 충실성 (Faithfulness): 응답이 제공된 문맥에 기반하는가?
3. 완전성 (Completeness): 필요한 정보를 빠뜨리지 않았는가?

## 입력
- 질문: {question}
- 문맥: {contexts}
- 응답: {answer}
- 기대 답변: {ground_truth}

## 출력 형식 (JSON)
{{\"correctness\": 점수, \"faithfulness\": 점수, \"completeness\": 점수, \"reasoning\": \"평가 이유\"}}";

/// LLM-as-a-Judge 방식 평가를 수행한다.
///
/// # Errors
///
/// 챗봇 초기화 실패 또는 LLM 호출 실패 시 에러를 반환한다.
pub async fn run_llm_as_judge(samples: &[EvalSample]) -> anyhow::Result<Vec<EvalResult>> {
    let config = RagConfig::from_env();

    println!("챗봇 초기화 중...");
    let chatbot = create_demo_chatbot().await?;

    // 평가용 LLM
    let judge_llm = LlmClient::new(&config)?;
    let backend = if config.use_azure { "Azure OpenAI" } else { "OpenAI API" };
    println!("[LLM-as-Judge] 백엔드: {backend}");

    let mut results = Vec::new();
    println!("\n{}개 샘플 LLM 평가 중...", samples.len());

    for sample in samples {
        let response = chatbot.query(&sample.question).await?;

        let eval_input = EVAL_PROMPT_TEMPLATE
            .replace("{question}", &sample.question)
            .replace("{contexts}", &response.contexts.join("\n"))
            .replace("{answer}", &response.answer)
            .replace("{ground_truth}", &sample.ground_truth);

        let eval_response = judge_llm.chat("당신은 AI 응답 품질 평가자입니다.", &eval_input).await?;

        match parse_judge_response(&eval_response) {
            | Some(scores) => {
                let correctness = scores.correctness / 10.0;
                let faithfulness = scores.faithfulness / 10.0;
                let overall = (scores.correctness + scores.faithfulness + scores.completeness) / 30.0;

                println!("\n[평가] {}...", truncate(&sample.question, 40));
                println!("  정확성: {correctness:.2}");
                println!("  충실성: {faithfulness:.2}");
                println!("  종합: {overall:.2}");
                if let Some(reasoning) = &scores.reasoning {
                    println!("  이유: {}...", truncate(reasoning, 100));
                }

                results.push(EvalResult {
                    question: sample.question.clone(),
                    answer: response.answer.clone(),
                    ground_truth: sample.ground_truth.clone(),
                    correctness: Some(correctness),
                    faithfulness: Some(faithfulness),
                    overall: Some(overall),
                    ..Default::default()
                });
            },
            | None => {
                tracing::warn!("평가 파싱 실패: {}...", truncate(&sample.question, 30));
            },
        }
    }

    // 평균 계산
    if !results.is_empty() {
        let n = results.len() as f64;
        let avg_correctness: f64 = results.iter().filter_map(|r| r.correctness).sum::<f64>() / n;
        let avg_faithfulness: f64 = results.iter().filter_map(|r| r.faithfulness).sum::<f64>() / n;
        let avg_overall: f64 = results.iter().filter_map(|r| r.overall).sum::<f64>() / n;

        println!("\n=== LLM-as-a-Judge 평가 결과 ===");
        println!("평균 정확성: {avg_correctness:.2}");
        println!("평균 충실성: {avg_faithfulness:.2}");
        println!("평균 종합: {avg_overall:.2}");
    }

    Ok(results)
}

/// 파싱된 Judge 점수
struct JudgeScores {
    correctness: f64,
    faithfulness: f64,
    completeness: f64,
    reasoning: Option<String>,
}

/// 평가 JSON 응답을 파싱한다.
fn parse_judge_response(text: &str) -> Option<JudgeScores> {
    let start = text.find('{')?;
    let end = text.rfind('}')? + 1;
    let json_str = &text[start .. end];

    let value: serde_json::Value = serde_json::from_str(json_str).ok()?;

    Some(JudgeScores {
        correctness: value.get("correctness")?.as_f64()?,
        faithfulness: value.get("faithfulness")?.as_f64()?,
        completeness: value.get("completeness")?.as_f64()?,
        reasoning: value.get("reasoning").and_then(|v| v.as_str()).map(String::from),
    })
}

/// 문자열을 지정 길이로 자른다.
fn truncate(s: &str, max_chars: usize) -> String { s.chars().take(max_chars).collect() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 유효한_json_응답을_파싱한다() {
        let response = r#"{"correctness": 8, "faithfulness": 9, "completeness": 7, "reasoning": "좋은 답변"}"#;
        let result = parse_judge_response(response);
        assert!(result.is_some());
        let scores = result.unwrap();
        assert!((scores.correctness - 8.0).abs() < f64::EPSILON);
        assert!((scores.faithfulness - 9.0).abs() < f64::EPSILON);
        assert!((scores.completeness - 7.0).abs() < f64::EPSILON);
        assert_eq!(scores.reasoning.unwrap(), "좋은 답변");
    }

    #[test]
    fn json_이_아닌_텍스트에서_json_을_추출한다() {
        let response = "평가 결과:\n{\"correctness\": 7, \"faithfulness\": 8, \"completeness\": 6, \"reasoning\": \"테스트\"}";
        let result = parse_judge_response(response);
        assert!(result.is_some());
    }

    #[test]
    fn 잘못된_응답은_none을_반환한다() {
        assert!(parse_judge_response("no json here").is_none());
        assert!(parse_judge_response("{}").is_none());
    }
}
