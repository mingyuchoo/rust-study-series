//! 통합 질의응답 엔드포인트 (MVP)
//! 모든 주석은 한국어로 작성됩니다.

use actix_web::{post, web, HttpRequest, Result};
use std::time::Instant;

use crate::auth::require_auth;
use crate::error::Error;
use crate::models::{ChatAskRequest, ChatAskResponse, SourceItem, GraphPathItem};
use crate::search::AppState;

#[post("/api/chat/ask")]
pub async fn chat_ask(state: web::Data<AppState>, req: HttpRequest, payload: web::Json<ChatAskRequest>) -> Result<web::Json<ChatAskResponse>, Error> {
    // 인증 토큰 검증
    let _user = require_auth(&req, &state.cfg)?;

    let t0 = Instant::now();

    // 1) (MVP) 벡터 검색 스텁: 임시로 빈 소스
    let sources: Vec<SourceItem> = vec![];

    // 2) 그래프 경로 스텁: 임시로 빈 경로
    let graph_paths: Vec<GraphPathItem> = vec![];

    // 3) LLM 호출
    let system_prompt = "당신은 사용자의 질문에 대해 제공된 소스와 도메인 힌트를 참고해 간결하고 정확하게 한국어로 답변하는 도우미입니다.";
    let temperature = payload
        .options
        .as_ref()
        .and_then(|o| o.get("temperature").and_then(|v| v.as_f64()))
        .map(|v| v as f32);

    let (answer, tokens_used) = state
        .azure
        .chat_complete(system_prompt, &payload.query, temperature)
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
