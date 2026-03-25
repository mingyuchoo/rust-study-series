//! Langfuse 평가 모듈
//!
//! Langfuse를 사용한 추적 및 평가.
//! On-Premise Langfuse 서버에 RAG 실행 결과를 저장하고 평가 점수를 기록.
//!
//! 사전 요구사항:
//! - `docker-compose -f docker-compose.langfuse.yml up -d`
//! - `LANGFUSE_ENABLED=true`
//! - `LANGFUSE_PUBLIC_KEY`, `LANGFUSE_SECRET_KEY` 설정

use models::{EvalSample,
             LangfuseDetail,
             LangfuseResult};
use rag_core::{RagConfig,
               create_demo_chatbot};
use serde::Serialize;
use std::collections::HashSet;

/// Langfuse 추적 생성 요청
#[derive(Serialize)]
struct LangfuseTraceRequest {
    name: String,
    input: serde_json::Value,
    output: serde_json::Value,
    metadata: serde_json::Value,
}

/// Langfuse 점수 생성 요청
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LangfuseScoreRequest {
    trace_id: String,
    name: String,
    value: f64,
    comment: String,
}

/// Langfuse를 사용한 추적 및 평가를 수행한다.
///
/// # Errors
///
/// Langfuse 비활성화, API 키 누락, 챗봇 초기화 실패 시 에러를 반환한다.
pub async fn run_langfuse_evaluation(samples: &[EvalSample]) -> anyhow::Result<LangfuseResult> {
    let config = RagConfig::from_env();

    if !config.is_langfuse_available() {
        println!("Langfuse가 비활성화되어 있습니다. LANGFUSE_ENABLED=true로 설정하세요.");
        anyhow::bail!("Langfuse 비활성화");
    }

    let langfuse_host = &config.langfuse_host;
    let public_key = config
        .langfuse_public_key
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("LANGFUSE_PUBLIC_KEY가 설정되지 않았습니다"))?;
    let secret_key = config
        .langfuse_secret_key
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("LANGFUSE_SECRET_KEY가 설정되지 않았습니다"))?;

    println!("챗봇 초기화 중...");
    let chatbot = create_demo_chatbot().await?;

    let http = reqwest::Client::new();
    let mut results = Vec::new();

    println!("\n{}개 샘플 Langfuse 추적 중...", samples.len());

    for (i, sample) in samples.iter().enumerate() {
        println!("\n[{}/{}] {}...", i + 1, samples.len(), truncate(&sample.question, 40));

        let response = chatbot.query(&sample.question).await?;

        // 키워드 기반 간단 평가
        let ground_truth_words: HashSet<String> = sample.ground_truth.to_lowercase().split_whitespace().map(String::from).collect();
        let answer_words: HashSet<String> = response.answer.to_lowercase().split_whitespace().map(String::from).collect();
        let overlap = ground_truth_words.intersection(&answer_words).count() as f64 / ground_truth_words.len().max(1) as f64;

        // Langfuse API로 트레이스 생성
        let trace_request = LangfuseTraceRequest {
            name: "rag-evaluation".to_string(),
            input: serde_json::json!({ "question": sample.question }),
            output: serde_json::json!({
                "answer": response.answer,
                "contexts": response.contexts,
            }),
            metadata: serde_json::json!({
                "sample_index": i,
                "ground_truth": sample.ground_truth,
                "tags": ["evaluation", "golden-dataset"],
            }),
        };

        let trace_id = match http
            .post(format!("{langfuse_host}/api/public/traces"))
            .basic_auth(public_key, Some(secret_key))
            .json(&trace_request)
            .send()
            .await
        {
            | Ok(resp) if resp.status().is_success() => {
                let body: serde_json::Value = resp.json().await.unwrap_or_default();
                body.get("id").and_then(|v| v.as_str()).map(String::from)
            },
            | Ok(resp) => {
                tracing::warn!("Langfuse 트레이스 생성 실패: {}", resp.status());
                None
            },
            | Err(e) => {
                tracing::warn!("Langfuse 요청 실패: {e}");
                None
            },
        };

        // 점수 기록
        if let Some(ref tid) = trace_id {
            let score_request = LangfuseScoreRequest {
                trace_id: tid.clone(),
                name: "keyword_overlap".to_string(),
                value: overlap,
                comment: format!("키워드 일치율: {:.2}%", overlap * 100.0),
            };

            if let Err(e) = http
                .post(format!("{langfuse_host}/api/public/scores"))
                .basic_auth(public_key, Some(secret_key))
                .json(&score_request)
                .send()
                .await
            {
                tracing::warn!("Langfuse 점수 기록 실패: {e}");
            }
        }

        println!("  응답: {}...", truncate(&response.answer, 80));
        println!("  키워드 일치율: {:.2}%", overlap * 100.0);
        if let Some(ref tid) = trace_id {
            println!("  트레이스 ID: {tid}");
        }

        results.push(LangfuseDetail {
            question: sample.question.clone(),
            answer: response.answer.clone(),
            ground_truth: sample.ground_truth.clone(),
            contexts: response.contexts.clone(),
            keyword_overlap: overlap,
            trace_id,
        });
    }

    let avg_overlap = if results.is_empty() {
        0.0
    } else {
        results.iter().map(|r| r.keyword_overlap).sum::<f64>() / results.len() as f64
    };

    println!("\n=== Langfuse 평가 결과 ===");
    println!("총 샘플: {}", samples.len());
    println!("평균 키워드 일치율: {:.2}%", avg_overlap * 100.0);
    println!("\n대시보드에서 상세 결과 확인:");
    println!("  {langfuse_host}/traces");

    Ok(LangfuseResult {
        total: samples.len(),
        avg_keyword_overlap: avg_overlap,
        details: results,
    })
}

/// 문자열을 지정 길이로 자른다.
fn truncate(s: &str, max_chars: usize) -> String { s.chars().take(max_chars).collect() }
