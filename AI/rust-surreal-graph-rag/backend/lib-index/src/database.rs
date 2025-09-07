//! SurrealDB 저장/조회 모듈
//! - 문서/청크/엔티티/관계 저장
//! - 벡터 유사도 검색(간단 쿼리)

use anyhow::Result;
use lib_db::DB;

use crate::types::ProcessedDocument;

/// 전처리/임베딩이 완료된 문서를 SurrealDB에 저장한다.
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

    // 청크 저장
    for (i, ch) in doc.chunks.iter().enumerate() {
        let emb = doc.chunk_embeddings.get(i).cloned().unwrap_or_default();
        DB.query(
            r#"
            CREATE chunk SET 
                id = rand::uuid(),
                doc_id = $doc_id,
                embedding_type = $embedding_type,
                embedding_deployment = $embedding_deployment,
                index = $index,
                level = $level,
                kind = $kind,
                content = $content,
                embedding_semantic = $embedding_semantic,
                embedding_structural = $embedding_structural,
                embedding_functional = $embedding_functional,
                metadata = $metadata;
            "#,
        )
        .bind(("doc_id", doc.doc_id.clone()))
        .bind(("embedding_type", doc.embedding_type.clone()))
        .bind(("embedding_deployment", doc.embedding_deployment.clone()))
        .bind(("index", i as i64))
        .bind(("level", ch.level as i64))
        .bind(("kind", format!("{:?}", ch.kind)))
        .bind(("content", ch.content.clone()))
        .bind(("embedding_semantic", emb.semantic))
        .bind(("embedding_structural", emb.structural))
        .bind(("embedding_functional", emb.functional))
        .bind(("metadata", ch.metadata.clone()))
        .await?;
    }

    // 엔티티 저장(중복 가능, 단순 저장)
    for (i, e) in doc.entities.iter().enumerate() {
        let e_emb = doc.entity_embeddings.get(i).cloned().unwrap_or_default();
        DB.query(
            r#"
            CREATE entity SET id = rand::uuid(), doc_id = $doc_id, name = $name, type = $type,
                embedding_type = $embedding_type,
                embedding_deployment = $embedding_deployment,
                embedding_semantic = $embedding_semantic,
                embedding_structural = $embedding_structural,
                embedding_functional = $embedding_functional;
            "#,
        )
        .bind(("doc_id", doc.doc_id.clone()))
        .bind(("name", e.name.clone()))
        .bind(("type", e.r#type.clone()))
        .bind(("embedding_type", doc.embedding_type.clone()))
        .bind(("embedding_deployment", doc.embedding_deployment.clone()))
        .bind(("embedding_semantic", e_emb.semantic))
        .bind(("embedding_structural", e_emb.structural))
        .bind(("embedding_functional", e_emb.functional))
        .await?;
    }

    // 관계 저장
    for (i, r) in doc.relations.iter().enumerate() {
        let r_emb = doc.relation_embeddings.get(i).cloned().unwrap_or_default();
        DB.query(
            r#"
            CREATE relation SET id = rand::uuid(), doc_id = $doc_id, subject = $s, predicate = $p, object = $o, weight = $w,
                embedding_type = $embedding_type,
                embedding_deployment = $embedding_deployment,
                embedding_semantic = $embedding_semantic,
                embedding_structural = $embedding_structural,
                embedding_functional = $embedding_functional;
            "#,
        )
        .bind(("doc_id", doc.doc_id.clone()))
        .bind(("s", r.subject.clone()))
        .bind(("p", r.predicate.clone()))
        .bind(("o", r.object.clone()))
        .bind(("w", r.weight))
        .bind(("embedding_type", doc.embedding_type.clone()))
        .bind(("embedding_deployment", doc.embedding_deployment.clone()))
        .bind(("embedding_semantic", r_emb.semantic))
        .bind(("embedding_structural", r_emb.structural))
        .bind(("embedding_functional", r_emb.functional))
        .await?;
    }

    Ok(())
}

/// 간단한 벡터 유사도 검색(코사인 근사): SurrealDB가 벡터 연산을 직접 지원하지 않는 경우,
/// 서버측 함수 또는 외부 서비스로 대체해야 한다. 여기서는 스텁.
pub async fn vector_search(_query_vec: &[f32], _top_k: usize) -> Result<Vec<serde_json::Value>> {
    // TODO: 실제 구현 시 SurrealDB 함수/플러그인 또는 애플리케이션 레벨 유사도 검색
    Ok(vec![])
}
