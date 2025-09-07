//! 통합 질의응답 엔드포인트 (MVP)


use actix_web::{post, web, HttpRequest, Result};
use std::time::Instant;

use crate::auth::require_auth;
use crate::error::Error;
use crate::models::{ChatAskRequest, ChatAskResponse, SourceItem, GraphPathItem};
use crate::search::AppState;
use lib_db::DB;

#[utoipa::path(
    tag = "chat",
    post,
    path = "/api/chat/ask",
    request_body = ChatAskRequest,
    responses(
        (status = 200, description = "질의응답 결과", body = ChatAskResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 401, description = "인증 실패"),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/chat/ask")]
pub async fn chat_ask(state: web::Data<AppState>, req: HttpRequest, payload: web::Json<ChatAskRequest>) -> Result<web::Json<ChatAskResponse>, Error> {
    // 인증 토큰 검증
    let _user = require_auth(&req, &state.cfg)?;

    let t0 = Instant::now();

    // 1) 벡터 검색: 질의 임베딩 생성 후 chunk 테이블에서 유사도 상위 문맥 수집
    let embeddings = state
        .azure
        .embed(&[&payload.query])
        .await
        .map_err(|e| Error::External(e.to_string()))?;
    let query_vec = embeddings.get(0).cloned().unwrap_or_default();

    let mut res = DB
        .query(
            r#"
            SELECT id, content, metadata,
                   vector::similarity::cosine(embedding_semantic, $q) AS score
            FROM chunk
            ORDER BY score DESC
            LIMIT 8;
            "#,
        )
        .bind(("q", query_vec))
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    let rows: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
    let mut sources: Vec<SourceItem> = Vec::new();
    let mut context_text = String::new();
    for v in rows {
        let score = v.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0) as f32;
        let content = v
            .get("content")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string();
        let metadata = v.get("metadata").cloned().unwrap_or(serde_json::json!({}));
        if !content.is_empty() {
            context_text.push_str("- ");
            context_text.push_str(&content);
            context_text.push_str("\n");
        }
        sources.push(SourceItem {
            r#type: "chunk".into(),
            content,
            score,
            metadata,
        });
    }

    // 2) 그래프 경로 (MVP): 아직 추출하지 않으므로 빈 목록 유지
    let graph_paths: Vec<GraphPathItem> = vec![];

    // 3) LLM 호출 — 컨텍스트를 system_prompt에 포함하여 RAG 효과
    let system_prompt = format!(
        "{}\n\n[컨텍스트]\n{}",
        "당신은 제공된 문서 청크 컨텍스트를 활용하여 질문에 대해 간결하고 정확하게 한국어로 답변합니다. 모르는 내용은 추측하지 말고 모른다고 답하세요.",
        context_text
    );
    let temperature = payload
        .options
        .as_ref()
        .and_then(|o| o.get("temperature").and_then(|v| v.as_f64()))
        .map(|v| v as f32);

    let (answer, tokens_used) = state
        .azure
        .chat_complete(&system_prompt, &payload.query, temperature)
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    let elapsed = t0.elapsed().as_secs_f32();

    Ok(web::Json(ChatAskResponse {
        response: answer,
        conversation_id: payload.conversation_id.clone(),
        sources,
        graph_paths,
        query_time: elapsed,
        tokens_used,
    }))
}
