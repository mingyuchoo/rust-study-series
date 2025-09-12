use actix_web::{HttpResponse, ResponseError, Result, web};
use serde::Deserialize;
use std::time::Instant;
use tracing::{debug, error, info, warn};

use crate::clients::AzureOpenAIClient;
use crate::config::AppConfig;
use crate::models::ServiceError;
use crate::repository::{QdrantRepository, VectorRepository};
use crate::services::{
    RAGService,
    embedding::EmbeddingServiceImpl,
    rag::{RAGConfig, RAGServiceImpl},
    vector_search::VectorSearchServiceImpl,
};

/// Request structure for question-answering queries
#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub question: String,
    #[serde(default)]
    pub config: Option<QueryConfig>,
}

/// Optional configuration for query behavior
#[derive(Debug, Deserialize, Clone)]
pub struct QueryConfig {
    /// Maximum number of chunks to retrieve for context
    pub max_chunks: Option<usize>,
    /// Minimum similarity score for chunk inclusion
    pub similarity_threshold: Option<f32>,
    /// Maximum tokens for the generated response
    pub max_response_tokens: Option<u32>,
    /// Temperature for response generation (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Whether to include low-confidence answers
    pub include_low_confidence: Option<bool>,
}

impl From<QueryConfig> for RAGConfig {
    fn from(config: QueryConfig) -> Self {
        let mut rag_config = RAGConfig::default();

        if let Some(max_chunks) = config.max_chunks {
            rag_config.max_chunks = max_chunks;
        }
        if let Some(threshold) = config.similarity_threshold {
            rag_config.similarity_threshold = threshold;
        }
        if let Some(max_tokens) = config.max_response_tokens {
            rag_config.max_response_tokens = max_tokens;
        }
        if let Some(temp) = config.temperature {
            rag_config.temperature = temp;
        }
        if let Some(include_low) = config.include_low_confidence {
            rag_config.include_low_confidence = include_low;
        }

        rag_config
    }
}

/// Query handler that processes question-answering requests
pub async fn query_handler(request: web::Json<QueryRequest>, config: web::Data<AppConfig>, azure_client: web::Data<AzureOpenAIClient>) -> Result<HttpResponse> {
    let start_time = Instant::now();

    info!("Processing query request: {}", request.question);

    // Validate input
    if request.question.trim().is_empty() {
        warn!("Empty question received in query request");
        return Ok(ServiceError::validation("Question cannot be empty").error_response());
    }

    // Check question length (reasonable limit)
    if request.question.len() > 1000 {
        warn!("Question too long: {} characters", request.question.len());
        return Ok(ServiceError::validation("Question is too long (maximum 1000 characters)").error_response());
    }

    // Create RAG service with dependencies
    let rag_service = create_rag_service(&config, &azure_client).await?;

    // Convert query config if provided
    let rag_config = request.config.as_ref().map(|c| RAGConfig::from(c.clone()));

    // Process the query
    let result = if let Some(config) = rag_config {
        rag_service.answer_question_with_config(request.question.clone(), config).await
    } else {
        rag_service.answer_question(request.question.clone()).await
    };

    match result {
        | Ok(response) => {
            let processing_time = start_time.elapsed().as_millis() as u64;

            info!(
                "Successfully answered question in {}ms, confidence: {:.2}, sources: {}",
                processing_time,
                response.confidence,
                response.source_count()
            );

            debug!("Answer: {}", response.answer);
            debug!("Sources: {:?}", response.sources.iter().map(|s| &s.source_file).collect::<Vec<_>>());

            Ok(HttpResponse::Ok().json(response))
        },
        | Err(e) => {
            error!("Failed to process query '{}': {}", request.question, e);
            Ok(e.error_response())
        },
    }
}

/// Simple query handler that accepts just a question string
pub async fn simple_query_handler(
    question: web::Path<String>,
    config: web::Data<AppConfig>,
    azure_client: web::Data<AzureOpenAIClient>,
) -> Result<HttpResponse> {
    let request = QueryRequest {
        question: question.into_inner(),
        config: None,
    };

    query_handler(web::Json(request), config, azure_client).await
}

/// Helper function to create RAG service with all dependencies
async fn create_rag_service(config: &AppConfig, azure_client: &AzureOpenAIClient) -> Result<RAGServiceImpl, ServiceError> {
    // Create Qdrant repository
    let qdrant_repo = std::sync::Arc::new(
        QdrantRepository::new(config.qdrant.clone())
            .await
            .map_err(|e| ServiceError::database(format!("Failed to initialize Qdrant repository: {}", e)))?,
    ) as std::sync::Arc<dyn VectorRepository>;

    // Create embedding service
    let embedding_service = std::sync::Arc::new(EmbeddingServiceImpl::new(azure_client.clone())) as std::sync::Arc<dyn crate::services::EmbeddingService>;

    // Create vector search service
    let vector_search_service =
        std::sync::Arc::new(VectorSearchServiceImpl::new(qdrant_repo.clone())) as std::sync::Arc<dyn crate::services::VectorSearchService>;

    // Create RAG service
    Ok(RAGServiceImpl::new(embedding_service, vector_search_service, azure_client.clone()))
}
