//! SurrealDB 저장/조회 모듈
//! - 문서/청크/엔티티/관계를 배치 저장

use anyhow::Result;
use lib_db::DB;
use serde_json::json;

use crate::types::ProcessedDocument;

/// 전처리/임베딩이 완료된 문서를 SurrealDB에 배치 저장한다.
pub async fn store_processed_document(doc: &ProcessedDocument) -> Result<()> {
    // 문서 노드
    DB.query(
        r#"
        CREATE document SET id = $doc_id, title = $title, created_at = time::now();
        "#,
    )
    .bind(("doc_id", doc.doc_id.clone()))
    .bind(("title", doc.title.clone()))
    .await?;

    // 청크 배치 저장
    if !doc.chunks.is_empty() {
        let chunk_records: Vec<serde_json::Value> = doc
            .chunks
            .iter()
            .enumerate()
            .map(|(i, ch)| {
                let emb = doc.chunk_embeddings.get(i).cloned().unwrap_or_default();
                json!({
                    "doc_id": doc.doc_id,
                    "embedding_type": doc.embedding_type,
                    "embedding_deployment": doc.embedding_deployment,
                    "index": i,
                    "level": ch.level,
                    "kind": format!("{:?}", ch.kind),
                    "content": ch.content,
                    "embedding_semantic": emb.semantic,
                    "embedding_structural": emb.structural,
                    "embedding_functional": emb.functional,
                    "metadata": ch.metadata,
                })
            })
            .collect();

        DB.query("INSERT INTO chunk $records;")
            .bind(("records", chunk_records))
            .await?;
    }

    // 엔티티 배치 저장
    if !doc.entities.is_empty() {
        let entity_records: Vec<serde_json::Value> = doc
            .entities
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let emb = doc.entity_embeddings.get(i).cloned().unwrap_or_default();
                json!({
                    "doc_id": doc.doc_id,
                    "name": e.name,
                    "type": e.r#type,
                    "embedding_type": doc.embedding_type,
                    "embedding_deployment": doc.embedding_deployment,
                    "embedding_semantic": emb.semantic,
                    "embedding_structural": emb.structural,
                    "embedding_functional": emb.functional,
                })
            })
            .collect();

        DB.query("INSERT INTO entity $records;")
            .bind(("records", entity_records))
            .await?;
    }

    // 관계 배치 저장
    if !doc.relations.is_empty() {
        let relation_records: Vec<serde_json::Value> = doc
            .relations
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let emb = doc.relation_embeddings.get(i).cloned().unwrap_or_default();
                json!({
                    "doc_id": doc.doc_id,
                    "subject": r.subject,
                    "predicate": r.predicate,
                    "object": r.object,
                    "weight": r.weight,
                    "embedding_type": doc.embedding_type,
                    "embedding_deployment": doc.embedding_deployment,
                    "embedding_semantic": emb.semantic,
                    "embedding_structural": emb.structural,
                    "embedding_functional": emb.functional,
                })
            })
            .collect();

        DB.query("INSERT INTO relation $records;")
            .bind(("records", relation_records))
            .await?;
    }

    Ok(())
}
