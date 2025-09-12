use crate::models::{DocumentChunk, SearchResult, ServiceError};
use crate::repository::VectorRepository;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, info};

#[async_trait]
#[allow(dead_code)]
pub trait VectorSearchService: Send + Sync {
    async fn search_similar(&self, query_embedding: Vec<f32>, limit: usize) -> Result<Vec<SearchResult>, ServiceError>;
    async fn search_similar_with_threshold(&self, query_embedding: Vec<f32>, limit: usize, score_threshold: f32) -> Result<Vec<SearchResult>, ServiceError>;
    async fn store_embeddings(&self, chunks: Vec<DocumentChunk>) -> Result<(), ServiceError>;
    async fn delete_document_embeddings(&self, document_id: &str) -> Result<(), ServiceError>;
    async fn get_collection_stats(&self) -> Result<VectorCollectionStats, ServiceError>;
}

/// Statistics about the vector collection
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VectorCollectionStats {
    pub total_vectors: u64,
    pub indexed_vectors: u64,
    pub collection_status: String,
}

pub struct VectorSearchServiceImpl {
    vector_repository: Arc<dyn VectorRepository>,
    #[allow(dead_code)]
    default_score_threshold: f32,
    max_search_limit: usize,
}

impl VectorSearchServiceImpl {
    pub fn new(vector_repository: Arc<dyn VectorRepository>) -> Self {
        Self {
            vector_repository,
            default_score_threshold: 0.7, // Default similarity threshold
            max_search_limit: 100,        // Maximum number of results to return
        }
    }

    #[allow(dead_code)]
    pub fn with_config(vector_repository: Arc<dyn VectorRepository>, default_score_threshold: f32, max_search_limit: usize) -> Self {
        Self {
            vector_repository,
            default_score_threshold,
            max_search_limit,
        }
    }

    /// Validates search parameters
    fn validate_search_params(&self, query_embedding: &[f32], limit: usize) -> Result<(), ServiceError> {
        if query_embedding.is_empty() {
            return Err(ServiceError::validation("Query embedding cannot be empty"));
        }

        if limit == 0 {
            return Err(ServiceError::validation("Search limit must be greater than 0"));
        }

        if limit > self.max_search_limit {
            return Err(ServiceError::validation(format!(
                "Search limit {} exceeds maximum allowed limit {}",
                limit, self.max_search_limit
            )));
        }

        Ok(())
    }

    /// Validates score threshold
    fn validate_score_threshold(&self, threshold: f32) -> Result<(), ServiceError> {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(ServiceError::validation(format!(
                "Score threshold must be between 0.0 and 1.0, got: {}",
                threshold
            )));
        }
        Ok(())
    }

    /// Validates chunks before storage
    #[allow(dead_code)]
    fn validate_chunks(&self, chunks: &[DocumentChunk]) -> Result<(), ServiceError> {
        if chunks.is_empty() {
            return Ok(()); // Empty chunks is valid, just a no-op
        }

        for (i, chunk) in chunks.iter().enumerate() {
            if chunk.id.trim().is_empty() {
                return Err(ServiceError::validation(format!("Chunk at index {} has empty ID", i)));
            }

            if chunk.document_id.trim().is_empty() {
                return Err(ServiceError::validation(format!("Chunk at index {} has empty document ID", i)));
            }

            if chunk.content.trim().is_empty() {
                return Err(ServiceError::validation(format!("Chunk at index {} has empty content", i)));
            }

            if chunk.embedding.is_none() {
                return Err(ServiceError::validation(format!("Chunk at index {} is missing embedding", i)));
            }

            if let Some(embedding) = &chunk.embedding {
                if embedding.is_empty() {
                    return Err(ServiceError::validation(format!("Chunk at index {} has empty embedding", i)));
                }
            }
        }

        Ok(())
    }

    /// Filters and ranks search results
    fn filter_and_rank_results(&self, mut results: Vec<SearchResult>, score_threshold: Option<f32>) -> Vec<SearchResult> {
        // Apply score threshold if provided
        if let Some(threshold) = score_threshold {
            results.retain(|result| result.relevance_score >= threshold);
        }

        // Sort by relevance score (descending)
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));

        // Log filtering results
        if let Some(threshold) = score_threshold {
            debug!("Filtered {} results with score threshold {}", results.len(), threshold);
        }

        results
    }
}

#[async_trait]
impl VectorSearchService for VectorSearchServiceImpl {
    async fn search_similar(&self, query_embedding: Vec<f32>, limit: usize) -> Result<Vec<SearchResult>, ServiceError> {
        debug!("VectorSearchService: searching for similar vectors with limit {}", limit);

        self.validate_search_params(&query_embedding, limit)?;

        let results = self
            .vector_repository
            .search_similar(query_embedding, limit, Some(self.default_score_threshold))
            .await
            .map_err(|e| ServiceError::vector_search(format!("Failed to search similar vectors: {}", e)))?;

        let filtered_results = self.filter_and_rank_results(results, Some(self.default_score_threshold));

        info!("VectorSearchService: found {} similar vectors", filtered_results.len());
        Ok(filtered_results)
    }

    async fn search_similar_with_threshold(&self, query_embedding: Vec<f32>, limit: usize, score_threshold: f32) -> Result<Vec<SearchResult>, ServiceError> {
        debug!(
            "VectorSearchService: searching for similar vectors with limit {} and threshold {}",
            limit, score_threshold
        );

        self.validate_search_params(&query_embedding, limit)?;
        self.validate_score_threshold(score_threshold)?;

        let results = self
            .vector_repository
            .search_similar(query_embedding, limit, Some(score_threshold))
            .await
            .map_err(|e| ServiceError::vector_search(format!("Failed to search similar vectors: {}", e)))?;

        let filtered_results = self.filter_and_rank_results(results, Some(score_threshold));

        info!(
            "VectorSearchService: found {} similar vectors with threshold {}",
            filtered_results.len(),
            score_threshold
        );
        Ok(filtered_results)
    }

    async fn store_embeddings(&self, chunks: Vec<DocumentChunk>) -> Result<(), ServiceError> {
        debug!("VectorSearchService: storing {} chunks with embeddings", chunks.len());

        self.validate_chunks(&chunks)?;

        if chunks.is_empty() {
            debug!("No chunks to store, skipping");
            return Ok(());
        }

        // Verify all chunks have embeddings
        let chunks_without_embeddings: Vec<_> = chunks
            .iter()
            .enumerate()
            .filter(|(_, chunk)| chunk.embedding.is_none())
            .map(|(i, _)| i)
            .collect();

        if !chunks_without_embeddings.is_empty() {
            return Err(ServiceError::validation(format!(
                "Chunks at indices {:?} are missing embeddings",
                chunks_without_embeddings
            )));
        }

        self.vector_repository
            .store_chunks(chunks.clone())
            .await
            .map_err(|e| ServiceError::vector_search(format!("Failed to store embeddings: {}", e)))?;

        info!("VectorSearchService: successfully stored {} chunks", chunks.len());
        Ok(())
    }

    async fn delete_document_embeddings(&self, document_id: &str) -> Result<(), ServiceError> {
        debug!("VectorSearchService: deleting embeddings for document ID: {}", document_id);

        if document_id.trim().is_empty() {
            return Err(ServiceError::validation("Document ID cannot be empty"));
        }

        self.vector_repository
            .delete_chunks_by_document_id(document_id)
            .await
            .map_err(|e| ServiceError::vector_search(format!("Failed to delete document embeddings: {}", e)))?;

        info!("VectorSearchService: successfully deleted embeddings for document ID: {}", document_id);
        Ok(())
    }

    async fn get_collection_stats(&self) -> Result<VectorCollectionStats, ServiceError> {
        debug!("VectorSearchService: retrieving collection statistics");

        let collection_info = self
            .vector_repository
            .get_collection_info()
            .await
            .map_err(|e| ServiceError::vector_search(format!("Failed to get collection info: {}", e)))?;

        let stats = VectorCollectionStats {
            total_vectors: collection_info.points_count.unwrap_or(0),
            indexed_vectors: collection_info.indexed_vectors_count.unwrap_or(0),
            collection_status: format!("{:?}", collection_info.status()),
        };

        info!(
            "VectorSearchService: collection has {} total vectors, {} indexed",
            stats.total_vectors, stats.indexed_vectors
        );

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ChunkMetadata, ChunkType};
    use crate::repository::VectorRepository;
    use async_trait::async_trait;
    use chrono::Utc;
    use std::collections::HashMap;
    use tokio::sync::Mutex;
    use uuid::Uuid;

    // Mock vector repository for testing
    struct MockVectorRepository {
        stored_chunks: Arc<Mutex<HashMap<String, Vec<DocumentChunk>>>>,
        search_results: Arc<Mutex<Vec<SearchResult>>>,
        should_fail: bool,
        fail_with_error: Option<ServiceError>,
    }

    impl MockVectorRepository {
        fn new() -> Self {
            Self {
                stored_chunks: Arc::new(Mutex::new(HashMap::new())),
                search_results: Arc::new(Mutex::new(Vec::new())),
                should_fail: false,
                fail_with_error: None,
            }
        }

        fn with_failure(error: ServiceError) -> Self {
            Self {
                stored_chunks: Arc::new(Mutex::new(HashMap::new())),
                search_results: Arc::new(Mutex::new(Vec::new())),
                should_fail: true,
                fail_with_error: Some(error),
            }
        }

        async fn set_search_results(&self, results: Vec<SearchResult>) {
            let mut search_results = self.search_results.lock().await;
            *search_results = results;
        }

        async fn get_stored_chunks_count(&self) -> usize {
            let chunks = self.stored_chunks.lock().await;
            chunks.values().map(|v| v.len()).sum()
        }
    }

    #[async_trait]
    impl VectorRepository for MockVectorRepository {
        async fn initialize_collection(&self) -> Result<(), ServiceError> {
            if self.should_fail {
                return Err(self.fail_with_error.clone().unwrap_or_else(|| ServiceError::database("Mock failure")));
            }
            Ok(())
        }

        async fn collection_exists(&self) -> Result<bool, ServiceError> {
            Ok(true)
        }

        async fn store_chunks(&self, chunks: Vec<DocumentChunk>) -> Result<(), ServiceError> {
            if self.should_fail {
                return Err(self.fail_with_error.clone().unwrap_or_else(|| ServiceError::database("Mock store failure")));
            }

            let mut stored = self.stored_chunks.lock().await;
            for chunk in chunks {
                stored.entry(chunk.document_id.clone()).or_insert_with(Vec::new).push(chunk);
            }
            Ok(())
        }

        async fn search_similar(&self, _query_embedding: Vec<f32>, limit: usize, _score_threshold: Option<f32>) -> Result<Vec<SearchResult>, ServiceError> {
            if self.should_fail {
                return Err(self
                    .fail_with_error
                    .clone()
                    .unwrap_or_else(|| ServiceError::vector_search("Mock search failure")));
            }

            let search_results = self.search_results.lock().await;
            let mut results = search_results.clone();
            results.truncate(limit);
            Ok(results)
        }

        async fn get_chunks_by_document_id(&self, document_id: &str) -> Result<Vec<DocumentChunk>, ServiceError> {
            let stored = self.stored_chunks.lock().await;
            Ok(stored.get(document_id).cloned().unwrap_or_default())
        }

        async fn delete_chunks_by_document_id(&self, document_id: &str) -> Result<(), ServiceError> {
            if self.should_fail {
                return Err(self.fail_with_error.clone().unwrap_or_else(|| ServiceError::database("Mock delete failure")));
            }

            let mut stored = self.stored_chunks.lock().await;
            stored.remove(document_id);
            Ok(())
        }

        async fn delete_chunk(&self, _chunk_id: &str) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn get_collection_info(&self) -> Result<qdrant_client::qdrant::CollectionInfo, ServiceError> {
            if self.should_fail {
                return Err(self.fail_with_error.clone().unwrap_or_else(|| ServiceError::database("Mock info failure")));
            }

            let stored = self.stored_chunks.lock().await;
            let total_count = stored.values().map(|v| v.len()).sum::<usize>() as u64;

            Ok(qdrant_client::qdrant::CollectionInfo {
                status: 1, // Green status
                optimizer_status: None,
                vectors_count: Some(total_count),
                indexed_vectors_count: Some(total_count),
                points_count: Some(total_count),
                segments_count: 1,
                config: None,
                payload_schema: HashMap::new(),
            })
        }

        async fn health_check(&self) -> Result<bool, ServiceError> {
            Ok(!self.should_fail)
        }
    }

    fn create_test_chunk(document_id: &str, content: &str, embedding: Vec<f32>) -> DocumentChunk {
        let metadata = ChunkMetadata::new("test.md".to_string(), 0, ChunkType::Text);

        DocumentChunk {
            id: Uuid::new_v4().to_string(),
            document_id: document_id.to_string(),
            content: content.to_string(),
            metadata,
            embedding: Some(embedding),
            created_at: Utc::now(),
        }
    }

    fn create_test_search_result(content: &str, score: f32) -> SearchResult {
        let chunk = create_test_chunk("test-doc", content, vec![0.1; 384]);
        SearchResult::new(chunk, score)
    }

    #[tokio::test]
    async fn test_search_similar_success() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo.clone());

        // Set up mock search results
        let mock_results = vec![create_test_search_result("First result", 0.9), create_test_search_result("Second result", 0.8)];
        mock_repo.set_search_results(mock_results).await;

        let query_embedding = vec![0.5; 384];
        let result = service.search_similar(query_embedding, 5).await;

        assert!(result.is_ok(), "Search should succeed");
        let results = result.unwrap();
        assert_eq!(results.len(), 2, "Should return mock results");
        assert!(results[0].relevance_score >= results[1].relevance_score, "Results should be sorted by score");
    }

    #[tokio::test]
    async fn test_search_similar_empty_embedding() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let result = service.search_similar(vec![], 5).await;

        assert!(result.is_err(), "Empty embedding should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_search_similar_zero_limit() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let query_embedding = vec![0.5; 384];
        let result = service.search_similar(query_embedding, 0).await;

        assert!(result.is_err(), "Zero limit should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_search_similar_limit_too_high() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let query_embedding = vec![0.5; 384];
        let result = service.search_similar(query_embedding, 1000).await; // Exceeds max_search_limit

        assert!(result.is_err(), "Limit too high should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_search_similar_with_threshold() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo.clone());

        // Set up mock results with different scores
        let mock_results = vec![
            create_test_search_result("High score result", 0.9),
            create_test_search_result("Medium score result", 0.6),
            create_test_search_result("Low score result", 0.3),
        ];
        mock_repo.set_search_results(mock_results).await;

        let query_embedding = vec![0.5; 384];
        let result = service.search_similar_with_threshold(query_embedding, 10, 0.7).await;

        assert!(result.is_ok(), "Search with threshold should succeed");
        let results = result.unwrap();

        // Should only return results above threshold
        for result in &results {
            assert!(result.relevance_score >= 0.7, "All results should be above threshold");
        }
    }

    #[tokio::test]
    async fn test_search_similar_invalid_threshold() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let query_embedding = vec![0.5; 384];

        // Test negative threshold
        let result = service.search_similar_with_threshold(query_embedding.clone(), 5, -0.1).await;
        assert!(result.is_err(), "Negative threshold should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));

        // Test threshold > 1.0
        let result = service.search_similar_with_threshold(query_embedding, 5, 1.5).await;
        assert!(result.is_err(), "Threshold > 1.0 should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_store_embeddings_success() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo.clone());

        let chunks = vec![
            create_test_chunk("doc1", "Content 1", vec![0.1; 384]),
            create_test_chunk("doc1", "Content 2", vec![0.2; 384]),
        ];

        let result = service.store_embeddings(chunks).await;

        assert!(result.is_ok(), "Store embeddings should succeed");
        assert_eq!(mock_repo.get_stored_chunks_count().await, 2, "Should store all chunks");
    }

    #[tokio::test]
    async fn test_store_embeddings_empty() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let result = service.store_embeddings(vec![]).await;

        assert!(result.is_ok(), "Storing empty chunks should succeed");
    }

    #[tokio::test]
    async fn test_store_embeddings_missing_embedding() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let mut chunk = create_test_chunk("doc1", "Content", vec![0.1; 384]);
        chunk.embedding = None; // Remove embedding

        let result = service.store_embeddings(vec![chunk]).await;

        assert!(result.is_err(), "Chunk without embedding should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_store_embeddings_empty_id() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let mut chunk = create_test_chunk("doc1", "Content", vec![0.1; 384]);
        chunk.id = "".to_string(); // Empty ID

        let result = service.store_embeddings(vec![chunk]).await;

        assert!(result.is_err(), "Chunk with empty ID should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_store_embeddings_empty_content() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let mut chunk = create_test_chunk("doc1", "", vec![0.1; 384]);
        chunk.content = "   ".to_string(); // Whitespace-only content

        let result = service.store_embeddings(vec![chunk]).await;

        assert!(result.is_err(), "Chunk with empty content should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_delete_document_embeddings_success() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo.clone());

        // First store some chunks
        let chunks = vec![create_test_chunk("doc1", "Content", vec![0.1; 384])];
        service.store_embeddings(chunks).await.unwrap();

        // Then delete them
        let result = service.delete_document_embeddings("doc1").await;

        assert!(result.is_ok(), "Delete should succeed");
    }

    #[tokio::test]
    async fn test_delete_document_embeddings_empty_id() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo);

        let result = service.delete_document_embeddings("").await;

        assert!(result.is_err(), "Empty document ID should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_collection_stats_success() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo.clone());

        // Store some chunks first
        let chunks = vec![
            create_test_chunk("doc1", "Content 1", vec![0.1; 384]),
            create_test_chunk("doc2", "Content 2", vec![0.2; 384]),
        ];
        service.store_embeddings(chunks).await.unwrap();

        let result = service.get_collection_stats().await;

        assert!(result.is_ok(), "Get stats should succeed");
        let stats = result.unwrap();
        assert_eq!(stats.total_vectors, 2, "Should report correct vector count");
        assert_eq!(stats.indexed_vectors, 2, "Should report correct indexed count");
    }

    #[tokio::test]
    async fn test_repository_failure_propagation() {
        let mock_repo = Arc::new(MockVectorRepository::with_failure(ServiceError::database("Repository is down")));
        let service = VectorSearchServiceImpl::new(mock_repo);

        // Test search failure
        let query_embedding = vec![0.5; 384];
        let search_result = service.search_similar(query_embedding, 5).await;
        assert!(search_result.is_err(), "Repository failure should be propagated");

        // Test store failure
        let chunks = vec![create_test_chunk("doc1", "Content", vec![0.1; 384])];
        let store_result = service.store_embeddings(chunks).await;
        assert!(store_result.is_err(), "Repository failure should be propagated");

        // Test delete failure
        let delete_result = service.delete_document_embeddings("doc1").await;
        assert!(delete_result.is_err(), "Repository failure should be propagated");

        // Test stats failure
        let stats_result = service.get_collection_stats().await;
        assert!(stats_result.is_err(), "Repository failure should be propagated");
    }

    #[tokio::test]
    async fn test_custom_configuration() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::with_config(
            mock_repo.clone(),
            0.5, // Custom threshold
            50,  // Custom max limit
        );

        // Test custom max limit
        let query_embedding = vec![0.5; 384];
        let result = service.search_similar(query_embedding, 75).await; // Exceeds custom limit
        assert!(result.is_err(), "Should respect custom max limit");

        // Test custom threshold
        let mock_results = vec![create_test_search_result("High score", 0.8), create_test_search_result("Low score", 0.3)];
        mock_repo.set_search_results(mock_results).await;

        let query_embedding = vec![0.5; 384];
        let result = service.search_similar(query_embedding, 10).await;
        assert!(result.is_ok(), "Search should succeed");

        let results = result.unwrap();
        for result in &results {
            assert!(result.relevance_score >= 0.5, "Should use custom threshold");
        }
    }

    #[tokio::test]
    async fn test_result_filtering_and_ranking() {
        let mock_repo = Arc::new(MockVectorRepository::new());
        let service = VectorSearchServiceImpl::new(mock_repo.clone());

        // Set up results in random order
        let mock_results = vec![
            create_test_search_result("Medium score", 0.7),
            create_test_search_result("High score", 0.9),
            create_test_search_result("Low score", 0.5),
            create_test_search_result("Highest score", 0.95),
        ];
        mock_repo.set_search_results(mock_results).await;

        let query_embedding = vec![0.5; 384];
        let result = service.search_similar_with_threshold(query_embedding, 10, 0.6).await;

        assert!(result.is_ok(), "Search should succeed");
        let results = result.unwrap();

        // Should filter out low score (0.5) and sort by score descending
        assert_eq!(results.len(), 3, "Should filter out results below threshold");
        assert!(results[0].relevance_score >= results[1].relevance_score, "Should be sorted by score");
        assert!(results[1].relevance_score >= results[2].relevance_score, "Should be sorted by score");
        assert_eq!(results[0].relevance_score, 0.95, "Highest score should be first");
    }
}
