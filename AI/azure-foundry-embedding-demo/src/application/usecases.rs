use crate::application::ports::{EmbeddingRepositoryPort, EmbeddingServicePort};
use crate::domain::entities::Embedding;
use crate::domain::value_objects::SimilarityResult;
use anyhow::Result;
use std::sync::Arc;

/// 임베딩 생성 유스케이스
pub struct CreateEmbeddingUseCase {
    embedding_service: Arc<dyn EmbeddingServicePort>,
    embedding_repository: Arc<dyn EmbeddingRepositoryPort>,
}

impl CreateEmbeddingUseCase {
    /// 새로운 유스케이스 생성
    pub fn new(embedding_service: Arc<dyn EmbeddingServicePort>, embedding_repository: Arc<dyn EmbeddingRepositoryPort>) -> Self {
        Self {
            embedding_service,
            embedding_repository,
        }
    }

    /// 텍스트에 대한 임베딩 생성 및 저장
    pub async fn execute(&self, text: String) -> Result<Embedding> {
        // 1. 임베딩 생성
        let vector = self.embedding_service.generate_embedding(text.clone()).await?;

        // 2. 데이터베이스에 저장
        let embedding = self.embedding_repository.save(text, vector).await?;

        Ok(embedding)
    }

    /// 여러 텍스트에 대한 임베딩 생성 및 저장
    pub async fn execute_batch(&self, texts: Vec<String>) -> Result<Vec<Embedding>> {
        // 1. 임베딩 생성
        let vectors = self.embedding_service.generate_embeddings(texts.clone()).await?;

        // 2. 데이터베이스에 저장
        let mut embeddings = Vec::new();
        for (text, vector) in texts.into_iter().zip(vectors.into_iter()) {
            let embedding = self.embedding_repository.save(text, vector).await?;
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }
}

/// 유사도 검색 유스케이스
pub struct SearchSimilarEmbeddingsUseCase {
    embedding_service: Arc<dyn EmbeddingServicePort>,
    embedding_repository: Arc<dyn EmbeddingRepositoryPort>,
}

impl SearchSimilarEmbeddingsUseCase {
    /// 새로운 유스케이스 생성
    pub fn new(embedding_service: Arc<dyn EmbeddingServicePort>, embedding_repository: Arc<dyn EmbeddingRepositoryPort>) -> Self {
        Self {
            embedding_service,
            embedding_repository,
        }
    }

    /// 쿼리 텍스트와 유사한 임베딩 검색
    pub async fn execute(&self, query: String, limit: usize) -> Result<Vec<SimilarityResult>> {
        // 1. 쿼리 텍스트의 임베딩 생성
        let query_vector = self.embedding_service.generate_embedding(query).await?;

        // 2. 유사한 임베딩 검색
        let results = self.embedding_repository.find_similar(&query_vector, limit).await?;

        Ok(results)
    }
}

/// 임베딩 조회 유스케이스
pub struct GetEmbeddingUseCase {
    embedding_repository: Arc<dyn EmbeddingRepositoryPort>,
}

impl GetEmbeddingUseCase {
    /// 새로운 유스케이스 생성
    pub fn new(embedding_repository: Arc<dyn EmbeddingRepositoryPort>) -> Self {
        Self {
            embedding_repository,
        }
    }

    /// ID로 임베딩 조회
    pub async fn execute(&self, id: i64) -> Result<Option<Embedding>> { self.embedding_repository.find_by_id(id).await }

    /// 모든 임베딩 조회
    pub async fn execute_all(&self) -> Result<Vec<Embedding>> { self.embedding_repository.find_all().await }
}

/// 임베딩 삭제 유스케이스
pub struct DeleteEmbeddingUseCase {
    embedding_repository: Arc<dyn EmbeddingRepositoryPort>,
}

impl DeleteEmbeddingUseCase {
    /// 새로운 유스케이스 생성
    pub fn new(embedding_repository: Arc<dyn EmbeddingRepositoryPort>) -> Self {
        Self {
            embedding_repository,
        }
    }

    /// 임베딩 삭제
    pub async fn execute(&self, id: i64) -> Result<()> { self.embedding_repository.delete(id).await }
}
