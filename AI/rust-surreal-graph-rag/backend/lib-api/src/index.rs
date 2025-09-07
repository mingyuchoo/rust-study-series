//! 인덱싱 생성 엔드포인트 (MVP)

use actix_web::{Result, post, web};
use serde_json::json;
use std::time::Instant;

use crate::error::Error;
use crate::models::{IndexChunkInput, IndexCreateRequest, IndexCreateResponse};
use crate::search::AppState;
use lib_index::{
    RegexNer, database as index_db,
    embedding::{self, EmbeddingMode},
    graph_builder, pdf_processor,
    types::{Chunk, ChunkKind, Embeddings3, ProcessedDocument},
};

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
pub async fn index_create(state: web::Data<AppState>, payload: web::Json<IndexCreateRequest>) -> Result<web::Json<IndexCreateResponse>, Error> {
    let t0 = Instant::now();

    let doc_id = payload.document_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let title = payload.title.clone().unwrap_or_else(|| "Untitled".to_string());

    // 0) PDF 경로가 제공된 경우: 서버 사이드에서 PDF 처리 파이프라인 수행
    if let Some(pdf_path) = payload.pdf_path.clone() {
        // a) PDF → 청킹
        let chunks_h = pdf_processor::process_pdf(&pdf_path).map_err(|e| Error::External(format!("PDF 처리 실패: {}", e)))?;

        // b) 엔티티/관계 추출 (NER Trait 사용)
        let ner = RegexNer::default();
        let entities = graph_builder::extract_entities_with(&ner, &chunks_h);
        let relations = graph_builder::infer_relations(&chunks_h, &entities);

        // c) 임베딩 생성: TF-IDF 또는 Azure 외부 임베딩 (다중 관점)
        let chunk_embeddings: Vec<Embeddings3> = if payload.use_tfidf {
            embedding::embed_chunks_e3(&chunks_h, EmbeddingMode::Tfidf).map_err(|e| Error::External(format!("TF-IDF 임베딩 실패: {}", e)))?
        } else {
            // Azure 임베딩은 semantic으로 사용, structural/functional은 휴리스틱
            let texts: Vec<String> = chunks_h.iter().map(|c| c.content.clone()).collect();
            let refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
            let sems = state.azure.embed(&refs).await.map_err(|e| Error::External(e.to_string()))?;
            // 래핑
            chunks_h
                .iter()
                .enumerate()
                .map(|(i, ch)| Embeddings3 {
                    semantic: sems.get(i).cloned().unwrap_or_default(),
                    structural: vec![ch.level as f32, i as f32 / chunks_h.len().max(1) as f32],
                    functional: vec![ch.content.chars().count() as f32],
                })
                .collect()
        };

        // c-1) 엔티티/관계 임베딩
        let entity_texts: Vec<String> = entities.iter().map(|e| e.name.clone()).collect();
        let relation_texts: Vec<String> = relations.iter().map(|r| format!("{} {} {}", r.subject, r.predicate, r.object)).collect();
        let entity_embeddings: Vec<Embeddings3> = if payload.use_tfidf {
            embedding::embed_texts_e3(&entity_texts, EmbeddingMode::Tfidf).map_err(|e| Error::External(format!("TF-IDF 엔티티 임베딩 실패: {}", e)))?
        } else {
            let refs: Vec<&str> = entity_texts.iter().map(|s| s.as_str()).collect();
            let sems = state.azure.embed(&refs).await.map_err(|e| Error::External(e.to_string()))?;
            entity_texts
                .iter()
                .enumerate()
                .map(|(i, t)| Embeddings3 {
                    semantic: sems.get(i).cloned().unwrap_or_default(),
                    structural: vec![i as f32 / entity_texts.len().max(1) as f32],
                    functional: vec![t.chars().count() as f32],
                })
                .collect()
        };
        let relation_embeddings: Vec<Embeddings3> = if payload.use_tfidf {
            embedding::embed_texts_e3(&relation_texts, EmbeddingMode::Tfidf).map_err(|e| Error::External(format!("TF-IDF 관계 임베딩 실패: {}", e)))?
        } else {
            let refs: Vec<&str> = relation_texts.iter().map(|s| s.as_str()).collect();
            let sems = state.azure.embed(&refs).await.map_err(|e| Error::External(e.to_string()))?;
            relation_texts
                .iter()
                .enumerate()
                .map(|(i, t)| Embeddings3 {
                    semantic: sems.get(i).cloned().unwrap_or_default(),
                    structural: vec![i as f32 / relation_texts.len().max(1) as f32],
                    functional: vec![t.chars().count() as f32],
                })
                .collect()
        };

        // d) SurrealDB 저장(문서/청크/엔티티/관계/임베딩)
        let processed = ProcessedDocument {
            doc_id: doc_id.clone(),
            title: title.clone(),
            chunks: chunks_h,
            entities,
            relations,
            chunk_embeddings,
            entity_embeddings,
            relation_embeddings,
            embedding_type: if payload.use_tfidf { "tfidf".into() } else { "azure".into() },
            embedding_deployment: if payload.use_tfidf {
                "tfidf".to_string()
            } else {
                state.azure.embed_deployment().to_string()
            },
        };
        index_db::store_processed_document(&processed)
            .await
            .map_err(|e| Error::External(format!("DB 저장 실패: {}", e)))?;

        let elapsed = t0.elapsed().as_secs_f32();
        return Ok(web::Json(IndexCreateResponse {
            document_id: doc_id,
            chunks_indexed: processed.chunks.len() as u32,
            elapsed,
        }));
    }

    // 1) (클라이언트 청크 경로) 제공된 청크를 계층 정보가 없는 문단 수준으로 어댑트
    let chunk_inputs: Vec<IndexChunkInput> = payload.chunks.clone();
    let mut chunks_h: Vec<Chunk> = Vec::with_capacity(chunk_inputs.len());
    for (i, ch) in chunk_inputs.into_iter().enumerate() {
        chunks_h.push(Chunk {
            content: ch.content,
            level: 0, // 문단 수준으로 간주
            kind: ChunkKind::Paragraph,
            index: i,
            metadata: ch.metadata.unwrap_or_else(|| json!({})),
        });
    }

    // 2) 엔티티/관계 추출 (NER Trait 사용)
    let ner = RegexNer::default();
    let entities = graph_builder::extract_entities_with(&ner, &chunks_h);
    let relations = graph_builder::infer_relations(&chunks_h, &entities);

    // 3) 임베딩(청크/엔티티/관계)
    let chunk_embeddings: Vec<Embeddings3> = if payload.use_tfidf {
        embedding::embed_chunks_e3(&chunks_h, EmbeddingMode::Tfidf).map_err(|e| Error::External(format!("TF-IDF 임베딩 실패: {}", e)))?
    } else {
        let texts: Vec<String> = chunks_h.iter().map(|c| c.content.clone()).collect();
        let refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let sems = state.azure.embed(&refs).await.map_err(|e| Error::External(e.to_string()))?;
        chunks_h
            .iter()
            .enumerate()
            .map(|(i, ch)| Embeddings3 {
                semantic: sems.get(i).cloned().unwrap_or_default(),
                structural: vec![ch.level as f32, i as f32 / chunks_h.len().max(1) as f32],
                functional: vec![ch.content.chars().count() as f32],
            })
            .collect()
    };

    let entity_texts: Vec<String> = entities.iter().map(|e| e.name.clone()).collect();
    let relation_texts: Vec<String> = relations.iter().map(|r| format!("{} {} {}", r.subject, r.predicate, r.object)).collect();
    let entity_embeddings: Vec<Embeddings3> = if payload.use_tfidf {
        embedding::embed_texts_e3(&entity_texts, EmbeddingMode::Tfidf).map_err(|e| Error::External(format!("TF-IDF 엔티티 임베딩 실패: {}", e)))?
    } else {
        let refs: Vec<&str> = entity_texts.iter().map(|s| s.as_str()).collect();
        let sems = state.azure.embed(&refs).await.map_err(|e| Error::External(e.to_string()))?;
        entity_texts
            .iter()
            .enumerate()
            .map(|(i, t)| Embeddings3 {
                semantic: sems.get(i).cloned().unwrap_or_default(),
                structural: vec![i as f32 / entity_texts.len().max(1) as f32],
                functional: vec![t.chars().count() as f32],
            })
            .collect()
    };
    let relation_embeddings: Vec<Embeddings3> = if payload.use_tfidf {
        embedding::embed_texts_e3(&relation_texts, EmbeddingMode::Tfidf).map_err(|e| Error::External(format!("TF-IDF 관계 임베딩 실패: {}", e)))?
    } else {
        let refs: Vec<&str> = relation_texts.iter().map(|s| s.as_str()).collect();
        let sems = state.azure.embed(&refs).await.map_err(|e| Error::External(e.to_string()))?;
        relation_texts
            .iter()
            .enumerate()
            .map(|(i, t)| Embeddings3 {
                semantic: sems.get(i).cloned().unwrap_or_default(),
                structural: vec![i as f32 / relation_texts.len().max(1) as f32],
                functional: vec![t.chars().count() as f32],
            })
            .collect()
    };

    // 4) SurrealDB 저장
    let processed = ProcessedDocument {
        doc_id: doc_id.clone(),
        title: title.clone(),
        chunks: chunks_h,
        entities,
        relations,
        chunk_embeddings,
        entity_embeddings,
        relation_embeddings,
        embedding_type: if payload.use_tfidf { "tfidf".into() } else { "azure".into() },
        embedding_deployment: if payload.use_tfidf {
            "tfidf".to_string()
        } else {
            state.azure.embed_deployment().to_string()
        },
    };
    index_db::store_processed_document(&processed)
        .await
        .map_err(|e| Error::External(format!("DB 저장 실패: {}", e)))?;

    let elapsed = t0.elapsed().as_secs_f32();
    Ok(web::Json(IndexCreateResponse {
        document_id: doc_id,
        chunks_indexed: processed.chunks.len() as u32,
        elapsed,
    }))
}
