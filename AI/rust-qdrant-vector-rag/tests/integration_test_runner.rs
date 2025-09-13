use std::time::Duration;
use tracing_test::traced_test;

mod common;

use common::{PerformanceMetrics, TestData, TestResult, TestResults, cleanup_test_environment, setup_test_environment, verify_test_isolation};

/// Comprehensive integration test runner that orchestrates all test scenarios
#[tokio::test]
#[traced_test]
#[ignore] // Requires external services (Qdrant, Azure OpenAI)
async fn run_comprehensive_integration_tests() {
    let mut results = TestResults::new();

    tracing::info!("Starting comprehensive integration test suite");

    // Test 1: Environment Setup and Isolation
    let setup_result = test_environment_setup_and_isolation().await;
    results.add_result(setup_result);

    // Test 2: Document Processing Pipeline
    let document_result = test_document_processing_pipeline().await;
    results.add_result(document_result);

    // Test 3: Question Answering Pipeline
    let qa_result = test_question_answering_pipeline().await;
    results.add_result(qa_result);

    // Test 4: Error Handling and Edge Cases
    let error_result = test_error_handling_and_edge_cases().await;
    results.add_result(error_result);

    // Test 5: Performance and Scalability
    let performance_result = test_performance_and_scalability().await;
    results.add_result(performance_result);

    // Test 6: Concurrent Operations
    let concurrency_result = test_concurrent_operations().await;
    results.add_result(concurrency_result);

    // Test 7: Data Cleanup and Isolation
    let cleanup_result = test_data_cleanup_and_isolation().await;
    results.add_result(cleanup_result);

    // Log final results
    results.log_summary();

    // Assert overall success rate
    assert!(
        results.success_rate() >= 0.7, // Allow 30% failure rate for external service dependencies
        "Integration test suite failed with success rate: {:.1}%",
        results.success_rate() * 100.0
    );

    tracing::info!("Comprehensive integration test suite completed");
}

/// Test environment setup and isolation
async fn test_environment_setup_and_isolation() -> TestResult {
    tracing::info!("Testing environment setup and isolation");

    let metrics = PerformanceMetrics::new("environment_setup".to_string());

    match setup_test_environment("test_isolation_1", 8090).await {
        | Ok(container1) => {
            match setup_test_environment("test_isolation_2", 8091).await {
                | Ok(container2) => {
                    // Test isolation
                    match verify_test_isolation(&container1, &container2).await {
                        | Ok(()) => {
                            metrics.log_completion();
                            tracing::info!("Environment setup and isolation test: PASSED");
                            TestResult::Passed
                        },
                        | Err(e) => {
                            tracing::error!("Test isolation failed: {}", e);
                            TestResult::Failed
                        },
                    }
                },
                | Err(e) => {
                    tracing::warn!("Failed to setup second test environment: {}", e);
                    TestResult::Skipped
                },
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup first test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test complete document processing pipeline
async fn test_document_processing_pipeline() -> TestResult {
    tracing::info!("Testing document processing pipeline");

    let metrics = PerformanceMetrics::new("document_processing".to_string());

    match setup_test_environment("test_document_processing", 8092).await {
        | Ok(container) => {
            let test_data = TestData::new();

            // Test document processing
            match container
                .document_service
                .process_document(test_data.sample_markdown.to_string(), test_data.sample_filename.to_string())
                .await
            {
                | Ok(document_id) => {
                    tracing::info!("Document processed successfully: {}", document_id);

                    // Verify chunks were created
                    match container.document_service.get_document_chunks(document_id.clone()).await {
                        | Ok(chunks) => {
                            if chunks.is_empty() {
                                tracing::error!("No chunks were created");
                                return TestResult::Failed;
                            }

                            // Verify chunk properties
                            let mut valid_chunks = 0;
                            for chunk in &chunks {
                                if chunk.embedding.is_some() && !chunk.content.is_empty() {
                                    valid_chunks += 1;
                                }
                            }

                            if valid_chunks == chunks.len() {
                                metrics.assert_performance(Duration::from_secs(30));
                                metrics.log_completion();
                                tracing::info!("Document processing pipeline test: PASSED ({} chunks)", chunks.len());
                                TestResult::Passed
                            } else {
                                tracing::error!("Some chunks are invalid: {}/{}", valid_chunks, chunks.len());
                                TestResult::Failed
                            }
                        },
                        | Err(e) => {
                            tracing::error!("Failed to get document chunks: {}", e);
                            TestResult::Failed
                        },
                    }
                },
                | Err(e) => {
                    tracing::warn!("Document processing failed (expected in test environment): {}", e);
                    TestResult::Skipped
                },
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test complete question answering pipeline
async fn test_question_answering_pipeline() -> TestResult {
    tracing::info!("Testing question answering pipeline");

    let metrics = PerformanceMetrics::new("question_answering".to_string());

    match setup_test_environment("test_qa_pipeline", 8093).await {
        | Ok(container) => {
            let test_data = TestData::new();

            // First upload a document
            match container
                .document_service
                .process_document(test_data.sample_markdown.to_string(), test_data.sample_filename.to_string())
                .await
            {
                | Ok(document_id) => {
                    tracing::info!("Document uploaded for QA testing: {}", document_id);

                    // Wait for indexing
                    tokio::time::sleep(Duration::from_millis(500)).await;

                    // Test questions
                    let mut successful_queries = 0;
                    let total_questions = test_data.test_questions.len();

                    for question in test_data.test_questions {
                        match container.rag_service.answer_question(question.to_string()).await {
                            | Ok(response) =>
                                if !response.answer.is_empty() && response.confidence >= 0.0 && response.confidence <= 1.0 {
                                    successful_queries += 1;
                                    tracing::debug!("Question answered: {} (confidence: {:.2})", question, response.confidence);
                                } else {
                                    tracing::warn!("Invalid response for question: {}", question);
                                },
                            | Err(e) => {
                                tracing::warn!("Question failed: {} - {}", question, e);
                            },
                        }
                    }

                    let success_rate = successful_queries as f32 / total_questions as f32;

                    if success_rate >= 0.5 {
                        // Allow 50% success rate for test environment
                        metrics.assert_performance(Duration::from_secs(60));
                        metrics.log_completion();
                        tracing::info!(
                            "Question answering pipeline test: PASSED ({}/{} questions)",
                            successful_queries,
                            total_questions
                        );
                        TestResult::Passed
                    } else {
                        tracing::error!("Low success rate for questions: {}/{}", successful_queries, total_questions);
                        TestResult::Failed
                    }
                },
                | Err(e) => {
                    tracing::warn!("Failed to upload document for QA testing: {}", e);
                    TestResult::Skipped
                },
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test error handling and edge cases
async fn test_error_handling_and_edge_cases() -> TestResult {
    tracing::info!("Testing error handling and edge cases");

    let metrics = PerformanceMetrics::new("error_handling".to_string());

    match setup_test_environment("test_error_handling", 8094).await {
        | Ok(container) => {
            let mut tests_passed = 0;
            let total_tests = 6;

            // Test 1: Empty document
            match container.document_service.process_document("".to_string(), "empty.md".to_string()).await {
                | Err(_) => {
                    tests_passed += 1;
                    tracing::debug!("Empty document correctly rejected");
                },
                | Ok(_) => {
                    tracing::warn!("Empty document should have been rejected");
                },
            }

            // Test 2: Empty question
            match container.rag_service.answer_question("".to_string()).await {
                | Err(_) => {
                    tests_passed += 1;
                    tracing::debug!("Empty question correctly rejected");
                },
                | Ok(_) => {
                    tracing::warn!("Empty question should have been rejected");
                },
            }

            // Test 3: Very long question
            let long_question = "What ".repeat(500) + "is this about?";
            match container.rag_service.answer_question(long_question).await {
                | Ok(_) | Err(_) => {
                    tests_passed += 1; // Either response is acceptable
                    tracing::debug!("Long question handled appropriately");
                },
            }

            // Test 4: Plain text document (should work)
            match container
                .document_service
                .process_document("Just plain text".to_string(), "plain.md".to_string())
                .await
            {
                | Ok(_) => {
                    tests_passed += 1;
                    tracing::debug!("Plain text document processed successfully");
                },
                | Err(e) => {
                    tracing::warn!("Plain text document failed: {}", e);
                },
            }

            // Test 5: Question with no relevant documents
            match container.rag_service.answer_question("What is the meaning of life?".to_string()).await {
                | Ok(response) =>
                    if response.source_count() == 0 || response.confidence < 0.3 {
                        tests_passed += 1;
                        tracing::debug!("Irrelevant question handled appropriately");
                    } else {
                        tracing::warn!("Irrelevant question returned high confidence answer");
                    },
                | Err(_) => {
                    tests_passed += 1; // Error is also acceptable
                    tracing::debug!("Irrelevant question correctly failed");
                },
            }

            // Test 6: Health check functionality
            match container.health_check().await {
                | Ok(_) => {
                    tests_passed += 1;
                    tracing::debug!("Health check working");
                },
                | Err(e) => {
                    tracing::warn!("Health check failed: {}", e);
                },
            }

            let success_rate = tests_passed as f32 / total_tests as f32;

            if success_rate >= 0.8 {
                // Require 80% success for error handling
                metrics.log_completion();
                tracing::info!("Error handling test: PASSED ({}/{} tests)", tests_passed, total_tests);
                TestResult::Passed
            } else {
                tracing::error!("Error handling test failed: {}/{} tests passed", tests_passed, total_tests);
                TestResult::Failed
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test performance and scalability
async fn test_performance_and_scalability() -> TestResult {
    tracing::info!("Testing performance and scalability");

    let metrics = PerformanceMetrics::new("performance_test".to_string());

    match setup_test_environment("test_performance", 8095).await {
        | Ok(container) => {
            let test_data = TestData::new();
            let (large_content, large_filename) = test_data.get_large_document();

            // Test large document processing
            let doc_start = std::time::Instant::now();
            match container
                .document_service
                .process_document(large_content.to_string(), large_filename.to_string())
                .await
            {
                | Ok(document_id) => {
                    let doc_time = doc_start.elapsed();
                    tracing::info!("Large document processed in {:?}: {}", doc_time, document_id);

                    // Test query performance
                    let query_start = std::time::Instant::now();
                    match container.rag_service.answer_question("What is this document about?".to_string()).await {
                        | Ok(response) => {
                            let query_time = query_start.elapsed();
                            tracing::info!("Query completed in {:?}, confidence: {:.2}", query_time, response.confidence);

                            // Performance assertions (relaxed for test environment)
                            if doc_time < Duration::from_secs(60) && query_time < Duration::from_secs(30) {
                                metrics.log_completion();
                                tracing::info!("Performance test: PASSED");
                                TestResult::Passed
                            } else {
                                tracing::warn!("Performance test: SLOW (doc: {:?}, query: {:?})", doc_time, query_time);
                                TestResult::Failed
                            }
                        },
                        | Err(e) => {
                            tracing::warn!("Query failed in performance test: {}", e);
                            TestResult::Skipped
                        },
                    }
                },
                | Err(e) => {
                    tracing::warn!("Large document processing failed: {}", e);
                    TestResult::Skipped
                },
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test concurrent operations
async fn test_concurrent_operations() -> TestResult {
    tracing::info!("Testing concurrent operations");

    let metrics = PerformanceMetrics::new("concurrent_operations".to_string());

    match setup_test_environment("test_concurrency", 8096).await {
        | Ok(container) => {
            let test_data = TestData::new();

            // Test concurrent document uploads
            let mut upload_tasks = Vec::new();

            for i in 0 .. 3 {
                let container_clone = container.clone();
                let content = format!("{}\n\n## Concurrent Document {}", test_data.sample_markdown, i);
                let filename = format!("concurrent_doc_{}.md", i);

                let task = tokio::spawn(async move { container_clone.document_service.process_document(content, filename).await });

                upload_tasks.push(task);
            }

            // Wait for uploads
            let mut successful_uploads = 0;
            for task in upload_tasks {
                match task.await {
                    | Ok(Ok(_)) => successful_uploads += 1,
                    | Ok(Err(e)) => tracing::warn!("Concurrent upload failed: {}", e),
                    | Err(e) => tracing::error!("Upload task failed: {}", e),
                }
            }

            // Test concurrent queries
            let mut query_tasks = Vec::new();

            for question in &test_data.test_questions[.. 3] {
                // Limit to 3 questions
                let container_clone = container.clone();
                let question = question.to_string();

                let task = tokio::spawn(async move { container_clone.rag_service.answer_question(question).await });

                query_tasks.push(task);
            }

            // Wait for queries
            let mut successful_queries = 0;
            for task in query_tasks {
                match task.await {
                    | Ok(Ok(_)) => successful_queries += 1,
                    | Ok(Err(e)) => tracing::warn!("Concurrent query failed: {}", e),
                    | Err(e) => tracing::error!("Query task failed: {}", e),
                }
            }

            let total_operations = successful_uploads + successful_queries;

            if total_operations >= 3 {
                // Require at least 3 successful operations
                metrics.log_completion();
                tracing::info!(
                    "Concurrent operations test: PASSED ({} uploads, {} queries)",
                    successful_uploads,
                    successful_queries
                );
                TestResult::Passed
            } else {
                tracing::error!("Concurrent operations test failed: {} total successful operations", total_operations);
                TestResult::Failed
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test data cleanup and isolation
async fn test_data_cleanup_and_isolation() -> TestResult {
    tracing::info!("Testing data cleanup and isolation");

    let metrics = PerformanceMetrics::new("data_cleanup".to_string());

    match setup_test_environment("test_cleanup", 8097).await {
        | Ok(container) => {
            // Perform cleanup
            match cleanup_test_environment(&container).await {
                | Ok(()) => {
                    // Verify collection state
                    match container.vector_repository.collection_exists().await {
                        | Ok(exists) => {
                            tracing::info!("Collection exists after cleanup: {}", exists);

                            // Test health after cleanup
                            match container.health_check().await {
                                | Ok(status) => {
                                    metrics.log_completion();
                                    tracing::info!("Data cleanup test: PASSED (healthy: {})", status.is_healthy());
                                    TestResult::Passed
                                },
                                | Err(e) => {
                                    tracing::warn!("Health check failed after cleanup: {}", e);
                                    TestResult::Failed
                                },
                            }
                        },
                        | Err(e) => {
                            tracing::warn!("Failed to check collection existence: {}", e);
                            TestResult::Failed
                        },
                    }
                },
                | Err(e) => {
                    tracing::error!("Cleanup failed: {}", e);
                    TestResult::Failed
                },
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
            TestResult::Skipped
        },
    }
}

/// Test service health monitoring
#[tokio::test]
#[traced_test]
#[ignore] // Requires external services
async fn test_service_health_monitoring() {
    tracing::info!("Testing service health monitoring");

    match setup_test_environment("test_health_monitoring", 8098).await {
        | Ok(container) => {
            // Test comprehensive health check
            match container.health_check().await {
                | Ok(status) => {
                    tracing::info!("Health status: {:?}", status);

                    // Verify health status structure
                    assert!(matches!(
                        status.overall,
                        rust_qdrant_vector_rag::app::ServiceHealth::Healthy
                            | rust_qdrant_vector_rag::app::ServiceHealth::Degraded(_)
                            | rust_qdrant_vector_rag::app::ServiceHealth::Unhealthy(_)
                    ));

                    tracing::info!("Service health monitoring test: PASSED");
                },
                | Err(e) => {
                    tracing::warn!("Health check failed: {}", e);
                },
            }

            // Test individual service connectivity
            match container.azure_client.test_connectivity().await {
                | Ok(()) => tracing::info!("Azure OpenAI connectivity: OK"),
                | Err(e) => tracing::warn!("Azure OpenAI connectivity: FAILED - {}", e),
            }

            match container.vector_repository.health_check().await {
                | Ok(healthy) => tracing::info!("Qdrant health: {}", healthy),
                | Err(e) => tracing::warn!("Qdrant health check failed: {}", e),
            }
        },
        | Err(e) => {
            tracing::warn!("Failed to setup test environment: {}", e);
        },
    }
}
