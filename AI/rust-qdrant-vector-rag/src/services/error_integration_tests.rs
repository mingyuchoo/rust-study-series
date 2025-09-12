use crate::models::ServiceError;
use crate::services::{ResilienceConfig, ResilienceService};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::sleep;

/// Integration tests for error handling across services
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_error_propagation_chain() {
        // Test that errors propagate correctly through service layers

        // Simulate a chain: RAG -> Embedding -> Azure OpenAI (fails)
        let embedding_error = ServiceError::external_api("Azure OpenAI rate limit exceeded");
        let rag_error = ServiceError::internal(format!("Failed to generate question embedding: {}", embedding_error));

        assert!(matches!(rag_error, ServiceError::Internal(_)));
        assert!(rag_error.to_string().contains("Azure OpenAI rate limit exceeded"));
        assert_eq!(rag_error.severity(), "critical");
        assert!(!rag_error.is_retryable()); // Internal errors are not retryable by default
    }

    #[tokio::test]
    async fn test_resilience_with_different_error_types() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 2,
            base_delay_ms: 10,
            ..Default::default()
        });

        let call_count = Arc::new(AtomicUsize::new(0));

        // Test with retryable error (should retry)
        let call_count_clone = call_count.clone();
        let result = resilience
            .retry_with_backoff(|| {
                let count = call_count_clone.clone();
                async move {
                    let current = count.fetch_add(1, Ordering::SeqCst);
                    if current < 1 {
                        Err(ServiceError::network("Temporary network issue"))
                    } else {
                        Ok("success")
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(call_count.load(Ordering::SeqCst), 2); // Initial + 1 retry

        // Reset counter
        call_count.store(0, Ordering::SeqCst);

        // Test with non-retryable error (should not retry)
        let call_count_clone = call_count.clone();
        let result = resilience
            .retry_with_backoff(|| {
                let count = call_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<&str, ServiceError>(ServiceError::validation("Invalid input"))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // No retries for validation errors
    }

    #[tokio::test]
    async fn test_error_context_preservation() {
        // Test that error context is preserved through transformations
        let original_error = ServiceError::rate_limit("API quota exceeded for embedding model");
        let context = original_error.context();

        assert_eq!(context.get("error_category"), Some(&"rate_limit".to_string()));
        assert_eq!(context.get("retry_after"), Some(&"60".to_string()));
        assert_eq!(context.get("retryable"), Some(&"true".to_string()));

        // Transform the error (as might happen in service layers)
        let transformed_error = ServiceError::embedding_generation(format!("Upstream error: {}", original_error));
        let transformed_context = transformed_error.context();

        assert_eq!(transformed_context.get("error_category"), Some(&"embedding_generation".to_string()));
        assert!(transformed_error.to_string().contains("API quota exceeded"));
    }

    #[tokio::test]
    async fn test_concurrent_error_handling() {
        let resilience = ResilienceService::with_default_config();
        let error_count = Arc::new(AtomicUsize::new(0));
        let success_count = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];

        // Simulate concurrent operations with mixed success/failure
        for i in 0..10 {
            let error_count_clone = error_count.clone();
            let success_count_clone = success_count.clone();
            let resilience_clone = Arc::new(resilience.clone());

            let handle = tokio::spawn(async move {
                let result = resilience_clone
                    .retry_with_backoff(|| async move {
                        if i % 3 == 0 {
                            Err(ServiceError::network("Simulated network error"))
                        } else {
                            Ok(format!("Success {}", i))
                        }
                    })
                    .await;

                match result {
                    | Ok(_) => success_count_clone.fetch_add(1, Ordering::SeqCst),
                    | Err(_) => error_count_clone.fetch_add(1, Ordering::SeqCst),
                };
            });

            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            handle.await.unwrap();
        }

        let total_errors = error_count.load(Ordering::SeqCst);
        let total_successes = success_count.load(Ordering::SeqCst);

        assert_eq!(total_errors + total_successes, 10);
        assert!(total_errors > 0); // Some operations should fail
        assert!(total_successes > 0); // Some operations should succeed
    }

    #[tokio::test]
    async fn test_error_recovery_scenarios() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 3,
            base_delay_ms: 10,
            ..Default::default()
        });

        // Scenario 1: Service recovers after temporary failure
        let attempt_count = Arc::new(AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = resilience
            .retry_with_backoff(|| {
                let count = attempt_count_clone.clone();
                async move {
                    let current = count.fetch_add(1, Ordering::SeqCst);
                    match current {
                        | 0 => Err(ServiceError::network("Connection timeout")),
                        | 1 => Err(ServiceError::external_api("Service temporarily unavailable")),
                        | _ => Ok("Service recovered"),
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Service recovered");
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);

        // Scenario 2: Service never recovers
        attempt_count.store(0, Ordering::SeqCst);
        let attempt_count_clone = attempt_count.clone();

        let result = resilience
            .retry_with_backoff(|| {
                let count = attempt_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<&str, ServiceError>(ServiceError::database("Database connection failed"))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 4); // Initial + 3 retries
    }

    #[tokio::test]
    async fn test_fallback_error_handling() {
        let resilience = ResilienceService::with_default_config();

        // Test successful fallback
        let result = resilience
            .with_fallback(
                || async { Err::<String, ServiceError>(ServiceError::external_api("Primary service down")) },
                || async { Ok("Fallback response".to_string()) },
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Fallback response");

        // Test both primary and fallback fail
        let result = resilience
            .with_fallback(
                || async { Err::<String, ServiceError>(ServiceError::external_api("Primary service down")) },
                || async { Err::<String, ServiceError>(ServiceError::external_api("Fallback also down")) },
            )
            .await;

        assert!(result.is_err());
        // Should return the primary error
        assert!(result.unwrap_err().to_string().contains("Primary service down"));
    }

    #[tokio::test]
    async fn test_timeout_error_handling() {
        let resilience = ResilienceService::new(ResilienceConfig {
            operation_timeout_seconds: 1,
            ..Default::default()
        });

        // Test operation that completes within timeout
        let result = resilience
            .with_timeout(|| async {
                sleep(Duration::from_millis(100)).await;
                Ok::<String, ServiceError>("Completed in time".to_string())
            })
            .await;

        assert!(result.is_ok());

        // Test operation that times out
        let result = resilience
            .with_timeout(|| async {
                sleep(Duration::from_secs(2)).await;
                Ok::<String, ServiceError>("Should not reach here".to_string())
            })
            .await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, ServiceError::Network(_)));
        assert!(error.to_string().contains("timed out"));
    }

    #[tokio::test]
    async fn test_batch_error_handling() {
        let resilience = ResilienceService::with_default_config();
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let results = resilience
            .batch_execute(items, 3, |item| async move {
                if item % 4 == 0 {
                    Err(ServiceError::validation(format!("Item {} is invalid", item)))
                } else if item % 7 == 0 {
                    Err(ServiceError::network(format!("Network error for item {}", item)))
                } else {
                    Ok(item * 2)
                }
            })
            .await;

        assert_eq!(results.len(), 10);

        // Check specific results
        assert_eq!(results[0].as_ref().unwrap(), &2); // 1 * 2
        assert_eq!(results[1].as_ref().unwrap(), &4); // 2 * 2
        assert_eq!(results[2].as_ref().unwrap(), &6); // 3 * 2
        assert!(results[3].is_err()); // 4 % 4 == 0
        assert_eq!(results[4].as_ref().unwrap(), &10); // 5 * 2
        assert_eq!(results[5].as_ref().unwrap(), &12); // 6 * 2
        assert!(results[6].is_err()); // 7 % 7 == 0
        assert!(results[7].is_err()); // 8 % 4 == 0

        // Count successes and failures
        let successes = results.iter().filter(|r| r.is_ok()).count();
        let failures = results.iter().filter(|r| r.is_err()).count();

        assert_eq!(successes + failures, 10);
        assert!(successes > 0);
        assert!(failures > 0);
    }

    #[tokio::test]
    async fn test_health_check_error_scenarios() {
        let resilience = ResilienceService::new(ResilienceConfig {
            max_retries: 2,
            base_delay_ms: 10,
            ..Default::default()
        });

        let attempt_count = Arc::new(AtomicUsize::new(0));

        // Test health check that eventually succeeds
        let attempt_count_clone = attempt_count.clone();
        let result = resilience
            .health_check(|| {
                let count = attempt_count_clone.clone();
                async move {
                    let current = count.fetch_add(1, Ordering::SeqCst);
                    if current < 1 {
                        Err(ServiceError::network("Health check failed"))
                    } else {
                        Ok(true)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        // Test health check that always fails
        attempt_count.store(0, Ordering::SeqCst);
        let attempt_count_clone = attempt_count.clone();
        let result = resilience
            .health_check(|| {
                let count = attempt_count_clone.clone();
                async move {
                    count.fetch_add(1, Ordering::SeqCst);
                    Err::<bool, ServiceError>(ServiceError::database("Database unreachable"))
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false); // Health check returns false instead of error
    }

    #[tokio::test]
    async fn test_error_logging_integration() {
        // This test verifies that errors are properly structured for logging
        let errors = vec![
            ServiceError::document_processing("Failed to parse markdown"),
            ServiceError::embedding_generation("Azure OpenAI API error"),
            ServiceError::vector_search("Qdrant connection timeout"),
            ServiceError::rate_limit("Embedding API quota exceeded"),
            ServiceError::authentication("Invalid API key"),
            ServiceError::validation("Empty document content"),
        ];

        for error in errors {
            let context = error.context();

            // Verify all errors have required logging context
            assert!(context.contains_key("error_category"));
            assert!(context.contains_key("severity"));
            assert!(context.contains_key("retryable"));
            assert!(context.contains_key("status_code"));

            // Verify severity levels are appropriate
            match error.category() {
                | "validation" | "authentication" => assert_eq!(error.severity(), "warn"),
                | "rate_limit" | "network" | "database" | "external_api" => assert_eq!(error.severity(), "error"),
                | "configuration" | "internal" => assert_eq!(error.severity(), "critical"),
                | _ => assert!(["warn", "error", "critical"].contains(&error.severity())),
            }
        }
    }

    #[tokio::test]
    async fn test_error_aggregation() {
        // Test collecting and analyzing multiple errors
        let mut errors = Vec::new();

        // Simulate various service failures
        errors.push(ServiceError::network("Connection timeout to Qdrant"));
        errors.push(ServiceError::external_api("Azure OpenAI rate limit"));
        errors.push(ServiceError::validation("Invalid document format"));
        errors.push(ServiceError::network("DNS resolution failed"));
        errors.push(ServiceError::external_api("Azure OpenAI service unavailable"));

        // Analyze error patterns
        let network_errors = errors.iter().filter(|e| matches!(e, ServiceError::Network(_))).count();
        let api_errors = errors.iter().filter(|e| matches!(e, ServiceError::ExternalAPI(_))).count();
        let validation_errors = errors.iter().filter(|e| matches!(e, ServiceError::Validation(_))).count();

        assert_eq!(network_errors, 2);
        assert_eq!(api_errors, 2);
        assert_eq!(validation_errors, 1);

        // Test retryable vs non-retryable classification
        let retryable_count = errors.iter().filter(|e| e.is_retryable()).count();
        let non_retryable_count = errors.len() - retryable_count;

        assert_eq!(retryable_count, 4); // network and external_api errors
        assert_eq!(non_retryable_count, 1); // validation error
    }

    #[tokio::test]
    async fn test_error_recovery_patterns() {
        let resilience = ResilienceService::with_default_config();

        // Pattern 1: Immediate recovery
        let result = resilience.retry_with_backoff(|| async { Ok::<&str, ServiceError>("immediate success") }).await;
        assert!(result.is_ok());

        // Pattern 2: Recovery after one failure
        let attempt = Arc::new(AtomicUsize::new(0));
        let attempt_clone = attempt.clone();
        let result = resilience
            .retry_with_backoff(|| {
                let count = attempt_clone.clone();
                async move {
                    if count.fetch_add(1, Ordering::SeqCst) == 0 {
                        Err(ServiceError::network("First attempt fails"))
                    } else {
                        Ok("recovered")
                    }
                }
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(attempt.load(Ordering::SeqCst), 2);

        // Pattern 3: Gradual degradation then recovery
        attempt.store(0, Ordering::SeqCst);
        let attempt_clone = attempt.clone();
        let result = resilience
            .retry_with_backoff(|| {
                let count = attempt_clone.clone();
                async move {
                    let current = count.fetch_add(1, Ordering::SeqCst);
                    match current {
                        | 0 => Err(ServiceError::network("Network slow")),
                        | 1 => Err(ServiceError::external_api("Service degraded")),
                        | 2 => Err(ServiceError::rate_limit("Rate limited")),
                        | _ => Ok("fully recovered"),
                    }
                }
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(attempt.load(Ordering::SeqCst), 4);
    }
}
