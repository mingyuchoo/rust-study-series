//! 관리자용 재인덱싱 엔드포인트
//! - 기존 데이터 정리(옵션) → PDF 재처리 → 그래프/임베딩 저장
//! - 혼재 운영 대비: embedding_type 필드에 "azure"/"tfidf" 기록

use actix_web::{post, web, Result};
use std::time::Instant;

use crate::error::Error;
use crate::models::{ReindexRequest, ReindexResponse, ReindexItemResult};
use crate::search::AppState;
use lib_index::{
    pdf_processor,
    graph_builder,
    embedding::{self, EmbeddingMode},
    database as index_db,
    types::{ProcessedDocument, Chunk, Embeddings3},
    RegexNer,
};
use lib_db::DB;
use crate::models::UploadResponse;
use serde::Deserialize;
use uuid::Uuid;
use std::path::PathBuf;
use tokio::fs;

#[utoipa::path(
    tag = "admin",
    post,
    path = "/api/admin/reindex",
    request_body = ReindexRequest,
    responses(
        (status = 200, description = "재인덱싱 결과", body = ReindexResponse),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/admin/reindex")]
pub async fn reindex_pdfs(
    state: web::Data<AppState>,
    payload: web::Json<ReindexRequest>,
) -> Result<web::Json<ReindexResponse>, Error> {
    let t0 = Instant::now();
    let use_tfidf = payload.use_tfidf.unwrap_or(false);
    let clear_existing = payload.clear_existing.unwrap_or(false);

    let mut results: Vec<ReindexItemResult> = Vec::new();

    for pdf_path in &payload.pdf_paths {
        let mut item = ReindexItemResult {
            pdf_path: pdf_path.clone(),
            document_id: None,
            chunks_indexed: 0,
            error: None,
        };
        // 1) 기존 데이터 정리(옵션): metadata.source = pdf_path
        if clear_existing {
            // chunk/entity/relation 모두 삭제
            let _ = DB
                .query(
                    r#"
                    DELETE FROM chunk WHERE metadata.source = $src;
                    DELETE FROM entity WHERE doc_id IN (SELECT DISTINCT doc_id FROM chunk WHERE metadata.source = $src);
                    DELETE FROM relation WHERE doc_id IN (SELECT DISTINCT doc_id FROM chunk WHERE metadata.source = $src);
                    "#,
                )
                .bind(("src", pdf_path.clone()))
                .await;
        }

/// 업로드 쿼리 파라미터(파일명 전달)
#[derive(Debug, Deserialize)]
pub struct UploadQuery { pub filename: Option<String> }

#[utoipa::path(
    tag = "admin",
    post,
    path = "/api/admin/upload",
    responses(
        (status = 200, description = "업로드 결과", body = UploadResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 500, description = "서버 오류"),
    )
)]
#[post("/api/admin/upload")]
pub async fn upload_file(q: web::Query<UploadQuery>, body: web::Bytes) -> Result<web::Json<UploadResponse>, Error> {
    // 업로드 디렉터리 생성(없으면 생성)
    let mut dir = PathBuf::from("uploads");
    if let Err(e) = fs::create_dir_all(&dir).await { return Err(Error::External(format!("업로드 디렉터리 생성 실패: {}", e))); }

    // 파일명 결정: 쿼리 filename 또는 UUID
    let fname = q.filename.clone().unwrap_or_else(|| format!("{}.bin", Uuid::new_v4()));
    dir.push(fname);
    let path = dir;

    // 파일 저장
    if let Err(e) = fs::write(&path, body).await { return Err(Error::External(format!("파일 저장 실패: {}", e))); }
    let meta = fs::metadata(&path).await.map_err(|e| Error::External(e.to_string()))?;

    let resp = UploadResponse { path: path.to_string_lossy().to_string(), size: meta.len() };
    Ok(web::Json(resp))
}

        // 2) PDF 처리 → 청킹
        let chunks = match pdf_processor::process_pdf(pdf_path) {
            Ok(v) => v,
            Err(e) => {
                item.error = Some(format!("PDF 처리 실패: {}", e));
                results.push(item);
                continue;
            }
        };

        // 3) 엔티티/관계 추출
        let ner = RegexNer::default();
        let entities = graph_builder::extract_entities_with(&ner, &chunks);
        let relations = graph_builder::infer_relations(&chunks, &entities);

        // 4) 임베딩 생성
        let chunk_embeddings: Vec<Embeddings3> = if use_tfidf {
            match embedding::embed_chunks_e3(&chunks, EmbeddingMode::Tfidf) {
                Ok(v) => v,
                Err(e) => {
                    item.error = Some(format!("TF-IDF 임베딩 실패: {}", e));
                    results.push(item);
                    continue;
                }
            }
        } else {
            let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
            let refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
            let sems = match state.azure.embed(&refs).await {
                Ok(v) => v,
                Err(e) => {
                    item.error = Some(format!("Azure 임베딩 실패: {}", e));
                    results.push(item);
                    continue;
                }
            };
            chunks.iter().enumerate().map(|(i, ch)| Embeddings3 {
                semantic: sems.get(i).cloned().unwrap_or_default(),
                structural: vec![ch.level as f32, i as f32 / chunks.len().max(1) as f32],
                functional: vec![ch.content.chars().count() as f32],
            }).collect()
        };

        let entity_texts: Vec<String> = entities.iter().map(|e| e.name.clone()).collect();
        let relation_texts: Vec<String> = relations
            .iter()
            .map(|r| format!("{} {} {}", r.subject, r.predicate, r.object))
            .collect();
        let entity_embeddings: Vec<Embeddings3> = if use_tfidf {
            match embedding::embed_texts_e3(&entity_texts, EmbeddingMode::Tfidf) {
                Ok(v) => v,
                Err(e) => {
                    item.error = Some(format!("TF-IDF 엔티티 임베딩 실패: {}", e));
                    results.push(item);
                    continue;
                }
            }
        } else {
            let refs: Vec<&str> = entity_texts.iter().map(|s| s.as_str()).collect();
            let sems = match state.azure.embed(&refs).await { Ok(v) => v, Err(e) => { item.error = Some(e.to_string()); results.push(item); continue; } };
            entity_texts.iter().enumerate().map(|(i, t)| Embeddings3 {
                semantic: sems.get(i).cloned().unwrap_or_default(),
                structural: vec![i as f32 / entity_texts.len().max(1) as f32],
                functional: vec![t.chars().count() as f32],
            }).collect()
        };
        let relation_embeddings: Vec<Embeddings3> = if use_tfidf {
            match embedding::embed_texts_e3(&relation_texts, EmbeddingMode::Tfidf) {
                Ok(v) => v,
                Err(e) => {
                    item.error = Some(format!("TF-IDF 관계 임베딩 실패: {}", e));
                    results.push(item);
                    continue;
                }
            }
        } else {
            let refs: Vec<&str> = relation_texts.iter().map(|s| s.as_str()).collect();
            let sems = match state.azure.embed(&refs).await { Ok(v) => v, Err(e) => { item.error = Some(e.to_string()); results.push(item); continue; } };
            relation_texts.iter().enumerate().map(|(i, t)| Embeddings3 {
                semantic: sems.get(i).cloned().unwrap_or_default(),
                structural: vec![i as f32 / relation_texts.len().max(1) as f32],
                functional: vec![t.chars().count() as f32],
            }).collect()
        };

        // 5) 문서 ID: 파일명 기반(동일 파일 재인덱싱 시 동일 ID를 기대)
        let doc_id = std::path::Path::new(pdf_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("doc")
            .to_string();
        let title = doc_id.clone();

        let processed = ProcessedDocument {
            doc_id: doc_id.clone(),
            title,
            chunks,
            entities,
            relations,
            chunk_embeddings,
            entity_embeddings,
            relation_embeddings,
            embedding_type: if use_tfidf { "tfidf".into() } else { "azure".into() },
        };
        if let Err(e) = index_db::store_processed_document(&processed).await {
            item.error = Some(format!("DB 저장 실패: {}", e));
        } else {
            item.document_id = Some(doc_id);
            item.chunks_indexed = processed.chunks.len() as u32;
        }
        results.push(item);
    }

    let elapsed = t0.elapsed().as_secs_f32();
    Ok(web::Json(ReindexResponse { results, elapsed }))
}
