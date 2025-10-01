use crate::domain::entities::Embedding;
use crate::domain::value_objects::SimilarityResult;
use anyhow::Result;
use async_trait::async_trait;

/// 임베딩 서비스 포트 (외부 API 호출)
#[async_trait]
pub trait EmbeddingServicePort: Send + Sync {
    /// 텍스트 목록에 대한 임베딩 생성
    async fn generate_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>>;

    /// 단일 텍스트에 대한 임베딩 생성
    async fn generate_embedding(&self, text: String) -> Result<Vec<f32>>;
}

/// 임베딩 저장소 포트 (데이터베이스)
#[async_trait]
pub trait EmbeddingRepositoryPort: Send + Sync {
    /// 임베딩 저장
    async fn save(&self, text: String, vector: Vec<f32>) -> Result<Embedding>;

    /// ID로 임베딩 조회
    async fn find_by_id(&self, id: i64) -> Result<Option<Embedding>>;

    /// 모든 임베딩 조회
    async fn find_all(&self) -> Result<Vec<Embedding>>;

    /// 유사도 검색 (상위 N개)
    async fn find_similar(&self, vector: &[f32], limit: usize) -> Result<Vec<SimilarityResult>>;

    /// 임베딩 삭제
    async fn delete(&self, id: i64) -> Result<()>;
}
