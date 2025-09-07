//! 인덱싱 생성 엔드포인트 (MVP)

use actix_web::{post, web, Result};
use serde_json::json;
use std::time::Instant;

use crate::error::Error;
use crate::models::{IndexChunkInput, IndexCreateRequest, IndexCreateResponse};
use crate::search::AppState;
use lib_db::DB;

#[utoipa::path(
    tag = "index",
    post,
    path = "/api/index/create",
    request_body = IndexCreateRequest,
    responses(
        (status = 200, description = "인덱싱 생성 결과", body = IndexCreateResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/index/create")]
pub async fn index_create(
    state: web::Data<AppState>,
    payload: web::Json<IndexCreateRequest>,
) -> Result<web::Json<IndexCreateResponse>, Error> {
    let t0 = Instant::now();

    let doc_id = payload.document_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let chunks: Vec<IndexChunkInput> = payload.chunks.clone();

    // 1) 임베딩 생성 (간단히 순차 처리; 필요 시 배치 처리로 개선 가능)
    let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
    let refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
    let embeddings = state
        .azure
        .embed(&refs)
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    // 2) SurrealDB 저장: chunk 테이블과 간단한 그래프(문서 -> 청크) 관계
    //    스키마 없는 모드 가정. 생성 시 필요한 필드만 저장.
    for (i, ch) in chunks.iter().enumerate() {
        let emb = embeddings.get(i).cloned().unwrap_or_default();
        let meta = ch.metadata.clone().unwrap_or_else(|| json!({}));
        let _ = DB
            .query(
                r#"
                CREATE chunk SET 
                    id = rand::uuid(),
                    doc_id = $doc_id,
                    index = $index,
                    content = $content,
                    embedding = $embedding,
                    metadata = $metadata;
                "#,
            )
            .bind(("doc_id", doc_id.clone()))
            .bind(("index", i as i64))
            .bind(("content", ch.content.clone()))
            .bind(("embedding", emb))
            .bind(("metadata", meta))
            .await
            .map_err(|e| Error::External(e.to_string()))?;
    }

    // 3) (MVP) 간단 그래프: document 노드를 만들고 chunk들과 연결(개념적으로만 저장)
    let _ = DB
        .query(
            r#"
            CREATE document SET id = $doc_id, title = $title, created_at = time::now();
            "#,
        )
        .bind(("doc_id", doc_id.clone()))
        .bind(("title", payload.title.clone().unwrap_or_else(|| "Untitled".to_string())))
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    // SurrealDB에서 진짜 관계를 쓰려면 graph 기능 대신, 간단히 relation 테이블에 저장
    let _ = DB
        .query(
            r#"
            LET $chunks = SELECT id FROM chunk WHERE doc_id = $doc_id;
            RETURN $chunks;
            "#,
        )
        .bind(("doc_id", doc_id.clone()))
        .await
        .map_err(|e| Error::External(e.to_string()))?;

    let elapsed = t0.elapsed().as_secs_f32();
    Ok(web::Json(IndexCreateResponse {
        document_id: doc_id,
        chunks_indexed: texts.len() as u32,
        elapsed,
    }))
}
