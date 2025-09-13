#[cfg(test)]
mod tests {
    use crate::models::document::*;
    use crate::models::error::*;
    use crate::models::response::*;
    use serde_json;

    #[test]
    fn test_document_chunk_creation() {
        let metadata = ChunkMetadata::new("test.md".to_string(), 0, ChunkType::Text);

        let chunk = DocumentChunk::new("doc123".to_string(), "This is test content".to_string(), metadata);

        assert_eq!(chunk.document_id, "doc123");
        assert_eq!(chunk.content, "This is test content");
        assert_eq!(chunk.metadata.source_file, "test.md");
        assert_eq!(chunk.metadata.chunk_index, 0);
        assert_eq!(chunk.metadata.chunk_type, ChunkType::Text);
        assert!(chunk.embedding.is_none());
        assert!(!chunk.id.is_empty());
    }

    #[test]
    fn test_document_chunk_with_embedding() {
        let metadata = ChunkMetadata::new("test.md".to_string(), 0, ChunkType::Text);

        let embedding = vec![0.1, 0.2, 0.3, 0.4];
        let chunk = DocumentChunk::new("doc123".to_string(), "Test content".to_string(), metadata).with_embedding(embedding.clone());

        assert_eq!(chunk.embedding, Some(embedding));
    }

    #[test]
    fn test_document_chunk_token_estimation() {
        let metadata = ChunkMetadata::new("test.md".to_string(), 0, ChunkType::Text);

        let chunk = DocumentChunk::new(
            "doc123".to_string(),
            "This is a test content with exactly forty characters".to_string(), // 50 chars
            metadata,
        );

        // Should estimate ~12-13 tokens (50 chars / 4)
        let token_count = chunk.estimated_token_count();
        assert!(token_count >= 10 && token_count <= 15);
    }

    #[test]
    fn test_chunk_metadata_builder() {
        let metadata = ChunkMetadata::new("test.md".to_string(), 5, ChunkType::CodeBlock)
            .with_headers(vec!["Header 1".to_string(), "Header 2".to_string()])
            .with_position(100, 200)
            .with_parent_section("Introduction".to_string());

        assert_eq!(metadata.source_file, "test.md");
        assert_eq!(metadata.chunk_index, 5);
        assert_eq!(metadata.chunk_type, ChunkType::CodeBlock);
        assert_eq!(metadata.headers, vec!["Header 1", "Header 2"]);
        assert_eq!(metadata.start_position, Some(100));
        assert_eq!(metadata.end_position, Some(200));
        assert_eq!(metadata.parent_section, Some("Introduction".to_string()));
    }

    #[test]
    fn test_chunk_type_serialization() {
        let chunk_types = vec![
            ChunkType::Text,
            ChunkType::CodeBlock,
            ChunkType::List,
            ChunkType::Table,
            ChunkType::Header,
            ChunkType::Quote,
        ];

        for chunk_type in chunk_types {
            let serialized = serde_json::to_string(&chunk_type).unwrap();
            let deserialized: ChunkType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(chunk_type, deserialized);
        }
    }

    #[test]
    fn test_search_result_creation() {
        let metadata = ChunkMetadata::new("test.md".to_string(), 0, ChunkType::Text);

        let chunk = DocumentChunk::new("doc123".to_string(), "Test content".to_string(), metadata);

        let search_result = SearchResult::new(chunk.clone(), 0.85);

        assert_eq!(search_result.chunk, chunk);
        assert_eq!(search_result.relevance_score, 0.85);
    }

    #[test]
    fn test_service_error_creation() {
        let error = ServiceError::document_processing("Failed to parse");
        assert!(matches!(error, ServiceError::DocumentProcessing(_)));
        assert_eq!(error.to_string(), "Document processing failed: Failed to parse");
        assert_eq!(error.status_code(), 500);
        assert!(!error.is_retryable());

        let network_error = ServiceError::Network("Connection timeout".to_string());
        assert!(network_error.is_retryable());
        assert_eq!(network_error.status_code(), 503);
    }

    #[test]
    fn test_service_error_status_codes() {
        assert_eq!(ServiceError::validation("test").status_code(), 400);
        assert_eq!(ServiceError::Authentication("test".to_string()).status_code(), 401);
        assert_eq!(ServiceError::not_found("test").status_code(), 404);
        assert_eq!(ServiceError::rate_limit("test").status_code(), 429);
        assert_eq!(ServiceError::internal("test").status_code(), 500);
        assert_eq!(ServiceError::external_api("test").status_code(), 502);
        assert_eq!(ServiceError::Database("test".to_string()).status_code(), 503);
    }

    #[test]
    fn test_rag_response_creation() {
        let sources = vec![
            SourceReference::new(
                "doc1".to_string(),
                "chunk1".to_string(),
                0.9,
                "This is a snippet".to_string(),
                "file1.md".to_string(),
                0,
            ),
            SourceReference::new(
                "doc2".to_string(),
                "chunk2".to_string(),
                0.7,
                "Another snippet".to_string(),
                "file2.md".to_string(),
                1,
            ),
        ];

        let response = RAGResponse::new("This is the answer".to_string(), sources, 0.85, "What is the question?".to_string(), 150);

        assert_eq!(response.answer, "This is the answer");
        assert_eq!(response.query, "What is the question?");
        assert_eq!(response.confidence, 0.85);
        assert_eq!(response.response_time_ms, 150);
        assert_eq!(response.source_count(), 2);
        assert_eq!(response.max_relevance_score(), 0.9);
    }

    #[test]
    fn test_rag_response_sources_by_relevance() {
        let sources = vec![
            SourceReference::new(
                "doc1".to_string(),
                "chunk1".to_string(),
                0.7,
                "Lower relevance".to_string(),
                "file1.md".to_string(),
                0,
            ),
            SourceReference::new(
                "doc2".to_string(),
                "chunk2".to_string(),
                0.9,
                "Higher relevance".to_string(),
                "file2.md".to_string(),
                1,
            ),
        ];

        let response = RAGResponse::new("Answer".to_string(), sources, 0.8, "Question".to_string(), 100);

        let sorted_sources = response.sources_by_relevance();
        assert_eq!(sorted_sources[0].relevance_score, 0.9);
        assert_eq!(sorted_sources[1].relevance_score, 0.7);
    }

    #[test]
    fn test_source_reference_with_headers() {
        let source = SourceReference::new(
            "doc1".to_string(),
            "chunk1".to_string(),
            0.8,
            "Test snippet".to_string(),
            "test.md".to_string(),
            0,
        )
        .with_headers(vec!["Header 1".to_string(), "Header 2".to_string()]);

        assert_eq!(source.headers, vec!["Header 1", "Header 2"]);
    }

    #[test]
    fn test_source_reference_truncated_snippet() {
        let source = SourceReference::new(
            "doc1".to_string(),
            "chunk1".to_string(),
            0.8,
            "This is a very long snippet that should be truncated".to_string(),
            "test.md".to_string(),
            0,
        );

        let truncated = source.truncated_snippet(20);
        assert_eq!(truncated, "This is a very long ...");

        let not_truncated = source.truncated_snippet(100);
        assert_eq!(not_truncated, source.snippet);
    }

    #[test]
    fn test_upload_response_success() {
        let response = UploadResponse::success("doc123".to_string(), "test.md".to_string(), 5, 1000);

        assert_eq!(response.document_id, "doc123");
        assert_eq!(response.filename, "test.md");
        assert_eq!(response.chunks_created, 5);
        assert_eq!(response.processing_time_ms, 1000);
        assert_eq!(response.status, UploadStatus::Success);
        assert_eq!(response.message, "Document processed successfully");
    }

    #[test]
    fn test_upload_response_failure() {
        let response = UploadResponse::failure("test.md".to_string(), "Invalid file format".to_string());

        assert!(response.document_id.is_empty());
        assert_eq!(response.filename, "test.md");
        assert_eq!(response.chunks_created, 0);
        assert_eq!(response.processing_time_ms, 0);
        assert_eq!(response.status, UploadStatus::Failed);
        assert_eq!(response.message, "Invalid file format");
    }

    #[test]
    fn test_health_response() {
        let services = ServiceHealthStatus::new(true, false);
        let health = HealthResponse::new(services, 3600);

        assert_eq!(health.status, HealthStatus::Degraded);
        assert_eq!(health.uptime_seconds, 3600);
        assert!(health.services.qdrant);
        assert!(!health.services.azure_openai);
        assert!(!health.services.all_healthy());
    }

    #[test]
    fn test_service_health_all_healthy() {
        let healthy_services = ServiceHealthStatus::new(true, true);
        assert!(healthy_services.all_healthy());

        let unhealthy_services = ServiceHealthStatus::new(true, false);
        assert!(!unhealthy_services.all_healthy());
    }

    #[test]
    fn test_document_chunk_serialization() {
        let metadata = ChunkMetadata::new("test.md".to_string(), 0, ChunkType::Text);

        let chunk = DocumentChunk::new("doc123".to_string(), "Test content".to_string(), metadata);

        let serialized = serde_json::to_string(&chunk).unwrap();
        let deserialized: DocumentChunk = serde_json::from_str(&serialized).unwrap();

        assert_eq!(chunk.id, deserialized.id);
        assert_eq!(chunk.document_id, deserialized.document_id);
        assert_eq!(chunk.content, deserialized.content);
        assert_eq!(chunk.metadata, deserialized.metadata);
    }

    #[test]
    fn test_rag_response_serialization() {
        let sources = vec![SourceReference::new(
            "doc1".to_string(),
            "chunk1".to_string(),
            0.9,
            "Snippet".to_string(),
            "file.md".to_string(),
            0,
        )];

        let response = RAGResponse::new("Answer".to_string(), sources, 0.85, "Question".to_string(), 100);

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: RAGResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.answer, deserialized.answer);
        assert_eq!(response.query, deserialized.query);
        assert_eq!(response.confidence, deserialized.confidence);
        assert_eq!(response.sources, deserialized.sources);
    }

    #[test]
    fn test_error_conversions() {
        // Test serde_json::Error conversion
        let json_error = serde_json::from_str::<DocumentChunk>("invalid json");
        assert!(json_error.is_err());
        let service_error: ServiceError = json_error.unwrap_err().into();
        assert!(matches!(service_error, ServiceError::Serialization(_)));

        // Test config::ConfigError conversion
        let config_error = config::ConfigError::NotFound("test".to_string());
        let service_error: ServiceError = config_error.into();
        assert!(matches!(service_error, ServiceError::Configuration(_)));
    }
}
