use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_error_creation() {
        let error = ServiceError::document_processing("Test error");
        assert_eq!(error.to_string(), "Document processing failed: Test error");
        assert_eq!(error.status_code(), 500);
        assert!(!error.is_retryable());
        assert_eq!(error.category(), "document_processing");
        assert_eq!(error.severity(), "error");
    }

    #[test]
    fn test_service_error_retryable() {
        let network_error = ServiceError::network("Connection failed");
        assert!(network_error.is_retryable());

        let validation_error = ServiceError::validation("Invalid input");
        assert!(!validation_error.is_retryable());

        let rate_limit_error = ServiceError::rate_limit("Too many requests");
        assert!(rate_limit_error.is_retryable());
    }

    #[test]
    fn test_service_error_status_codes() {
        assert_eq!(ServiceError::validation("test").status_code(), 400);
        assert_eq!(ServiceError::authentication("test").status_code(), 401);
        assert_eq!(ServiceError::rate_limit("test").status_code(), 429);
        assert_eq!(ServiceError::internal("test").status_code(), 500);
        assert_eq!(ServiceError::external_api("test").status_code(), 502);
        assert_eq!(ServiceError::network("test").status_code(), 503);
        assert_eq!(ServiceError::database("test").status_code(), 503);
    }

    #[test]
    fn test_service_error_severity_levels() {
        assert_eq!(ServiceError::validation("test").severity(), "warn");
        assert_eq!(ServiceError::authentication("test").severity(), "warn");
        assert_eq!(ServiceError::rate_limit("test").severity(), "error");
        assert_eq!(ServiceError::network("test").severity(), "error");
        assert_eq!(ServiceError::external_api("test").severity(), "error");
        assert_eq!(ServiceError::configuration("test").severity(), "critical");
        assert_eq!(ServiceError::internal("test").severity(), "critical");
    }

    #[test]
    fn test_service_error_context() {
        let error = ServiceError::rate_limit("Too many requests");
        let context = error.context();

        assert_eq!(context.get("error_category"), Some(&"rate_limit".to_string()));
        assert_eq!(context.get("severity"), Some(&"error".to_string()));
        assert_eq!(context.get("retryable"), Some(&"true".to_string()));
        assert_eq!(context.get("status_code"), Some(&"429".to_string()));
        assert_eq!(context.get("retry_after"), Some(&"60".to_string()));
    }

    #[test]
    fn test_error_categorization() {
        let errors = vec![
            (ServiceError::document_processing("test"), "document_processing"),
            (ServiceError::embedding_generation("test"), "embedding_generation"),
            (ServiceError::vector_search("test"), "vector_search"),
            (ServiceError::external_api("test"), "external_api"),
            (ServiceError::configuration("test"), "configuration"),
            (ServiceError::internal("test"), "internal"),
            (ServiceError::validation("test"), "validation"),
            (ServiceError::rate_limit("test"), "rate_limit"),
            (ServiceError::authentication("test"), "authentication"),
            (ServiceError::serialization("test"), "serialization"),
            (ServiceError::network("test"), "network"),
            (ServiceError::database("test"), "database"),
        ];

        for (error, expected_category) in errors {
            assert_eq!(error.category(), expected_category);
        }
    }

    #[test]
    fn test_error_retry_logic() {
        let retryable_errors = vec![
            ServiceError::network("Connection failed"),
            ServiceError::database("Database timeout"),
            ServiceError::external_api("Service unavailable"),
            ServiceError::rate_limit("Too many requests"),
        ];

        let non_retryable_errors = vec![
            ServiceError::validation("Invalid input"),
            ServiceError::authentication("Invalid credentials"),
            ServiceError::configuration("Missing config"),
            ServiceError::internal("Internal error"),
        ];

        for error in retryable_errors {
            assert!(error.is_retryable(), "Error should be retryable: {}", error);
        }

        for error in non_retryable_errors {
            assert!(!error.is_retryable(), "Error should not be retryable: {}", error);
        }
    }

    #[test]
    fn test_error_conversions() {
        // Test std::io::Error conversion
        let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
        let service_error: ServiceError = io_error.into();
        assert!(matches!(service_error, ServiceError::Network(_)));

        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let service_error: ServiceError = io_error.into();
        assert!(matches!(service_error, ServiceError::Authentication(_)));

        let io_error = std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out");
        let service_error: ServiceError = io_error.into();
        assert!(matches!(service_error, ServiceError::Network(_)));
    }

    #[tokio::test]
    async fn test_concurrent_error_handling() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use tokio::task;

        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // Simulate concurrent error creation
        for i in 0 .. 100 {
            let counter_clone = counter.clone();
            let handle = task::spawn(async move {
                let error = ServiceError::internal(format!("Error {}", i));
                counter_clone.fetch_add(1, Ordering::SeqCst);
                error
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let error = handle.await.unwrap();
            assert!(matches!(error, ServiceError::Internal(_)));
        }

        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
}
