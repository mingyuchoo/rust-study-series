//! 관리자용 재인덱싱 엔드포인트
//! - 기존 데이터 정리(옵션) → PDF 재처리 → 그래프/임베딩 저장
//! - 임베딩 타입은 Azure 단일 모드로 저장(embedding_type = "azure")

use actix_web::{Result, post, web};
use std::time::Instant;

use crate::error::Error;
use crate::models::UploadResponse;
use crate::models::{ReindexItemResult, ReindexRequest, ReindexResponse};
use crate::search::AppState;
use lib_db::DB;
use lib_index::{
    RegexNer, database as index_db,
    graph_builder, pdf_processor,
    types::{Embeddings3, ProcessedDocument},
};
use log::{debug, error, info};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

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
pub async fn reindex_pdfs(state: web::Data<AppState>, payload: web::Json<ReindexRequest>) -> Result<web::Json<ReindexResponse>, Error> {
    let t0 = Instant::now();
    let clear_existing = payload.clear_existing.unwrap_or(false);

    info!(
        "[reindex] 시작: 파일 수={}, clear_existing={}",
        payload.pdf_paths.len(),
        clear_existing
    );

    let mut results: Vec<ReindexItemResult> = Vec::new();

    for pdf_path in &payload.pdf_paths {
        info!("[reindex] 대상 시작: path={}", pdf_path);
        let path_exists = std::path::Path::new(pdf_path).exists();
        debug!("[reindex] 경로 존재 여부: path={}, exists={}", pdf_path, path_exists);
        let mut item = ReindexItemResult {
            pdf_path: pdf_path.clone(),
            document_id: None,
            chunks_indexed: 0,
            error: None,
        };
        // 1) 기존 데이터 정리(옵션): metadata.source = pdf_path
        if clear_existing {
            // chunk/entity/relation 모두 삭제
            debug!("[reindex] 기존 데이터 삭제 시도: source={}", pdf_path);
            let _ = DB
                .query(
                    r#"
                    LET $doc_ids = (SELECT VALUE doc_id FROM chunk WHERE metadata.source = $src);
                    DELETE FROM entity WHERE doc_id IN $doc_ids;
                    DELETE FROM relation WHERE doc_id IN $doc_ids;
                    DELETE FROM chunk WHERE metadata.source = $src;
                    "#,
                )
                .bind(("src", pdf_path.clone()))
                .await;
            debug!("[reindex] 기존 데이터 삭제 완료(오류 무시)");
        }

        // 2) PDF 처리 → 청킹
        debug!("[reindex] PDF 처리 시작: {}", pdf_path);
        let chunks = match pdf_processor::process_pdf(pdf_path) {
            | Ok(v) => v,
            | Err(e) => {
                error!("[reindex] PDF 처리 실패: path={}, error={}", pdf_path, e);
                item.error = Some(format!("PDF 처리 실패: {}", e));
                results.push(item);
                continue;
            },
        };
        debug!("[reindex] PDF 처리 완료: path={}, chunk_count={}", pdf_path, chunks.len());

        // 3) 엔티티/관계 추출
        let ner = RegexNer::default();
        debug!("[reindex] 엔티티/관계 추출 시작: path={}", pdf_path);
        let entities = graph_builder::extract_entities_with(&ner, &chunks);
        let relations = graph_builder::infer_relations(&chunks, &entities);
        debug!("[reindex] 엔티티/관계 추출 완료: entities={}, relations={}", entities.len(), relations.len());

        // 4) 임베딩 생성
        let chunk_embeddings: Vec<Embeddings3> = {
            let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
            let refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
            if refs.is_empty() {
                debug!("[reindex] 청크 임베딩(Azure) 생략: 입력 청크 0개");
                Vec::new()
            } else {
                debug!("[reindex] 청크 임베딩(Azure) 시작: texts={}", refs.len());
                let sems = match state.azure.embed(&refs).await {
                    | Ok(v) => {
                        debug!(
                            "[reindex] 청크 임베딩(Azure) 완료: count={}, dim={}",
                            v.len(),
                            v.get(0).map(|e| e.len()).unwrap_or(0)
                        );
                        v
                    },
                    | Err(e) => {
                        error!("[reindex] 청크 임베딩(Azure) 실패: path={}, error={}", pdf_path, e);
                        item.error = Some(format!("Azure 임베딩 실패: {}", e));
                        results.push(item);
                        continue;
                    },
                };
                chunks
                    .iter()
                    .enumerate()
                    .map(|(i, ch)| Embeddings3 {
                        semantic: sems.get(i).cloned().unwrap_or_default(),
                        structural: vec![ch.level as f32, i as f32 / chunks.len().max(1) as f32],
                        functional: vec![ch.content.chars().count() as f32],
                    })
                    .collect()
            }
        };

        let entity_texts: Vec<String> = entities.iter().map(|e| e.name.clone()).collect();
        let relation_texts: Vec<String> = relations.iter().map(|r| format!("{} {} {}", r.subject, r.predicate, r.object)).collect();
        let entity_embeddings: Vec<Embeddings3> = {
            let refs: Vec<&str> = entity_texts.iter().map(|s| s.as_str()).collect();
            if refs.is_empty() {
                debug!("[reindex] 엔티티 임베딩(Azure) 생략: 입력 엔티티 0개");
                Vec::new()
            } else {
                debug!("[reindex] 엔티티 임베딩(Azure) 시작: entities={}", refs.len());
                let sems = match state.azure.embed(&refs).await {
                    | Ok(v) => {
                        debug!(
                            "[reindex] 엔티티 임베딩(Azure) 완료: count={}, dim={}",
                            v.len(),
                            v.get(0).map(|e| e.len()).unwrap_or(0)
                        );
                        v
                    },
                    | Err(e) => {
                        error!("[reindex] 엔티티 임베딩(Azure) 실패: path={}, error={}", pdf_path, e);
                        item.error = Some(e.to_string());
                        results.push(item);
                        continue;
                    },
                };
                entity_texts
                    .iter()
                    .enumerate()
                    .map(|(i, t)| Embeddings3 {
                        semantic: sems.get(i).cloned().unwrap_or_default(),
                        structural: vec![i as f32 / entity_texts.len().max(1) as f32],
                        functional: vec![t.chars().count() as f32],
                    })
                    .collect()
            }
        };
        let relation_embeddings: Vec<Embeddings3> = {
            let refs: Vec<&str> = relation_texts.iter().map(|s| s.as_str()).collect();
            if refs.is_empty() {
                debug!("[reindex] 관계 임베딩(Azure) 생략: 입력 관계 0개");
                Vec::new()
            } else {
                debug!("[reindex] 관계 임베딩(Azure) 시작: relations={}", refs.len());
                let sems = match state.azure.embed(&refs).await {
                    | Ok(v) => {
                        debug!(
                            "[reindex] 관계 임베딩(Azure) 완료: count={}, dim={}",
                            v.len(),
                            v.get(0).map(|e| e.len()).unwrap_or(0)
                        );
                        v
                    },
                    | Err(e) => {
                        error!("[reindex] 관계 임베딩(Azure) 실패: path={}, error={}", pdf_path, e);
                        item.error = Some(e.to_string());
                        results.push(item);
                        continue;
                    },
                };
                relation_texts
                    .iter()
                    .enumerate()
                    .map(|(i, t)| Embeddings3 {
                        semantic: sems.get(i).cloned().unwrap_or_default(),
                        structural: vec![i as f32 / relation_texts.len().max(1) as f32],
                        functional: vec![t.chars().count() as f32],
                    })
                    .collect()
            }
        };

        // 5) 문서 ID: 파일명 기반(동일 파일 재인덱싱 시 동일 ID를 기대)
        let doc_id = std::path::Path::new(pdf_path).file_name().and_then(|s| s.to_str()).unwrap_or("doc").to_string();
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
            embedding_type: "azure".into(),
            embedding_deployment: state.azure.embed_deployment().to_string(),
        };
        if let Err(e) = index_db::store_processed_document(&processed).await {
            error!("[reindex] DB 저장 실패: path={}, error={}", pdf_path, e);
            item.error = Some(format!("DB 저장 실패: {}", e));
        } else {
            info!(
                "[reindex] 저장 완료: path={}, doc_id={}, chunks_indexed={}",
                pdf_path,
                doc_id,
                processed.chunks.len()
            );
            item.document_id = Some(doc_id);
            item.chunks_indexed = processed.chunks.len() as u32;
        }
        results.push(item);
    }

    let elapsed = t0.elapsed().as_secs_f32();
    let success = results.iter().filter(|r| r.error.is_none()).count();
    let failed = results.len().saturating_sub(success);
    info!(
        "[reindex] 완료: total={}, success={}, failed={}, elapsed={:.3}s",
        results.len(),
        success,
        failed,
        elapsed
    );
    Ok(web::Json(ReindexResponse { results, elapsed }))
}

/// 업로드 쿼리 파라미터(파일명 전달)
#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    pub filename: Option<String>,
}

/// 파일 업로드 엔드포인트: application/octet-stream 바디를 받아 서버 로컬 uploads/에 저장
#[post("/api/admin/upload")]
pub async fn upload_file(q: web::Query<UploadQuery>, body: web::Bytes) -> Result<web::Json<UploadResponse>, Error> {
    // 업로드 디렉터리 생성(없으면 생성)
    let mut dir = PathBuf::from("uploads");
    if let Err(e) = fs::create_dir_all(&dir).await {
        return Err(Error::External(format!("업로드 디렉터리 생성 실패: {}", e)));
    }

    // 파일명 결정: 쿼리 filename 또는 UUID
    let fname = q.filename.clone().unwrap_or_else(|| format!("{}.bin", Uuid::new_v4()));
    dir.push(fname);
    let path = dir;

    // 파일 저장
    if let Err(e) = fs::write(&path, body).await {
        return Err(Error::External(format!("파일 저장 실패: {}", e)));
    }
    let meta = fs::metadata(&path).await.map_err(|e| Error::External(e.to_string()))?;

    let resp = UploadResponse {
        path: path.to_string_lossy().to_string(),
        size: meta.len(),
    };
    Ok(web::Json(resp))
}
