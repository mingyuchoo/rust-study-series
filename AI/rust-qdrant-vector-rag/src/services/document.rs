use crate::models::{DocumentChunk, ServiceError};
use crate::repository::VectorRepository;
use crate::services::{ChunkingConfig, DocumentChunker, DocumentParser, EmbeddingService};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

pub type DocumentId = String;

#[async_trait]
pub trait DocumentService: Send + Sync {
    async fn process_document(&self, content: String, filename: String) -> Result<DocumentId, ServiceError>;
    async fn get_document_chunks(&self, doc_id: DocumentId) -> Result<Vec<DocumentChunk>, ServiceError>;
}

pub struct DocumentServiceImpl {
    parser: DocumentParser,
    chunker: DocumentChunker,
    embedding_service: Arc<dyn EmbeddingService>,
    vector_repository: Arc<dyn VectorRepository>,
}

impl DocumentServiceImpl {
    pub fn new(embedding_service: Arc<dyn EmbeddingService>, vector_repository: Arc<dyn VectorRepository>) -> Self {
        Self {
            parser: DocumentParser::new(),
            chunker: DocumentChunker::with_config(ChunkingConfig::default()),
            embedding_service,
            vector_repository,
        }
    }

    pub fn with_chunking_config(
        embedding_service: Arc<dyn EmbeddingService>,
        vector_repository: Arc<dyn VectorRepository>,
        chunking_config: ChunkingConfig,
    ) -> Self {
        Self {
            parser: DocumentParser::new(),
            chunker: DocumentChunker::with_config(chunking_config),
            embedding_service,
            vector_repository,
        }
    }

    /// Validates the input content and filename
    fn validate_input(&self, content: &str, filename: &str) -> Result<(), ServiceError> {
        if content.trim().is_empty() {
            return Err(ServiceError::validation("Document content cannot be empty"));
        }

        if filename.trim().is_empty() {
            return Err(ServiceError::validation("Filename cannot be empty"));
        }

        // Check if filename has markdown extension
        if !filename.to_lowercase().ends_with(".md") && !filename.to_lowercase().ends_with(".markdown") {
            warn!("File '{}' does not have a markdown extension, processing anyway", filename);
        }

        // Check content size (reasonable limit to prevent memory issues)
        const MAX_CONTENT_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if content.len() > MAX_CONTENT_SIZE {
            return Err(ServiceError::validation(format!(
                "Document content too large: {} bytes (max: {} bytes)",
                content.len(),
                MAX_CONTENT_SIZE
            )));
        }

        Ok(())
    }

    /// Processes chunks in batches to avoid overwhelming the embedding service
    async fn process_chunks_in_batches(&self, mut chunks: Vec<DocumentChunk>, batch_size: usize) -> Result<Vec<DocumentChunk>, ServiceError> {
        if chunks.is_empty() {
            return Ok(chunks);
        }

        debug!("Processing {} chunks in batches of {}", chunks.len(), batch_size);

        let mut processed_chunks = Vec::with_capacity(chunks.len());

        for batch_start in (0..chunks.len()).step_by(batch_size) {
            let batch_end = std::cmp::min(batch_start + batch_size, chunks.len());
            let batch = &chunks[batch_start..batch_end];

            debug!("Processing batch {}-{} of {}", batch_start, batch_end - 1, chunks.len());

            // Extract text content for embedding generation
            let texts: Vec<&str> = batch.iter().map(|chunk| chunk.content.as_str()).collect();

            // Generate embeddings for the batch
            let embeddings = self
                .embedding_service
                .generate_embeddings_batch(texts)
                .await
                .map_err(|e| ServiceError::document_processing(format!("Failed to generate embeddings for batch: {}", e)))?;

            if embeddings.len() != batch.len() {
                return Err(ServiceError::document_processing(format!(
                    "Embedding count mismatch: expected {}, got {}",
                    batch.len(),
                    embeddings.len()
                )));
            }

            // Add embeddings to chunks
            for (i, embedding) in embeddings.into_iter().enumerate() {
                let chunk_index = batch_start + i;
                chunks[chunk_index] = chunks[chunk_index].clone().with_embedding(embedding);
            }

            // Add processed chunks to result
            processed_chunks.extend_from_slice(&chunks[batch_start..batch_end]);
        }

        info!("Successfully processed {} chunks with embeddings", processed_chunks.len());
        Ok(processed_chunks)
    }
}

#[async_trait]
impl DocumentService for DocumentServiceImpl {
    async fn process_document(&self, content: String, filename: String) -> Result<DocumentId, ServiceError> {
        info!("Processing document: {} ({} characters)", filename, content.len());

        // Validate input
        self.validate_input(&content, &filename)?;

        // Generate unique document ID
        let document_id = Uuid::new_v4().to_string();
        debug!("Generated document ID: {}", document_id);

        // Step 1: Parse the markdown content
        debug!("Parsing markdown content");
        let _parsed_elements = self
            .parser
            .parse(&content, filename.clone())
            .map_err(|e| ServiceError::document_processing(format!("Failed to parse document: {}", e)))?;

        // Step 2: Chunk the document
        debug!("Chunking document into optimal sizes");
        let chunks = self
            .chunker
            .chunk_document(&content, document_id.clone(), filename.clone())
            .map_err(|e| ServiceError::document_processing(format!("Failed to chunk document: {}", e)))?;

        if chunks.is_empty() {
            warn!("Document '{}' produced no chunks after processing", filename);
            return Ok(document_id);
        }

        info!("Created {} chunks from document '{}'", chunks.len(), filename);

        // Step 3: Generate embeddings for chunks in batches
        const EMBEDDING_BATCH_SIZE: usize = 10; // Process 10 chunks at a time
        let chunks_with_embeddings = self.process_chunks_in_batches(chunks, EMBEDDING_BATCH_SIZE).await?;

        // Step 4: Store chunks in vector database
        debug!("Storing {} chunks in vector database", chunks_with_embeddings.len());
        self.vector_repository
            .store_chunks(chunks_with_embeddings)
            .await
            .map_err(|e| ServiceError::document_processing(format!("Failed to store chunks: {}", e)))?;

        info!("Successfully processed document '{}' with ID: {}", filename, document_id);
        Ok(document_id)
    }

    async fn get_document_chunks(&self, doc_id: DocumentId) -> Result<Vec<DocumentChunk>, ServiceError> {
        debug!("Retrieving chunks for document ID: {}", doc_id);

        if doc_id.trim().is_empty() {
            return Err(ServiceError::validation("Document ID cannot be empty"));
        }

        let chunks = self
            .vector_repository
            .get_chunks_by_document_id(&doc_id)
            .await
            .map_err(|e| ServiceError::document_processing(format!("Failed to retrieve chunks: {}", e)))?;

        info!("Retrieved {} chunks for document ID: {}", chunks.len(), doc_id);
        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SearchResult;
    use crate::repository::VectorRepository;
    use crate::services::EmbeddingService;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    // Mock embedding service for testing
    struct MockEmbeddingService {
        call_count: Arc<Mutex<usize>>,
    }

    impl MockEmbeddingService {
        fn new() -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        async fn get_call_count(&self) -> usize {
            *self.call_count.lock().await
        }
    }

    #[async_trait]
    impl EmbeddingService for MockEmbeddingService {
        async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            if text.trim().is_empty() {
                return Err(ServiceError::validation("Text cannot be empty"));
            }

            // Generate a simple embedding based on text length
            Ok(vec![text.len() as f32 / 100.0; 384])
        }

        async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            if texts.is_empty() {
                return Ok(Vec::new());
            }

            let mut embeddings = Vec::new();
            for text in texts {
                if text.trim().is_empty() {
                    return Err(ServiceError::validation("Text cannot be empty"));
                }
                embeddings.push(vec![text.len() as f32 / 100.0; 384]);
            }

            Ok(embeddings)
        }
    }

    // Mock vector repository for testing
    struct MockVectorRepository {
        stored_chunks: Arc<Mutex<HashMap<String, Vec<DocumentChunk>>>>,
        call_count: Arc<Mutex<usize>>,
    }

    impl MockVectorRepository {
        fn new() -> Self {
            Self {
                stored_chunks: Arc::new(Mutex::new(HashMap::new())),
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        async fn get_call_count(&self) -> usize {
            *self.call_count.lock().await
        }

        async fn get_stored_chunks_count(&self) -> usize {
            let chunks = self.stored_chunks.lock().await;
            chunks.values().map(|v| v.len()).sum()
        }
    }

    #[async_trait]
    impl VectorRepository for MockVectorRepository {
        async fn initialize_collection(&self) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn collection_exists(&self) -> Result<bool, ServiceError> {
            Ok(true)
        }

        async fn store_chunks(&self, chunks: Vec<DocumentChunk>) -> Result<(), ServiceError> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            let mut stored = self.stored_chunks.lock().await;
            for chunk in chunks {
                stored.entry(chunk.document_id.clone()).or_insert_with(Vec::new).push(chunk);
            }
            Ok(())
        }

        async fn search_similar(&self, _query_embedding: Vec<f32>, _limit: usize, _score_threshold: Option<f32>) -> Result<Vec<SearchResult>, ServiceError> {
            Ok(Vec::new())
        }

        async fn get_chunks_by_document_id(&self, document_id: &str) -> Result<Vec<DocumentChunk>, ServiceError> {
            let stored = self.stored_chunks.lock().await;
            Ok(stored.get(document_id).cloned().unwrap_or_default())
        }

        async fn delete_chunks_by_document_id(&self, _document_id: &str) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn delete_chunk(&self, _chunk_id: &str) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn get_collection_info(&self) -> Result<qdrant_client::qdrant::CollectionInfo, ServiceError> {
            Err(ServiceError::internal("Not implemented in mock"))
        }

        async fn health_check(&self) -> Result<bool, ServiceError> {
            Ok(true)
        }
    }

    fn create_test_service() -> (DocumentServiceImpl, Arc<MockEmbeddingService>, Arc<MockVectorRepository>) {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_repository = Arc::new(MockVectorRepository::new());

        let service = DocumentServiceImpl::new(embedding_service.clone(), vector_repository.clone());

        (service, embedding_service, vector_repository)
    }

    #[tokio::test]
    async fn test_process_simple_document() {
        let (service, embedding_service, vector_repository) = create_test_service();

        let content = "# Test Document\n\nThis is a test document with some content.";
        let filename = "test.md";

        let result = service.process_document(content.to_string(), filename.to_string()).await;

        assert!(result.is_ok(), "Document processing should succeed");
        let document_id = result.unwrap();
        assert!(!document_id.is_empty(), "Document ID should not be empty");

        // Verify embedding service was called
        assert!(embedding_service.get_call_count().await > 0, "Embedding service should be called");

        // Verify chunks were stored
        assert!(vector_repository.get_call_count().await > 0, "Vector repository should be called");
        assert!(vector_repository.get_stored_chunks_count().await > 0, "Chunks should be stored");
    }

    #[tokio::test]
    async fn test_process_empty_document() {
        let (service, _, _) = create_test_service();

        let result = service.process_document("".to_string(), "test.md".to_string()).await;

        assert!(result.is_err(), "Empty document should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_process_whitespace_only_document() {
        let (service, _, _) = create_test_service();

        let result = service.process_document("   \n\n   ".to_string(), "test.md".to_string()).await;

        assert!(result.is_err(), "Whitespace-only document should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_process_empty_filename() {
        let (service, _, _) = create_test_service();

        let result = service.process_document("# Test".to_string(), "".to_string()).await;

        assert!(result.is_err(), "Empty filename should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_process_large_document() {
        let (service, embedding_service, vector_repository) = create_test_service();

        // Create a document that will generate multiple chunks
        let content = "# Large Document\n\n".to_string() + &"This is a paragraph with content. ".repeat(100);
        let filename = "large.md";

        let result = service.process_document(content, filename.to_string()).await;

        assert!(result.is_ok(), "Large document processing should succeed");

        // Verify multiple chunks were created and processed
        let stored_count = vector_repository.get_stored_chunks_count().await;
        assert!(stored_count > 1, "Large document should create multiple chunks, got: {}", stored_count);

        // Verify batch processing was used (should be fewer calls than chunks)
        let embedding_calls = embedding_service.get_call_count().await;
        assert!(embedding_calls > 0, "Embedding service should be called");
    }

    #[tokio::test]
    async fn test_get_document_chunks() {
        let (service, _, _) = create_test_service();

        // First process a document
        let content = "# Test Document\n\nThis is test content.";
        let filename = "test.md";

        let document_id = service.process_document(content.to_string(), filename.to_string()).await.unwrap();

        // Then retrieve its chunks
        let result = service.get_document_chunks(document_id.clone()).await;

        assert!(result.is_ok(), "Getting document chunks should succeed");
        let chunks = result.unwrap();
        assert!(!chunks.is_empty(), "Should retrieve chunks for processed document");

        // Verify chunk properties
        for chunk in &chunks {
            assert_eq!(chunk.document_id, document_id);
            assert!(!chunk.content.is_empty());
            assert!(chunk.embedding.is_some());
        }
    }

    #[tokio::test]
    async fn test_get_chunks_empty_document_id() {
        let (service, _, _) = create_test_service();

        let result = service.get_document_chunks("".to_string()).await;

        assert!(result.is_err(), "Empty document ID should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_chunks_nonexistent_document() {
        let (service, _, _) = create_test_service();

        let result = service.get_document_chunks("nonexistent-id".to_string()).await;

        assert!(result.is_ok(), "Getting chunks for nonexistent document should succeed");
        let chunks = result.unwrap();
        assert!(chunks.is_empty(), "Should return empty vector for nonexistent document");
    }

    #[tokio::test]
    async fn test_input_validation() {
        let (service, _, _) = create_test_service();

        // Test various invalid inputs
        let test_cases = vec![
            ("", "test.md", "Empty content"),
            ("   ", "test.md", "Whitespace-only content"),
            ("# Test", "", "Empty filename"),
            ("# Test", "   ", "Whitespace-only filename"),
        ];

        for (content, filename, description) in test_cases {
            let result = service.process_document(content.to_string(), filename.to_string()).await;
            assert!(result.is_err(), "{} should fail", description);
            assert!(
                matches!(result.unwrap_err(), ServiceError::Validation(_)),
                "{} should return validation error",
                description
            );
        }
    }

    #[tokio::test]
    async fn test_markdown_extension_warning() {
        let (service, _, _) = create_test_service();

        // This should work but might log a warning
        let result = service.process_document("# Test".to_string(), "test.txt".to_string()).await;

        assert!(result.is_ok(), "Non-markdown extension should still work");
    }

    #[tokio::test]
    async fn test_chunking_configuration() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_repository = Arc::new(MockVectorRepository::new());

        let custom_config = ChunkingConfig {
            max_chunk_size: 100,
            overlap_size: 20,
            min_chunk_size: 10,
            respect_boundaries: true,
        };

        let service = DocumentServiceImpl::with_chunking_config(embedding_service, vector_repository.clone(), custom_config);

        let content = "This is a test document with enough content to test custom chunking configuration. ".repeat(10);
        let result = service.process_document(content, "test.md".to_string()).await;

        assert!(result.is_ok(), "Document processing with custom config should succeed");

        // Verify chunks were created with custom configuration
        let stored_count = vector_repository.get_stored_chunks_count().await;
        assert!(stored_count > 0, "Should create chunks with custom configuration");
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let (service, embedding_service, _) = create_test_service();

        // Create a document that will generate many chunks to test batching
        let content = "# Test Document\n\n".to_string() + &"This is a test paragraph. ".repeat(200);

        let result = service.process_document(content, "test.md".to_string()).await;

        assert!(result.is_ok(), "Batch processing should succeed");

        // Verify that batch processing was used (fewer embedding calls than total chunks)
        let embedding_calls = embedding_service.get_call_count().await;
        assert!(embedding_calls > 0, "Should make embedding calls");
        // The exact number depends on chunking, but should be reasonable
        assert!(embedding_calls < 50, "Should use batch processing to limit calls");
    }
}
