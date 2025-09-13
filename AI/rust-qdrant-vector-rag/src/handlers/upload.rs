use crate::clients::AzureOpenAIClient;
use crate::config::AppConfig;
use crate::models::{ServiceError, UploadResponse};
use crate::repository::{QdrantRepository, VectorRepository};
use crate::services::DocumentService;
use crate::services::document::DocumentServiceImpl;
use crate::services::embedding::EmbeddingServiceImpl;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, ResponseError, Result, web};
use futures_util::TryStreamExt;
use serde::Deserialize;
use std::time::Instant;
use tracing::{debug, error, info, warn};
use utoipa::ToSchema;

/// Request structure for file upload (when using JSON)
#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadRequest {
    pub content: String,
    pub filename: String,
}

/// 루트 경로용 래퍼: POST /upload
/// 기존 멀티파트 업로드 핸들러를 재사용합니다.
#[utoipa::path(
    post,
    path = "/upload",
    tag = "upload",
    responses(
        (status = 200, description = "업로드 완료", body = UploadResponse),
        (status = 400, description = "유효성 검사 실패")
    )
)]
pub async fn upload_handler_root(
    payload: Multipart,
    config: web::Data<AppConfig>,
    azure_client: web::Data<AzureOpenAIClient>,
) -> Result<HttpResponse> {
    // 원본 핸들러는 가변 payload를 요구하므로, 이 래퍼는 동일 시그니처로 위임합니다.
    upload_handler(payload, config, azure_client).await
}

/// 멀티파트 기반 마크다운 파일 업로드 엔드포인트
/// Swagger 표시를 위해 OpenAPI 메타데이터를 추가합니다.
#[utoipa::path(
    post,
    path = "/api/v1/upload",
    tag = "upload",
    responses(
        (status = 200, description = "업로드 완료", body = UploadResponse),
        (status = 400, description = "유효성 검사 실패")
    )
)]
pub async fn upload_handler(mut payload: Multipart, config: web::Data<AppConfig>, azure_client: web::Data<AzureOpenAIClient>) -> Result<HttpResponse> {
    let start_time = Instant::now();

    info!("Processing file upload request");

    // Extract file from multipart form data
    let mut filename = String::new();
    let mut content = String::new();

    while let Some(mut field) = payload.try_next().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        ServiceError::validation(format!("Invalid multipart data: {}", e))
    })? {
        let content_disposition = field.content_disposition();

        match content_disposition.get_name() {
            | Some("file") => {
                // Get filename from content disposition
                if let Some(file_name) = content_disposition.get_filename() {
                    filename = file_name.to_string();
                    debug!("Processing file: {}", filename);

                    // Validate file extension
                    if !filename.to_lowercase().ends_with(".md") && !filename.to_lowercase().ends_with(".markdown") {
                        warn!("Invalid file type uploaded: {}", filename);
                        return Ok(ServiceError::validation("Only markdown files (.md, .markdown) are supported").error_response());
                    }
                }

                // Read file content
                let mut file_content = Vec::new();
                while let Some(chunk) = field.try_next().await.map_err(|e| {
                    error!("Failed to read file chunk: {}", e);
                    ServiceError::validation(format!("Failed to read file content: {}", e))
                })? {
                    file_content.extend_from_slice(chunk.as_ref());

                    // Check file size limit (from config)
                    if file_content.len() > config.server.max_request_size {
                        warn!("File too large: {} bytes", file_content.len());
                        return Ok(
                            ServiceError::validation(format!("File too large. Maximum size is {} bytes", config.server.max_request_size)).error_response(),
                        );
                    }
                }

                // Convert to UTF-8 string
                content = String::from_utf8(file_content).map_err(|e| {
                    error!("Invalid UTF-8 content in file: {}", e);
                    ServiceError::validation("File must contain valid UTF-8 text")
                })?;
            },
            | Some("filename") => {
                // Alternative way to get filename
                let mut field_content = Vec::new();
                while let Some(chunk) = field
                    .try_next()
                    .await
                    .map_err(|e| ServiceError::validation(format!("Failed to read filename field: {}", e)))?
                {
                    field_content.extend_from_slice(chunk.as_ref());
                }
                if filename.is_empty() {
                    filename = String::from_utf8_lossy(&field_content).to_string();
                }
            },
            | _ => {
                debug!("Ignoring unknown field: {:?}", content_disposition.get_name());
            },
        }
    }

    // Validate that we have both filename and content
    if filename.is_empty() {
        warn!("No filename provided in upload request");
        return Ok(ServiceError::validation("Filename is required").error_response());
    }

    if content.is_empty() {
        warn!("Empty file content uploaded: {}", filename);
        return Ok(ServiceError::validation("File content cannot be empty").error_response());
    }

    info!("Processing file: {} ({} bytes)", filename, content.len());

    // Create document service with dependencies
    let document_service = create_document_service(&config, &azure_client).await?;

    // Process the document
    match document_service.process_document(content, filename.clone()).await {
        | Ok(document_id) => {
            let processing_time = start_time.elapsed().as_millis() as u64;

            // Get chunk count for response
            let chunks_created = match document_service.get_document_chunks(document_id.clone()).await {
                | Ok(chunks) => chunks.len(),
                | Err(e) => {
                    warn!("Failed to get chunk count for document {}: {}", document_id, e);
                    0 // Don't fail the request, just return 0
                },
            };

            info!(
                "Successfully processed document {} in {}ms, created {} chunks",
                document_id, processing_time, chunks_created
            );

            let response = UploadResponse::success(document_id, filename, chunks_created, processing_time);

            Ok(HttpResponse::Ok().json(response))
        },
        | Err(e) => {
            error!("Failed to process document {}: {}", filename, e);
            let _response = UploadResponse::failure(filename, e.to_string());
            Ok(e.error_response())
        },
    }
}

/// JSON-based upload handler for when content is sent as JSON
/// OpenAPI 문서화를 위한 메타데이터를 추가합니다.
#[utoipa::path(
    post,
    path = "/api/v1/upload/json",
    tag = "upload",
    request_body = UploadRequest,
    responses(
        (status = 200, description = "업로드 완료", body = UploadResponse),
        (status = 400, description = "유효성 검사 실패")
    )
)]
pub async fn upload_json_handler(
    request: web::Json<UploadRequest>,
    config: web::Data<AppConfig>,
    azure_client: web::Data<AzureOpenAIClient>,
) -> Result<HttpResponse> {
    let start_time = Instant::now();

    info!("Processing JSON upload request for file: {}", request.filename);

    // Validate input
    if request.filename.is_empty() {
        return Ok(ServiceError::validation("Filename is required").error_response());
    }

    if request.content.is_empty() {
        return Ok(ServiceError::validation("File content cannot be empty").error_response());
    }

    // Validate file extension
    if !request.filename.to_lowercase().ends_with(".md") && !request.filename.to_lowercase().ends_with(".markdown") {
        return Ok(ServiceError::validation("Only markdown files (.md, .markdown) are supported").error_response());
    }

    // Check content size
    if request.content.len() > config.server.max_request_size {
        return Ok(ServiceError::validation(format!("Content too large. Maximum size is {} bytes", config.server.max_request_size)).error_response());
    }

    // Create document service with dependencies
    let document_service = create_document_service(&config, &azure_client).await?;

    // Process the document
    match document_service.process_document(request.content.clone(), request.filename.clone()).await {
        | Ok(document_id) => {
            let processing_time = start_time.elapsed().as_millis() as u64;

            // Get chunk count for response
            let chunks_created = match document_service.get_document_chunks(document_id.clone()).await {
                | Ok(chunks) => chunks.len(),
                | Err(e) => {
                    warn!("Failed to get chunk count for document {}: {}", document_id, e);
                    0
                },
            };

            info!(
                "Successfully processed JSON document {} in {}ms, created {} chunks",
                document_id, processing_time, chunks_created
            );

            let response = UploadResponse::success(document_id, request.filename.clone(), chunks_created, processing_time);

            Ok(HttpResponse::Ok().json(response))
        },
        | Err(e) => {
            error!("Failed to process document {}: {}", request.filename, e);
            let _response = UploadResponse::failure(request.filename.clone(), e.to_string());
            Ok(e.error_response())
        },
    }
}

/// Helper function to create document service with all dependencies
async fn create_document_service(config: &AppConfig, azure_client: &AzureOpenAIClient) -> Result<DocumentServiceImpl, ServiceError> {
    // Create Qdrant repository
    let qdrant_repo = std::sync::Arc::new(
        QdrantRepository::new(config.qdrant.clone())
            .await
            .map_err(|e| ServiceError::database(format!("Failed to initialize Qdrant repository: {}", e)))?,
    ) as std::sync::Arc<dyn VectorRepository>;

    // Create embedding service
    let embedding_service = std::sync::Arc::new(EmbeddingServiceImpl::new(azure_client.clone())) as std::sync::Arc<dyn crate::services::EmbeddingService>;

    // Create document service
    Ok(DocumentServiceImpl::new(embedding_service, qdrant_repo))
}
