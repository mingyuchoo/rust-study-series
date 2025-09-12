use async_trait::async_trait;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CollectionInfo, Condition, CreateCollection, Distance, FieldCondition, Filter, Match, PointStruct, PointsIdsList, PointsSelector, ScoredPoint, Value,
    VectorParams, VectorsConfig,
};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::config::QdrantConfig;
use crate::models::{DocumentChunk, SearchResult, ServiceError};

/// Repository trait for vector database operations
#[async_trait]
pub trait VectorRepository: Send + Sync {
    /// Initialize the vector collection
    async fn initialize_collection(&self) -> Result<(), ServiceError>;

    /// Check if the collection exists
    async fn collection_exists(&self) -> Result<bool, ServiceError>;

    /// Store document chunks with their embeddings
    async fn store_chunks(&self, chunks: Vec<DocumentChunk>) -> Result<(), ServiceError>;

    /// Search for similar chunks using vector similarity
    async fn search_similar(&self, query_embedding: Vec<f32>, limit: usize, score_threshold: Option<f32>) -> Result<Vec<SearchResult>, ServiceError>;

    /// Search for chunks by document ID
    async fn get_chunks_by_document_id(&self, document_id: &str) -> Result<Vec<DocumentChunk>, ServiceError>;

    /// Delete chunks by document ID
    async fn delete_chunks_by_document_id(&self, document_id: &str) -> Result<(), ServiceError>;

    /// Delete a specific chunk by ID
    async fn delete_chunk(&self, chunk_id: &str) -> Result<(), ServiceError>;

    /// Get collection statistics
    async fn get_collection_info(&self) -> Result<CollectionInfo, ServiceError>;

    /// Health check for the vector database
    async fn health_check(&self) -> Result<bool, ServiceError>;
}

/// Qdrant implementation of the vector repository
pub struct QdrantRepository {
    client: Qdrant,
    config: QdrantConfig,
}

impl QdrantRepository {
    /// Create a new QdrantRepository instance
    pub async fn new(config: QdrantConfig) -> Result<Self, ServiceError> {
        info!("Initializing Qdrant client with URL: {}", config.url);

        let mut client_config = qdrant_client::config::QdrantConfig::from_url(&config.url);

        // Set timeout
        client_config.timeout = Duration::from_secs(config.timeout_seconds);

        // Set API key if provided
        if let Some(api_key) = &config.api_key {
            client_config.api_key = Some(api_key.clone());
        }

        let client = Qdrant::new(client_config).map_err(|e| ServiceError::database(format!("Failed to create Qdrant client: {}", e)))?;

        let repository = Self { client, config };

        // Test connection
        repository.health_check().await?;

        info!("Qdrant client initialized successfully");
        Ok(repository)
    }

    /// Retry wrapper for Qdrant operations
    async fn retry_operation<F, T, Fut>(&self, operation: F) -> Result<T, ServiceError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, ServiceError>>,
    {
        let mut last_error = ServiceError::database("No attempts made".to_string());

        for attempt in 1..=self.config.max_retries {
            match operation().await {
                | Ok(result) => return Ok(result),
                | Err(e) => {
                    last_error = e;
                    if attempt < self.config.max_retries {
                        let delay = Duration::from_millis(100 * (2_u64.pow(attempt - 1)));
                        warn!(
                            "Qdrant operation failed (attempt {}/{}), retrying in {:?}: {}",
                            attempt, self.config.max_retries, delay, last_error
                        );
                        sleep(delay).await;
                    }
                },
            }
        }

        error!("Qdrant operation failed after {} attempts: {}", self.config.max_retries, last_error);
        Err(last_error)
    }

    /// Convert DocumentChunk to PointStruct for Qdrant
    fn chunk_to_point(&self, chunk: &DocumentChunk) -> Result<PointStruct, ServiceError> {
        let embedding = chunk
            .embedding
            .as_ref()
            .ok_or_else(|| ServiceError::validation("Chunk must have embedding to be stored"))?;

        if embedding.len() != self.config.vector_size as usize {
            return Err(ServiceError::validation(format!(
                "Embedding size {} does not match configured vector size {}",
                embedding.len(),
                self.config.vector_size
            )));
        }

        let mut payload = HashMap::new();
        payload.insert("document_id".to_string(), Value::from(chunk.document_id.clone()));
        payload.insert("content".to_string(), Value::from(chunk.content.clone()));
        payload.insert("source_file".to_string(), Value::from(chunk.metadata.source_file.clone()));
        payload.insert("chunk_index".to_string(), Value::from(chunk.metadata.chunk_index as i64));
        payload.insert("chunk_type".to_string(), Value::from(format!("{:?}", chunk.metadata.chunk_type)));
        payload.insert("created_at".to_string(), Value::from(chunk.created_at.to_rfc3339()));

        // Add headers as a JSON array
        if !chunk.metadata.headers.is_empty() {
            payload.insert(
                "headers".to_string(),
                Value::from(
                    serde_json::to_string(&chunk.metadata.headers).map_err(|e| ServiceError::serialization(format!("Failed to serialize headers: {}", e)))?,
                ),
            );
        }

        // Add optional metadata fields
        if let Some(parent_section) = &chunk.metadata.parent_section {
            payload.insert("parent_section".to_string(), Value::from(parent_section.clone()));
        }

        if let Some(start_pos) = chunk.metadata.start_position {
            payload.insert("start_position".to_string(), Value::from(start_pos as i64));
        }

        if let Some(end_pos) = chunk.metadata.end_position {
            payload.insert("end_position".to_string(), Value::from(end_pos as i64));
        }

        Ok(PointStruct::new(chunk.id.clone(), embedding.clone(), payload))
    }

    /// Convert ScoredPoint from Qdrant to SearchResult
    fn point_to_search_result(&self, point: ScoredPoint) -> Result<SearchResult, ServiceError> {
        let chunk = self.point_to_chunk(point.clone())?;
        Ok(SearchResult::new(chunk, point.score))
    }

    /// Convert ScoredPoint to DocumentChunk
    fn point_to_chunk(&self, point: ScoredPoint) -> Result<DocumentChunk, ServiceError> {
        let payload = point.payload;

        let document_id = payload
            .get("document_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ServiceError::database("Missing document_id in point payload"))?
            .to_string();

        let content = payload
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ServiceError::database("Missing content in point payload"))?
            .to_string();

        let source_file = payload
            .get("source_file")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ServiceError::database("Missing source_file in point payload"))?
            .to_string();

        let chunk_index = payload
            .get("chunk_index")
            .and_then(|v| v.as_integer())
            .ok_or_else(|| ServiceError::database("Missing chunk_index in point payload"))? as usize;

        let chunk_type_str = payload
            .get("chunk_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ServiceError::database("Missing chunk_type in point payload"))?;

        let chunk_type = match chunk_type_str.as_str() {
            | "Text" => crate::models::ChunkType::Text,
            | "CodeBlock" => crate::models::ChunkType::CodeBlock,
            | "List" => crate::models::ChunkType::List,
            | "Table" => crate::models::ChunkType::Table,
            | "Header" => crate::models::ChunkType::Header,
            | "Quote" => crate::models::ChunkType::Quote,
            | _ => crate::models::ChunkType::Text,
        };

        let created_at = payload
            .get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .ok_or_else(|| ServiceError::database("Missing or invalid created_at in point payload"))?;

        // Parse headers if present
        let headers = payload
            .get("headers")
            .and_then(|v| v.as_str())
            .map(|s| serde_json::from_str::<Vec<String>>(s).unwrap_or_default())
            .unwrap_or_default();

        // Parse optional fields
        let parent_section = payload.get("parent_section").and_then(|v| v.as_str()).map(|s| s.to_string());

        let start_position = payload.get("start_position").and_then(|v| v.as_integer()).map(|i| i as usize);

        let end_position = payload.get("end_position").and_then(|v| v.as_integer()).map(|i| i as usize);

        let mut metadata = crate::models::ChunkMetadata::new(source_file, chunk_index, chunk_type);
        metadata.headers = headers;
        metadata.parent_section = parent_section;
        metadata.start_position = start_position;
        metadata.end_position = end_position;

        // Get embedding from vectors
        let embedding = point.vectors.and_then(|vectors| vectors.vectors_options).and_then(|options| match options {
            | qdrant_client::qdrant::vectors_output::VectorsOptions::Vector(vector) => Some(vector.data),
            | _ => None,
        });

        let chunk_id = match point.id.unwrap().point_id_options.unwrap() {
            | qdrant_client::qdrant::point_id::PointIdOptions::Uuid(uuid) => uuid,
            | qdrant_client::qdrant::point_id::PointIdOptions::Num(num) => num.to_string(),
        };

        let chunk = DocumentChunk {
            id: chunk_id,
            document_id,
            content,
            metadata,
            embedding,
            created_at,
        };

        Ok(chunk)
    }
}

#[async_trait]
impl VectorRepository for QdrantRepository {
    async fn initialize_collection(&self) -> Result<(), ServiceError> {
        info!("Initializing Qdrant collection: {}", self.config.collection_name);

        self.retry_operation(|| async {
            // Check if collection already exists
            if self.collection_exists().await? {
                info!("Collection '{}' already exists", self.config.collection_name);
                return Ok(());
            }

            // Create collection
            let create_collection = CreateCollection {
                collection_name: self.config.collection_name.clone(),
                vectors_config: Some(VectorsConfig {
                    config: Some(qdrant_client::qdrant::vectors_config::Config::Params(VectorParams {
                        size: self.config.vector_size,
                        distance: Distance::Cosine as i32,
                        hnsw_config: None,
                        quantization_config: None,
                        on_disk: None,
                        datatype: None,
                        multivector_config: None,
                    })),
                }),
                hnsw_config: None,
                wal_config: None,
                optimizers_config: None,
                shard_number: None,
                on_disk_payload: None,
                timeout: Some(self.config.timeout_seconds),
                replication_factor: None,
                write_consistency_factor: None,
                init_from_collection: None,
                quantization_config: None,
                sharding_method: None,
                sparse_vectors_config: None,
                strict_mode_config: None,
            };

            let response = self
                .client
                .create_collection(create_collection)
                .await
                .map_err(|e| ServiceError::database(format!("Failed to create collection: {}", e)))?;

            if !response.result {
                return Err(ServiceError::database("Failed to create collection: operation returned false".to_string()));
            }

            info!("Successfully created collection: {}", self.config.collection_name);
            Ok(())
        })
        .await
    }

    async fn collection_exists(&self) -> Result<bool, ServiceError> {
        self.retry_operation(|| async {
            match self.client.collection_info(&self.config.collection_name).await {
                | Ok(_) => Ok(true),
                | Err(e) => {
                    let error_msg = e.to_string().to_lowercase();
                    if error_msg.contains("not found") || error_msg.contains("doesn't exist") {
                        Ok(false)
                    } else {
                        Err(ServiceError::database(format!("Failed to check collection existence: {}", e)))
                    }
                },
            }
        })
        .await
    }

    async fn store_chunks(&self, chunks: Vec<DocumentChunk>) -> Result<(), ServiceError> {
        if chunks.is_empty() {
            return Ok(());
        }

        debug!("Storing {} chunks in Qdrant", chunks.len());

        self.retry_operation(|| async {
            let points: Result<Vec<PointStruct>, ServiceError> = chunks.iter().map(|chunk| self.chunk_to_point(chunk)).collect();

            let points = points?;

            let upsert_request = qdrant_client::qdrant::UpsertPoints {
                collection_name: self.config.collection_name.clone(),
                wait: Some(true),
                points,
                ordering: None,
                shard_key_selector: None,
            };

            let response = self
                .client
                .upsert_points(upsert_request)
                .await
                .map_err(|e| ServiceError::database(format!("Failed to upsert points: {}", e)))?;

            if response.result.is_none() {
                return Err(ServiceError::database("Upsert operation failed: no result returned".to_string()));
            }

            info!("Successfully stored {} chunks", chunks.len());
            Ok(())
        })
        .await
    }

    async fn search_similar(&self, query_embedding: Vec<f32>, limit: usize, score_threshold: Option<f32>) -> Result<Vec<SearchResult>, ServiceError> {
        if query_embedding.len() != self.config.vector_size as usize {
            return Err(ServiceError::validation(format!(
                "Query embedding size {} does not match configured vector size {}",
                query_embedding.len(),
                self.config.vector_size
            )));
        }

        debug!("Searching for similar vectors with limit: {}, threshold: {:?}", limit, score_threshold);

        self.retry_operation(|| async {
            let search_request = qdrant_client::qdrant::SearchPoints {
                collection_name: self.config.collection_name.clone(),
                vector: query_embedding.clone(),
                filter: None,
                limit: limit as u64,
                with_vectors: Some(true.into()),
                with_payload: Some(true.into()),
                params: None,
                score_threshold,
                offset: None,
                vector_name: None,
                read_consistency: None,
                timeout: Some(self.config.timeout_seconds),
                shard_key_selector: None,
                sparse_indices: None,
            };

            let response = self
                .client
                .search_points(search_request)
                .await
                .map_err(|e| ServiceError::vector_search(format!("Failed to search points: {}", e)))?;

            let results: Result<Vec<SearchResult>, ServiceError> = response.result.into_iter().map(|point| self.point_to_search_result(point)).collect();

            let results = results?;
            debug!("Found {} similar chunks", results.len());

            Ok(results)
        })
        .await
    }

    async fn get_chunks_by_document_id(&self, document_id: &str) -> Result<Vec<DocumentChunk>, ServiceError> {
        debug!("Getting chunks for document ID: {}", document_id);

        self.retry_operation(|| async {
            let filter = Filter {
                should: vec![],
                must: vec![Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(FieldCondition {
                        key: "document_id".to_string(),
                        r#match: Some(Match {
                            match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keyword(document_id.to_string())),
                        }),
                        range: None,
                        geo_bounding_box: None,
                        geo_radius: None,
                        geo_polygon: None,
                        values_count: None,
                        is_empty: None,
                        is_null: None,
                        datetime_range: None,
                    })),
                }],
                must_not: vec![],
                min_should: None,
            };

            let search_request = qdrant_client::qdrant::SearchPoints {
                collection_name: self.config.collection_name.clone(),
                vector: vec![0.0; self.config.vector_size as usize], // Dummy vector for filter-only search
                filter: Some(filter),
                limit: 10000, // Large limit to get all chunks for the document
                with_vectors: Some(true.into()),
                with_payload: Some(true.into()),
                params: None,
                score_threshold: None,
                offset: None,
                vector_name: None,
                read_consistency: None,
                timeout: Some(self.config.timeout_seconds),
                shard_key_selector: None,
                sparse_indices: None,
            };

            let response = self
                .client
                .search_points(search_request)
                .await
                .map_err(|e| ServiceError::database(format!("Failed to get chunks by document ID: {}", e)))?;

            let chunks: Result<Vec<DocumentChunk>, ServiceError> = response.result.into_iter().map(|point| self.point_to_chunk(point)).collect();

            let chunks = chunks?;
            debug!("Found {} chunks for document ID: {}", chunks.len(), document_id);

            Ok(chunks)
        })
        .await
    }

    async fn delete_chunks_by_document_id(&self, document_id: &str) -> Result<(), ServiceError> {
        info!("Deleting chunks for document ID: {}", document_id);

        self.retry_operation(|| async {
            let filter = Filter {
                should: vec![],
                must: vec![Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(FieldCondition {
                        key: "document_id".to_string(),
                        r#match: Some(Match {
                            match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keyword(document_id.to_string())),
                        }),
                        range: None,
                        geo_bounding_box: None,
                        geo_radius: None,
                        geo_polygon: None,
                        values_count: None,
                        is_empty: None,
                        is_null: None,
                        datetime_range: None,
                    })),
                }],
                must_not: vec![],
                min_should: None,
            };

            let selector = PointsSelector {
                points_selector_one_of: Some(qdrant_client::qdrant::points_selector::PointsSelectorOneOf::Filter(filter)),
            };

            let delete_request = qdrant_client::qdrant::DeletePoints {
                collection_name: self.config.collection_name.clone(),
                wait: Some(true),
                points: Some(selector),
                ordering: None,
                shard_key_selector: None,
            };

            let response = self
                .client
                .delete_points(delete_request)
                .await
                .map_err(|e| ServiceError::database(format!("Failed to delete points: {}", e)))?;

            if response.result.is_none() {
                return Err(ServiceError::database("Delete operation failed: no result returned".to_string()));
            }

            info!("Successfully deleted chunks for document ID: {}", document_id);
            Ok(())
        })
        .await
    }

    async fn delete_chunk(&self, chunk_id: &str) -> Result<(), ServiceError> {
        info!("Deleting chunk with ID: {}", chunk_id);

        self.retry_operation(|| async {
            let selector = PointsSelector {
                points_selector_one_of: Some(qdrant_client::qdrant::points_selector::PointsSelectorOneOf::Points(PointsIdsList {
                    ids: vec![chunk_id.into()],
                })),
            };

            let delete_request = qdrant_client::qdrant::DeletePoints {
                collection_name: self.config.collection_name.clone(),
                wait: Some(true),
                points: Some(selector),
                ordering: None,
                shard_key_selector: None,
            };

            let response = self
                .client
                .delete_points(delete_request)
                .await
                .map_err(|e| ServiceError::database(format!("Failed to delete point: {}", e)))?;

            if response.result.is_none() {
                return Err(ServiceError::database("Delete operation failed: no result returned".to_string()));
            }

            info!("Successfully deleted chunk with ID: {}", chunk_id);
            Ok(())
        })
        .await
    }

    async fn get_collection_info(&self) -> Result<CollectionInfo, ServiceError> {
        self.retry_operation(|| async {
            let response = self
                .client
                .collection_info(&self.config.collection_name)
                .await
                .map_err(|e| ServiceError::database(format!("Failed to get collection info: {}", e)))?;

            response.result.ok_or_else(|| ServiceError::database("No collection info returned".to_string()))
        })
        .await
    }

    async fn health_check(&self) -> Result<bool, ServiceError> {
        self.retry_operation(|| async {
            match self.client.health_check().await {
                | Ok(_) => Ok(true),
                | Err(e) => Err(ServiceError::database(format!("Health check failed: {}", e))),
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ChunkMetadata, ChunkType};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_config() -> QdrantConfig {
        QdrantConfig {
            url: "http://localhost:6333".to_string(),
            api_key: None,
            collection_name: format!("test_collection_{}", Uuid::new_v4()),
            vector_size: 384, // Smaller size for testing
            timeout_seconds: 30,
            max_retries: 3,
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

    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_repository_initialization() {
        let config = create_test_config();
        let repository = QdrantRepository::new(config).await;

        assert!(repository.is_ok(), "Repository initialization should succeed");

        let repository = repository.unwrap();
        let health = repository.health_check().await;
        assert!(health.is_ok(), "Health check should succeed");
        assert!(health.unwrap(), "Health check should return true");
    }

    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_collection_operations() {
        let config = create_test_config();
        let repository = QdrantRepository::new(config).await.unwrap();

        // Test collection creation
        let result = repository.initialize_collection().await;
        assert!(result.is_ok(), "Collection initialization should succeed");

        // Test collection exists
        let exists = repository.collection_exists().await;
        assert!(exists.is_ok(), "Collection exists check should succeed");
        assert!(exists.unwrap(), "Collection should exist after creation");

        // Test collection info
        let info = repository.get_collection_info().await;
        assert!(info.is_ok(), "Getting collection info should succeed");
    }

    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_chunk_storage_and_retrieval() {
        let config = create_test_config();
        let repository = QdrantRepository::new(config).await.unwrap();
        repository.initialize_collection().await.unwrap();

        let document_id = "test_doc_1";
        let embedding = vec![0.1; 384]; // Match vector_size in test config
        let chunk = create_test_chunk(document_id, "Test content", embedding.clone());

        // Test storing chunks
        let result = repository.store_chunks(vec![chunk.clone()]).await;
        assert!(result.is_ok(), "Storing chunks should succeed");

        // Test retrieving chunks by document ID
        let retrieved = repository.get_chunks_by_document_id(document_id).await;
        assert!(retrieved.is_ok(), "Retrieving chunks should succeed");

        let chunks = retrieved.unwrap();
        assert_eq!(chunks.len(), 1, "Should retrieve one chunk");
        assert_eq!(chunks[0].document_id, document_id);
        assert_eq!(chunks[0].content, "Test content");
    }

    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_vector_search() {
        let config = create_test_config();
        let repository = QdrantRepository::new(config).await.unwrap();
        repository.initialize_collection().await.unwrap();

        let document_id = "test_doc_search";
        let embedding = vec![0.5; 384];
        let chunk = create_test_chunk(document_id, "Searchable content", embedding.clone());

        // Store the chunk
        repository.store_chunks(vec![chunk]).await.unwrap();

        // Search for similar vectors
        let query_embedding = vec![0.5; 384]; // Same as stored embedding
        let results = repository.search_similar(query_embedding, 5, None).await;

        assert!(results.is_ok(), "Vector search should succeed");
        let search_results = results.unwrap();
        assert!(!search_results.is_empty(), "Should find similar vectors");
        assert_eq!(search_results[0].chunk.content, "Searchable content");
    }

    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_chunk_deletion() {
        let config = create_test_config();
        let repository = QdrantRepository::new(config).await.unwrap();
        repository.initialize_collection().await.unwrap();

        let document_id = "test_doc_delete";
        let embedding = vec![0.3; 384];
        let chunk = create_test_chunk(document_id, "Content to delete", embedding);
        let chunk_id = chunk.id.clone();

        // Store the chunk
        repository.store_chunks(vec![chunk]).await.unwrap();

        // Verify it exists
        let chunks = repository.get_chunks_by_document_id(document_id).await.unwrap();
        assert_eq!(chunks.len(), 1);

        // Delete by chunk ID
        let result = repository.delete_chunk(&chunk_id).await;
        assert!(result.is_ok(), "Chunk deletion should succeed");

        // Verify it's gone
        let chunks = repository.get_chunks_by_document_id(document_id).await.unwrap();
        assert_eq!(chunks.len(), 0, "Chunk should be deleted");
    }

    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_delete_chunks_by_document_id() {
        let config = create_test_config();
        let repository = QdrantRepository::new(config).await.unwrap();
        repository.initialize_collection().await.unwrap();

        let document_id = "test_doc_bulk_delete";
        let embedding = vec![0.7; 384];

        // Create multiple chunks for the same document
        let chunks = vec![
            create_test_chunk(document_id, "Content 1", embedding.clone()),
            create_test_chunk(document_id, "Content 2", embedding.clone()),
            create_test_chunk(document_id, "Content 3", embedding),
        ];

        // Store the chunks
        repository.store_chunks(chunks).await.unwrap();

        // Verify they exist
        let stored_chunks = repository.get_chunks_by_document_id(document_id).await.unwrap();
        assert_eq!(stored_chunks.len(), 3);

        // Delete all chunks for the document
        let result = repository.delete_chunks_by_document_id(document_id).await;
        assert!(result.is_ok(), "Bulk deletion should succeed");

        // Verify they're gone
        let remaining_chunks = repository.get_chunks_by_document_id(document_id).await.unwrap();
        assert_eq!(remaining_chunks.len(), 0, "All chunks should be deleted");
    }
}
