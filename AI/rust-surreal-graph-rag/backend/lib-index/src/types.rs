//! 인덱싱 파이프라인에서 공용으로 사용하는 타입 정의

use serde::{Deserialize, Serialize};

/// 청크 종류(제목/섹션/문단/일반)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkKind {
    Title,
    Section,
    Paragraph,
    Text,
}

/// 계층 정보를 포함한 청크
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub content: String,
    pub level: u8, // 문단(0) → 섹션(1) → 챕터(2) → 문서(3)
    pub kind: ChunkKind,
    pub index: usize, // 문서 내 순서
    pub metadata: serde_json::Value,
}

/// 엔티티(인명/조직/장소/날짜 등)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub r#type: String,
}

/// 관계: 주어-술어-목적어 형태의 단순 표현
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub weight: f32,
}

/// 다중 관점 임베딩(semantic / structural / functional)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Embeddings3 {
    pub semantic: Vec<f32>,
    pub structural: Vec<f32>,
    pub functional: Vec<f32>,
}

/// 전처리 결과 집계 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDocument {
    pub doc_id: String,
    pub title: String,
    pub chunks: Vec<Chunk>,
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
    /// 청크별 다중 관점 임베딩
    pub chunk_embeddings: Vec<Embeddings3>,
    /// 엔티티별 다중 관점 임베딩
    pub entity_embeddings: Vec<Embeddings3>,
    /// 관계별 다중 관점 임베딩
    pub relation_embeddings: Vec<Embeddings3>,
    /// 임베딩 타입(혼재 운영 구분용): "azure" | "tfidf" 등
    pub embedding_type: String,
    /// 임베딩 배포명(모델 배포 식별자): 검색 시 동일 배포만 조회되도록 보장
    pub embedding_deployment: String,
}
