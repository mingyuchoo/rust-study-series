use crate::clients::azure_openai::{AzureOpenAIClient, ChatMessage};
use crate::models::{RAGResponse, ServiceError, SourceReference};
use crate::services::{EmbeddingService, VectorSearchService};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info, warn};

#[async_trait]
pub trait RAGService: Send + Sync {
    async fn answer_question(&self, question: String) -> Result<RAGResponse, ServiceError>;
    async fn answer_question_with_config(&self, question: String, config: RAGConfig) -> Result<RAGResponse, ServiceError>;
}

/// Configuration for RAG pipeline behavior
#[derive(Debug, Clone)]
pub struct RAGConfig {
    /// Maximum number of chunks to retrieve for context
    pub max_chunks: usize,
    /// Minimum similarity score for chunk inclusion
    pub similarity_threshold: f32,
    /// Maximum tokens for the generated response
    pub max_response_tokens: u32,
    /// Temperature for response generation (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum length of snippet in source references
    pub max_snippet_length: usize,
    /// Whether to include low-confidence answers
    pub include_low_confidence: bool,
    /// Minimum confidence threshold for answers
    pub min_confidence_threshold: f32,
}

impl Default for RAGConfig {
    fn default() -> Self {
        Self {
            max_chunks: 5,
            similarity_threshold: 0.7,
            max_response_tokens: 500,
            temperature: 0.3,
            max_snippet_length: 200,
            include_low_confidence: false,
            min_confidence_threshold: 0.6,
        }
    }
}

pub struct RAGServiceImpl {
    embedding_service: Arc<dyn EmbeddingService>,
    vector_search_service: Arc<dyn VectorSearchService>,
    azure_client: AzureOpenAIClient,
    default_config: RAGConfig,
}

impl RAGServiceImpl {
    pub fn new(embedding_service: Arc<dyn EmbeddingService>, vector_search_service: Arc<dyn VectorSearchService>, azure_client: AzureOpenAIClient) -> Self {
        Self {
            embedding_service,
            vector_search_service,
            azure_client,
            default_config: RAGConfig::default(),
        }
    }

    #[allow(dead_code)]
    pub fn with_config(
        embedding_service: Arc<dyn EmbeddingService>,
        vector_search_service: Arc<dyn VectorSearchService>,
        azure_client: AzureOpenAIClient,
        config: RAGConfig,
    ) -> Self {
        Self {
            embedding_service,
            vector_search_service,
            azure_client,
            default_config: config,
        }
    }

    /// Validates the input question
    fn validate_question(&self, question: &str) -> Result<(), ServiceError> {
        if question.trim().is_empty() {
            return Err(ServiceError::validation("Question cannot be empty"));
        }

        const MAX_QUESTION_LENGTH: usize = 1000;
        if question.len() > MAX_QUESTION_LENGTH {
            return Err(ServiceError::validation(format!(
                "Question too long: {} characters (max: {})",
                question.len(),
                MAX_QUESTION_LENGTH
            )));
        }

        Ok(())
    }

    /// Validates RAG configuration
    fn validate_config(&self, config: &RAGConfig) -> Result<(), ServiceError> {
        if config.max_chunks == 0 {
            return Err(ServiceError::validation("max_chunks must be greater than 0"));
        }

        if config.max_chunks > 20 {
            return Err(ServiceError::validation("max_chunks cannot exceed 20"));
        }

        if config.similarity_threshold < 0.0 || config.similarity_threshold > 1.0 {
            return Err(ServiceError::validation("similarity_threshold must be between 0.0 and 1.0"));
        }

        if config.temperature < 0.0 || config.temperature > 1.0 {
            return Err(ServiceError::validation("temperature must be between 0.0 and 1.0"));
        }

        if config.min_confidence_threshold < 0.0 || config.min_confidence_threshold > 1.0 {
            return Err(ServiceError::validation("min_confidence_threshold must be between 0.0 and 1.0"));
        }

        if config.max_response_tokens == 0 || config.max_response_tokens > 4000 {
            return Err(ServiceError::validation("max_response_tokens must be between 1 and 4000"));
        }

        Ok(())
    }

    /// Constructs context from retrieved chunks
    fn construct_context(&self, search_results: &[crate::models::SearchResult], config: &RAGConfig) -> String {
        if search_results.is_empty() {
            return String::new();
        }

        let mut context = String::new();
        context.push_str("Based on the following information:\n\n");

        for (i, result) in search_results.iter().enumerate() {
            let chunk = &result.chunk;

            // Add source information
            context.push_str(&format!("Source {} (from {}):\n", i + 1, chunk.metadata.source_file));

            // Add headers if available
            if !chunk.metadata.headers.is_empty() {
                context.push_str(&format!("Section: {}\n", chunk.metadata.headers.join(" > ")));
            }

            // Add content
            let content = if chunk.content.len() > config.max_snippet_length * 2 {
                // If content is very long, truncate but preserve readability
                let truncated = &chunk.content[..config.max_snippet_length * 2];
                if let Some(last_sentence) = truncated.rfind('.') {
                    format!("{}.", &truncated[..last_sentence])
                } else {
                    format!("{}...", truncated)
                }
            } else {
                chunk.content.clone()
            };

            context.push_str(&content);
            context.push_str("\n\n");
        }

        context
    }

    /// Creates source references from search results
    fn create_source_references(&self, search_results: &[crate::models::SearchResult], config: &RAGConfig) -> Vec<SourceReference> {
        search_results
            .iter()
            .map(|result| {
                let chunk = &result.chunk;
                let snippet = if chunk.content.len() > config.max_snippet_length {
                    let truncated = &chunk.content[..config.max_snippet_length];
                    if let Some(last_space) = truncated.rfind(' ') {
                        format!("{}...", &truncated[..last_space])
                    } else {
                        format!("{}...", truncated)
                    }
                } else {
                    chunk.content.clone()
                };

                SourceReference::new(
                    chunk.document_id.clone(),
                    chunk.id.clone(),
                    result.relevance_score,
                    snippet,
                    chunk.metadata.source_file.clone(),
                    chunk.metadata.chunk_index,
                )
                .with_headers(chunk.metadata.headers.clone())
            })
            .collect()
    }

    /// Estimates confidence based on search results and answer quality
    fn estimate_confidence(&self, search_results: &[crate::models::SearchResult], answer: &str) -> f32 {
        if search_results.is_empty() {
            return 0.0;
        }

        // Base confidence on the highest similarity score
        let max_similarity = search_results.iter().map(|r| r.relevance_score).fold(0.0, f32::max);

        // Adjust based on number of sources
        let source_factor = match search_results.len() {
            | 0 => 0.0,
            | 1 => 0.8,
            | 2..=3 => 1.0,
            | _ => 0.95, // Too many sources might indicate scattered information
        };

        // Adjust based on answer length (very short answers might be less reliable)
        let length_factor = if answer.len() < 50 {
            0.7
        } else if answer.len() > 1000 {
            0.9 // Very long answers might be less focused
        } else {
            1.0
        };

        // Check for uncertainty indicators in the answer
        let uncertainty_factor = if answer.to_lowercase().contains("i don't know")
            || answer.to_lowercase().contains("i'm not sure")
            || answer.to_lowercase().contains("unclear")
            || answer.to_lowercase().contains("cannot determine")
        {
            0.3
        } else if answer.to_lowercase().contains("might") || answer.to_lowercase().contains("possibly") || answer.to_lowercase().contains("perhaps") {
            0.7
        } else {
            1.0
        };

        // Combine factors
        let confidence = max_similarity * source_factor * length_factor * uncertainty_factor;

        // Clamp to [0.0, 1.0]
        confidence.max(0.0).min(1.0)
    }

    /// Creates the system prompt for the RAG pipeline
    fn create_system_prompt(&self) -> String {
        r#"You are a helpful AI assistant that answers questions based on provided context information. 

Instructions:
1. Use ONLY the information provided in the context to answer questions
2. If the context doesn't contain enough information to answer the question, say so clearly
3. Be concise but comprehensive in your answers
4. Cite specific sources when possible
5. If you're uncertain about something, express that uncertainty
6. Do not make up information that isn't in the provided context
7. Structure your answer clearly with proper formatting when appropriate

Remember: Your knowledge is limited to the provided context. Do not use external knowledge beyond what's given."#
            .to_string()
    }

    /// Creates the user prompt with context and question
    fn create_user_prompt(&self, context: &str, question: &str) -> String {
        if context.trim().is_empty() {
            format!(
                "Question: {}\n\nI don't have any relevant context information to answer this question.",
                question
            )
        } else {
            format!("{}\nQuestion: {}", context, question)
        }
    }
}

#[async_trait]
impl RAGService for RAGServiceImpl {
    async fn answer_question(&self, question: String) -> Result<RAGResponse, ServiceError> {
        self.answer_question_with_config(question, self.default_config.clone()).await
    }

    async fn answer_question_with_config(&self, question: String, config: RAGConfig) -> Result<RAGResponse, ServiceError> {
        let start_time = Instant::now();

        debug!("RAGService: processing question of length {}", question.len());

        // Validate inputs
        self.validate_question(&question)?;
        self.validate_config(&config)?;

        // Step 1: Generate embedding for the question
        debug!("RAGService: generating embedding for question");
        let question_embedding = self
            .embedding_service
            .generate_embedding(&question)
            .await
            .map_err(|e| ServiceError::internal(format!("Failed to generate question embedding: {}", e)))?;

        // Step 2: Search for similar chunks
        debug!("RAGService: searching for similar chunks with threshold {}", config.similarity_threshold);
        let search_results = self
            .vector_search_service
            .search_similar_with_threshold(question_embedding, config.max_chunks, config.similarity_threshold)
            .await
            .map_err(|e| ServiceError::internal(format!("Failed to search for similar chunks: {}", e)))?;

        info!("RAGService: found {} relevant chunks", search_results.len());

        // Step 3: Construct context from retrieved chunks
        let context = self.construct_context(&search_results, &config);
        debug!("RAGService: constructed context of length {}", context.len());

        // Step 4: Generate answer using Azure OpenAI
        let system_prompt = self.create_system_prompt();
        let user_prompt = self.create_user_prompt(&context, &question);

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        debug!("RAGService: generating answer with Azure OpenAI");
        let answer = self
            .azure_client
            .generate_chat_completion(messages, Some(config.max_response_tokens), Some(config.temperature))
            .await
            .map_err(|e| ServiceError::internal(format!("Failed to generate answer: {}", e)))?;

        // Step 5: Create source references
        let sources = self.create_source_references(&search_results, &config);

        // Step 6: Estimate confidence
        let confidence = self.estimate_confidence(&search_results, &answer);

        // Step 7: Check confidence threshold
        if !config.include_low_confidence && confidence < config.min_confidence_threshold {
            warn!(
                "RAGService: answer confidence {} below threshold {}",
                confidence, config.min_confidence_threshold
            );
            return Ok(RAGResponse::new(
                "I don't have enough reliable information to answer this question confidently. Please try rephrasing your question or provide more context."
                    .to_string(),
                sources,
                confidence,
                question,
                start_time.elapsed().as_millis() as u64,
            ));
        }

        let response_time = start_time.elapsed().as_millis() as u64;
        info!("RAGService: generated answer in {}ms with confidence {:.2}", response_time, confidence);

        Ok(RAGResponse::new(answer, sources, confidence, question, response_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::azure_openai::{AzureOpenAIClient, ChatMessage};
    use crate::config::AzureOpenAIConfig;
    use crate::models::{ChunkMetadata, ChunkType, DocumentChunk, SearchResult};
    use crate::services::{EmbeddingService, VectorSearchService};
    use async_trait::async_trait;
    use chrono::Utc;
    use std::collections::HashMap;
    use tokio::sync::Mutex;
    use uuid::Uuid;

    // Mock embedding service
    struct MockEmbeddingService {
        should_fail: bool,
    }

    impl MockEmbeddingService {
        fn new() -> Self {
            Self { should_fail: false }
        }

        fn with_failure() -> Self {
            Self { should_fail: true }
        }
    }

    #[async_trait]
    impl EmbeddingService for MockEmbeddingService {
        async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, ServiceError> {
            if self.should_fail {
                return Err(ServiceError::embedding_generation("Mock embedding failure"));
            }

            if text.trim().is_empty() {
                return Err(ServiceError::validation("Text cannot be empty"));
            }

            // Generate a simple embedding based on text content
            Ok(vec![text.len() as f32 / 100.0; 384])
        }

        async fn generate_embeddings_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, ServiceError> {
            if self.should_fail {
                return Err(ServiceError::embedding_generation("Mock batch embedding failure"));
            }

            let mut embeddings = Vec::new();
            for text in texts {
                embeddings.push(self.generate_embedding(text).await?);
            }
            Ok(embeddings)
        }
    }

    // Mock vector search service
    struct MockVectorSearchService {
        search_results: Arc<Mutex<Vec<SearchResult>>>,
        should_fail: bool,
    }

    impl MockVectorSearchService {
        fn new() -> Self {
            Self {
                search_results: Arc::new(Mutex::new(Vec::new())),
                should_fail: false,
            }
        }

        fn with_failure() -> Self {
            Self {
                search_results: Arc::new(Mutex::new(Vec::new())),
                should_fail: true,
            }
        }

        async fn set_search_results(&self, results: Vec<SearchResult>) {
            let mut search_results = self.search_results.lock().await;
            *search_results = results;
        }
    }

    #[async_trait]
    impl VectorSearchService for MockVectorSearchService {
        async fn search_similar(&self, _query_embedding: Vec<f32>, limit: usize) -> Result<Vec<SearchResult>, ServiceError> {
            if self.should_fail {
                return Err(ServiceError::vector_search("Mock search failure"));
            }

            let search_results = self.search_results.lock().await;
            let mut results = search_results.clone();
            results.truncate(limit);
            Ok(results)
        }

        async fn search_similar_with_threshold(&self, _query_embedding: Vec<f32>, limit: usize, threshold: f32) -> Result<Vec<SearchResult>, ServiceError> {
            if self.should_fail {
                return Err(ServiceError::vector_search("Mock search failure"));
            }

            let search_results = self.search_results.lock().await;
            let mut results: Vec<_> = search_results.iter().filter(|r| r.relevance_score >= threshold).cloned().collect();
            results.truncate(limit);
            Ok(results)
        }

        async fn store_embeddings(&self, _chunks: Vec<DocumentChunk>) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn delete_document_embeddings(&self, _document_id: &str) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn get_collection_stats(&self) -> Result<crate::services::vector_search::VectorCollectionStats, ServiceError> {
            Ok(crate::services::vector_search::VectorCollectionStats {
                total_vectors: 0,
                indexed_vectors: 0,
                collection_status: "Green".to_string(),
            })
        }
    }

    // Mock Azure OpenAI client
    struct MockAzureOpenAIClient {
        response: String,
        should_fail: bool,
    }

    impl MockAzureOpenAIClient {
        fn new(response: String) -> Self {
            Self { response, should_fail: false }
        }

        fn with_failure() -> Self {
            Self {
                response: String::new(),
                should_fail: true,
            }
        }

        async fn generate_chat_completion(
            &self,
            _messages: Vec<ChatMessage>,
            _max_tokens: Option<u32>,
            _temperature: Option<f32>,
        ) -> Result<String, ServiceError> {
            if self.should_fail {
                return Err(ServiceError::external_api("Mock chat completion failure"));
            }
            Ok(self.response.clone())
        }
    }

    fn create_test_chunk(document_id: &str, content: &str, source_file: &str) -> DocumentChunk {
        let metadata = ChunkMetadata::new(source_file.to_string(), 0, ChunkType::Text).with_headers(vec!["Test Section".to_string()]);

        DocumentChunk {
            id: Uuid::new_v4().to_string(),
            document_id: document_id.to_string(),
            content: content.to_string(),
            metadata,
            embedding: Some(vec![0.1; 384]),
            created_at: Utc::now(),
        }
    }

    fn create_test_search_result(content: &str, score: f32, source_file: &str) -> SearchResult {
        let chunk = create_test_chunk("test-doc", content, source_file);
        SearchResult::new(chunk, score)
    }

    fn create_test_service() -> RAGServiceImpl {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::new());

        let config = AzureOpenAIConfig {
            endpoint: "https://test.openai.azure.com".to_string(),
            api_key: "test-key".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        let azure_client = AzureOpenAIClient::new(config).unwrap();

        RAGServiceImpl::new(embedding_service, vector_search_service, azure_client)
    }

    fn create_mock_service(
        embedding_service: Arc<MockEmbeddingService>,
        vector_search_service: Arc<MockVectorSearchService>,
        azure_client: MockAzureOpenAIClient,
    ) -> MockRAGService {
        MockRAGService {
            embedding_service,
            vector_search_service,
            azure_client,
            default_config: RAGConfig::default(),
        }
    }

    // Mock RAG service for testing
    struct MockRAGService {
        embedding_service: Arc<MockEmbeddingService>,
        vector_search_service: Arc<MockVectorSearchService>,
        azure_client: MockAzureOpenAIClient,
        default_config: RAGConfig,
    }

    #[async_trait]
    impl RAGService for MockRAGService {
        async fn answer_question(&self, question: String) -> Result<RAGResponse, ServiceError> {
            self.answer_question_with_config(question, self.default_config.clone()).await
        }

        async fn answer_question_with_config(&self, question: String, config: RAGConfig) -> Result<RAGResponse, ServiceError> {
            let start_time = Instant::now();

            if question.trim().is_empty() {
                return Err(ServiceError::validation("Question cannot be empty"));
            }

            // Generate embedding
            let _embedding = self.embedding_service.generate_embedding(&question).await?;

            // Search for similar chunks
            let search_results = self
                .vector_search_service
                .search_similar_with_threshold(vec![0.5; 384], config.max_chunks, config.similarity_threshold)
                .await?;

            // Generate answer
            let answer = self
                .azure_client
                .generate_chat_completion(vec![], Some(config.max_response_tokens), Some(config.temperature))
                .await?;

            // Create source references
            let sources: Vec<SourceReference> = search_results
                .iter()
                .map(|result| {
                    let chunk = &result.chunk;
                    SourceReference::new(
                        chunk.document_id.clone(),
                        chunk.id.clone(),
                        result.relevance_score,
                        chunk.content.clone(),
                        chunk.metadata.source_file.clone(),
                        chunk.metadata.chunk_index,
                    )
                })
                .collect();

            let confidence = if search_results.is_empty() { 0.0 } else { 0.8 };
            let response_time = start_time.elapsed().as_millis() as u64;

            Ok(RAGResponse::new(answer, sources, confidence, question, response_time))
        }
    }

    #[tokio::test]
    async fn test_answer_question_success() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::new());
        let azure_client = MockAzureOpenAIClient::new("This is a test answer.".to_string());

        let service = create_mock_service(embedding_service, vector_search_service.clone(), azure_client);

        // Set up mock search results
        let search_results = vec![
            create_test_search_result("Relevant content about the question", 0.9, "test.md"),
            create_test_search_result("More relevant information", 0.8, "docs.md"),
        ];
        vector_search_service.set_search_results(search_results).await;

        let question = "What is the answer to my question?";
        let result = service.answer_question(question.to_string()).await;

        assert!(result.is_ok(), "Answer question should succeed");
        let response = result.unwrap();
        assert_eq!(response.answer, "This is a test answer.");
        assert_eq!(response.query, question);
        assert_eq!(response.sources.len(), 2);
        assert!(response.confidence > 0.0);
        assert!(response.response_time_ms > 0);
    }

    #[tokio::test]
    async fn test_answer_question_empty_question() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::new());
        let azure_client = MockAzureOpenAIClient::new("Answer".to_string());

        let service = create_mock_service(embedding_service, vector_search_service, azure_client);

        let result = service.answer_question("".to_string()).await;

        assert!(result.is_err(), "Empty question should fail");
        assert!(matches!(result.unwrap_err(), ServiceError::Validation(_)));
    }

    #[tokio::test]
    async fn test_answer_question_no_relevant_chunks() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::new());
        let azure_client = MockAzureOpenAIClient::new("I don't have enough information.".to_string());

        let service = create_mock_service(embedding_service, vector_search_service.clone(), azure_client);

        // No search results
        vector_search_service.set_search_results(vec![]).await;

        let result = service.answer_question("What is the answer?".to_string()).await;

        assert!(result.is_ok(), "Should succeed even with no chunks");
        let response = result.unwrap();
        assert!(response.sources.is_empty());
        assert_eq!(response.confidence, 0.0);
    }

    #[tokio::test]
    async fn test_answer_question_embedding_failure() {
        let embedding_service = Arc::new(MockEmbeddingService::with_failure());
        let vector_search_service = Arc::new(MockVectorSearchService::new());
        let azure_client = MockAzureOpenAIClient::new("Answer".to_string());

        let service = create_mock_service(embedding_service, vector_search_service, azure_client);

        let result = service.answer_question("What is the answer?".to_string()).await;

        assert!(result.is_err(), "Embedding failure should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::Internal(_)));
    }

    #[tokio::test]
    async fn test_answer_question_search_failure() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::with_failure());
        let azure_client = MockAzureOpenAIClient::new("Answer".to_string());

        let service = create_mock_service(embedding_service, vector_search_service, azure_client);

        let result = service.answer_question("What is the answer?".to_string()).await;

        assert!(result.is_err(), "Search failure should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::Internal(_)));
    }

    #[tokio::test]
    async fn test_answer_question_chat_completion_failure() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::new());
        let azure_client = MockAzureOpenAIClient::with_failure();

        let service = create_mock_service(embedding_service, vector_search_service.clone(), azure_client);

        // Set up search results
        let search_results = vec![create_test_search_result("Content", 0.8, "test.md")];
        vector_search_service.set_search_results(search_results).await;

        let result = service.answer_question("What is the answer?".to_string()).await;

        assert!(result.is_err(), "Chat completion failure should be propagated");
        assert!(matches!(result.unwrap_err(), ServiceError::Internal(_)));
    }

    #[tokio::test]
    async fn test_validate_question() {
        let service = create_test_service();

        // Valid question
        assert!(service.validate_question("What is the answer?").is_ok());

        // Empty question
        assert!(service.validate_question("").is_err());
        assert!(service.validate_question("   ").is_err());

        // Too long question
        let long_question = "a".repeat(1001);
        assert!(service.validate_question(&long_question).is_err());
    }

    #[tokio::test]
    async fn test_validate_config() {
        let service = create_test_service();

        // Valid config
        let valid_config = RAGConfig::default();
        assert!(service.validate_config(&valid_config).is_ok());

        // Invalid max_chunks
        let mut invalid_config = RAGConfig::default();
        invalid_config.max_chunks = 0;
        assert!(service.validate_config(&invalid_config).is_err());

        invalid_config.max_chunks = 25;
        assert!(service.validate_config(&invalid_config).is_err());

        // Invalid similarity_threshold
        invalid_config = RAGConfig::default();
        invalid_config.similarity_threshold = -0.1;
        assert!(service.validate_config(&invalid_config).is_err());

        invalid_config.similarity_threshold = 1.5;
        assert!(service.validate_config(&invalid_config).is_err());

        // Invalid temperature
        invalid_config = RAGConfig::default();
        invalid_config.temperature = -0.1;
        assert!(service.validate_config(&invalid_config).is_err());

        invalid_config.temperature = 1.5;
        assert!(service.validate_config(&invalid_config).is_err());
    }

    #[tokio::test]
    async fn test_construct_context() {
        let service = create_test_service();
        let config = RAGConfig::default();

        let search_results = vec![
            create_test_search_result("First piece of information", 0.9, "doc1.md"),
            create_test_search_result("Second piece of information", 0.8, "doc2.md"),
        ];

        let context = service.construct_context(&search_results, &config);

        assert!(!context.is_empty());
        assert!(context.contains("Based on the following information"));
        assert!(context.contains("First piece of information"));
        assert!(context.contains("Second piece of information"));
        assert!(context.contains("doc1.md"));
        assert!(context.contains("doc2.md"));
    }

    #[tokio::test]
    async fn test_construct_context_empty_results() {
        let service = create_test_service();
        let config = RAGConfig::default();

        let context = service.construct_context(&[], &config);

        assert!(context.is_empty());
    }

    #[tokio::test]
    async fn test_create_source_references() {
        let service = create_test_service();
        let config = RAGConfig::default();

        let search_results = vec![
            create_test_search_result("Short content", 0.9, "doc1.md"),
            create_test_search_result(&"Long content ".repeat(50), 0.8, "doc2.md"), // Long content
        ];

        let sources = service.create_source_references(&search_results, &config);

        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].relevance_score, 0.9);
        assert_eq!(sources[1].relevance_score, 0.8);
        assert_eq!(sources[0].source_file, "doc1.md");
        assert_eq!(sources[1].source_file, "doc2.md");

        // Long content should be truncated
        assert!(sources[1].snippet.len() <= config.max_snippet_length + 3); // +3 for "..."
        assert!(sources[1].snippet.ends_with("..."));
    }

    #[tokio::test]
    async fn test_estimate_confidence() {
        let service = create_test_service();

        // High confidence case
        let high_score_results = vec![
            create_test_search_result("Relevant content", 0.95, "doc.md"),
            create_test_search_result("More content", 0.9, "doc.md"),
        ];
        let confidence = service.estimate_confidence(&high_score_results, "This is a confident answer.");
        assert!(confidence > 0.8);

        // Low confidence case - no results
        let confidence = service.estimate_confidence(&[], "No information available.");
        assert_eq!(confidence, 0.0);

        // Uncertain answer
        let uncertain_results = vec![create_test_search_result("Some content", 0.8, "doc.md")];
        let confidence = service.estimate_confidence(&uncertain_results, "I'm not sure about this answer.");
        assert!(confidence < 0.6);

        // Very short answer
        let confidence = service.estimate_confidence(&uncertain_results, "Yes.");
        assert!(confidence < 0.8);
    }

    #[tokio::test]
    async fn test_create_system_prompt() {
        let service = create_test_service();
        let prompt = service.create_system_prompt();

        assert!(!prompt.is_empty());
        assert!(prompt.contains("helpful AI assistant"));
        assert!(prompt.contains("provided context"));
        assert!(prompt.contains("do not make up information"));
    }

    #[tokio::test]
    async fn test_create_user_prompt() {
        let service = create_test_service();

        // With context
        let context = "This is some context information.";
        let question = "What is the answer?";
        let prompt = service.create_user_prompt(context, question);

        assert!(prompt.contains(context));
        assert!(prompt.contains(question));

        // Without context
        let prompt = service.create_user_prompt("", question);
        assert!(prompt.contains("don't have any relevant context"));
        assert!(prompt.contains(question));
    }

    #[tokio::test]
    async fn test_answer_question_with_custom_config() {
        let embedding_service = Arc::new(MockEmbeddingService::new());
        let vector_search_service = Arc::new(MockVectorSearchService::new());
        let azure_client = MockAzureOpenAIClient::new("Custom answer".to_string());

        let service = create_mock_service(embedding_service, vector_search_service.clone(), azure_client);

        let search_results = vec![create_test_search_result("Content", 0.9, "test.md")];
        vector_search_service.set_search_results(search_results).await;

        let custom_config = RAGConfig {
            max_chunks: 3,
            similarity_threshold: 0.8,
            max_response_tokens: 200,
            temperature: 0.5,
            max_snippet_length: 100,
            include_low_confidence: true,
            min_confidence_threshold: 0.3,
        };

        let result = service.answer_question_with_config("Test question".to_string(), custom_config).await;

        assert!(result.is_ok(), "Custom config should work");
        let response = result.unwrap();
        assert_eq!(response.answer, "Custom answer");
    }
}
