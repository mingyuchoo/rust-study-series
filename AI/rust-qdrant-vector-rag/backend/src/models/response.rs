use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response structure for RAG (Retrieval Augmented Generation) queries
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct RAGResponse {
    pub answer: String,
    pub sources: Vec<SourceReference>,
    pub confidence: f32,
    pub query: String,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl RAGResponse {
    /// Creates a new RAG response
    pub fn new(answer: String, sources: Vec<SourceReference>, confidence: f32, query: String, response_time_ms: u64) -> Self {
        Self {
            answer,
            sources,
            confidence,
            query,
            response_time_ms,
            timestamp: Utc::now(),
        }
    }

    /// Returns the number of sources used
    pub fn source_count(&self) -> usize { self.sources.len() }

    /// Returns the highest relevance score among sources
    #[allow(dead_code)]
    pub fn max_relevance_score(&self) -> f32 { self.sources.iter().map(|s| s.relevance_score).fold(0.0, f32::max) }

    /// Returns sources sorted by relevance score (descending)
    #[allow(dead_code)]
    pub fn sources_by_relevance(&self) -> Vec<&SourceReference> {
        let mut sources: Vec<&SourceReference> = self.sources.iter().collect();
        sources.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        sources
    }
}

/// Reference to a source document chunk used in generating the answer
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, ToSchema)]
pub struct SourceReference {
    pub document_id: String,
    pub chunk_id: String,
    pub relevance_score: f32,
    pub snippet: String,
    pub source_file: String,
    pub chunk_index: usize,
    pub headers: Vec<String>,
}

impl SourceReference {
    /// Creates a new source reference
    pub fn new(document_id: String, chunk_id: String, relevance_score: f32, snippet: String, source_file: String, chunk_index: usize) -> Self {
        Self {
            document_id,
            chunk_id,
            relevance_score,
            snippet,
            source_file,
            chunk_index,
            headers: Vec::new(),
        }
    }

    /// Adds headers to the source reference
    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    /// Returns a truncated snippet if it exceeds the given length
    #[allow(dead_code)]
    pub fn truncated_snippet(&self, max_length: usize) -> String {
        if self.snippet.len() <= max_length {
            self.snippet.clone()
        } else {
            format!("{}...", &self.snippet[.. max_length])
        }
    }
}

/// Response for document upload operations
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadResponse {
    pub document_id: String,
    pub filename: String,
    pub chunks_created: usize,
    pub processing_time_ms: u64,
    pub status: UploadStatus,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl UploadResponse {
    /// Creates a successful upload response
    pub fn success(document_id: String, filename: String, chunks_created: usize, processing_time_ms: u64) -> Self {
        Self {
            document_id,
            filename,
            chunks_created,
            processing_time_ms,
            status: UploadStatus::Success,
            message: "Document processed successfully".to_string(),
            timestamp: Utc::now(),
        }
    }

    /// Creates a failed upload response
    pub fn failure(filename: String, error_message: String) -> Self {
        Self {
            document_id: String::new(),
            filename,
            chunks_created: 0,
            processing_time_ms: 0,
            status: UploadStatus::Failed,
            message: error_message,
            timestamp: Utc::now(),
        }
    }
}

/// Status of document upload operation
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum UploadStatus {
    Success,
    Failed,
    Processing,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub timestamp: DateTime<Utc>,
    pub services: ServiceHealthStatus,
    pub uptime_seconds: u64,
}

impl HealthResponse {
    /// Creates a new health response
    pub fn new(services: ServiceHealthStatus, uptime_seconds: u64) -> Self {
        let status = if services.all_healthy() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };

        Self {
            status,
            timestamp: Utc::now(),
            services,
            uptime_seconds,
        }
    }
}

/// Overall health status
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Health status of external services
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealthStatus {
    pub qdrant: bool,
    pub azure_openai: bool,
}

impl ServiceHealthStatus {
    /// Creates a new service health status
    pub fn new(qdrant: bool, azure_openai: bool) -> Self {
        Self {
            qdrant,
            azure_openai,
        }
    }

    /// Returns true if all services are healthy
    pub fn all_healthy(&self) -> bool { self.qdrant && self.azure_openai }
}
