use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::time::Duration;
use tracing::info;

/// Initialize metrics collection and Prometheus exporter
pub fn init_metrics() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing metrics collection...");

    let builder = PrometheusBuilder::new();
    builder.install().map_err(|e| format!("Failed to install Prometheus exporter: {}", e))?;

    // Register application metrics
    register_application_metrics();

    info!("Metrics collection initialized successfully");
    Ok(())
}

/// Register all application-specific metrics
fn register_application_metrics() {
    // Metrics are automatically registered when first used in the new metrics crate
    // version
    info!("Metrics will be registered on first use");
}

/// Metrics helper functions for easy use throughout the application
pub struct Metrics;

impl Metrics {
    // HTTP metrics
    pub fn increment_http_requests(method: &str, path: &str, status: u16) {
        counter!("http_requests_total", "method" => method.to_string(), "path" => path.to_string(), "status" => status.to_string()).increment(1);
    }

    pub fn record_http_duration(method: &str, path: &str, duration: Duration) {
        histogram!("http_request_duration_seconds", "method" => method.to_string(), "path" => path.to_string()).record(duration.as_secs_f64());
    }

    pub fn increment_http_errors(method: &str, path: &str, error_type: &str) {
        counter!("http_requests_errors_total", "method" => method.to_string(), "path" => path.to_string(), "error_type" => error_type.to_string()).increment(1);
    }

    // Document processing metrics
    pub fn increment_documents_processed() { counter!("documents_processed_total").increment(1); }

    pub fn record_document_processing_duration(duration: Duration) { histogram!("document_processing_duration_seconds").record(duration.as_secs_f64()); }

    pub fn set_document_chunks_stored(count: u64) { gauge!("document_chunks_stored").set(count as f64); }

    pub fn record_document_chunk_size(size_bytes: usize) { histogram!("document_chunk_size_bytes").record(size_bytes as f64); }

    // Embedding metrics
    pub fn increment_embeddings_generated(count: u64) { counter!("embeddings_generated_total").increment(count); }

    pub fn record_embedding_generation_duration(duration: Duration) { histogram!("embedding_generation_duration_seconds").record(duration.as_secs_f64()); }

    pub fn increment_embedding_batch_requests() { counter!("embedding_batch_requests_total").increment(1); }

    pub fn record_embedding_batch_size(size: usize) { histogram!("embedding_batch_size").record(size as f64); }

    // Vector search metrics
    pub fn increment_vector_searches() { counter!("vector_searches_total").increment(1); }

    pub fn record_vector_search_duration(duration: Duration) { histogram!("vector_search_duration_seconds").record(duration.as_secs_f64()); }

    pub fn record_vector_search_results_count(count: usize) { histogram!("vector_search_results_count").record(count as f64); }

    // RAG metrics
    pub fn increment_rag_queries() { counter!("rag_queries_total").increment(1); }

    pub fn record_rag_query_duration(duration: Duration) { histogram!("rag_query_duration_seconds").record(duration.as_secs_f64()); }

    pub fn record_rag_context_length(tokens: usize) { histogram!("rag_context_length_tokens").record(tokens as f64); }

    // External service metrics
    pub fn increment_azure_openai_requests(operation: &str) { counter!("azure_openai_requests_total", "operation" => operation.to_string()).increment(1); }

    pub fn record_azure_openai_duration(operation: &str, duration: Duration) {
        histogram!("azure_openai_request_duration_seconds", "operation" => operation.to_string()).record(duration.as_secs_f64());
    }

    pub fn increment_azure_openai_errors(operation: &str, error_type: &str) {
        counter!("azure_openai_errors_total", "operation" => operation.to_string(), "error_type" => error_type.to_string()).increment(1);
    }

    pub fn increment_azure_openai_rate_limits() { counter!("azure_openai_rate_limits_total").increment(1); }

    pub fn increment_qdrant_requests(operation: &str) { counter!("qdrant_requests_total", "operation" => operation.to_string()).increment(1); }

    pub fn record_qdrant_duration(operation: &str, duration: Duration) {
        histogram!("qdrant_request_duration_seconds", "operation" => operation.to_string()).record(duration.as_secs_f64());
    }

    pub fn increment_qdrant_errors(operation: &str, error_type: &str) {
        counter!("qdrant_errors_total", "operation" => operation.to_string(), "error_type" => error_type.to_string()).increment(1);
    }

    // System metrics
    pub fn set_memory_usage(bytes: u64) { gauge!("memory_usage_bytes").set(bytes as f64); }

    pub fn set_cpu_usage(percent: f64) { gauge!("cpu_usage_percent").set(percent); }

    pub fn set_active_connections(count: u64) { gauge!("active_connections").set(count as f64); }
}

/// Metrics middleware for automatic HTTP request tracking
pub struct MetricsMiddleware;

impl MetricsMiddleware {
    pub fn record_request_start() -> instant::Instant { instant::Instant::now() }

    pub fn record_request_end(start_time: instant::Instant, method: &str, path: &str, status: u16) {
        let duration = start_time.elapsed();
        Metrics::record_http_duration(method, path, duration);
        Metrics::increment_http_requests(method, path, status);
    }

    pub fn record_request_error(method: &str, path: &str, error_type: &str) { Metrics::increment_http_errors(method, path, error_type); }
}
