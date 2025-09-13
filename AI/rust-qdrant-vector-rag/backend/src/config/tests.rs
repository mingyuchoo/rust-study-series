use super::*;
use std::env;

/// Helper function to clear all environment variables used by the config
fn clear_env_vars() {
    let vars_to_clear = [
        "SERVER_HOST",
        "SERVER_PORT",
        "SERVER_MAX_REQUEST_SIZE",
        "SERVER_TIMEOUT_SECONDS",
        "AZURE_OPENAI_ENDPOINT",
        "AZURE_OPENAI_API_KEY",
        "AZURE_OPENAI_API_VERSION",
        "AZURE_OPENAI_CHAT_DEPLOYMENT",
        "AZURE_OPENAI_EMBED_DEPLOYMENT",
        "AZURE_OPENAI_MAX_RETRIES",
        "AZURE_OPENAI_TIMEOUT_SECONDS",
        "QDRANT_URL",
        "QDRANT_API_KEY",
        "QDRANT_COLLECTION_NAME",
        "QDRANT_VECTOR_SIZE",
        "QDRANT_TIMEOUT_SECONDS",
        "QDRANT_MAX_RETRIES",
    ];

    for var in &vars_to_clear {
        env::remove_var(var);
    }
}

/// Helper function to set valid environment variables
fn set_valid_env_vars() {
    env::set_var("AZURE_OPENAI_ENDPOINT", "https://test.openai.azure.com");
    env::set_var("AZURE_OPENAI_API_KEY", "sk-1234567890abcdef1234567890abcdef");
    env::set_var("AZURE_OPENAI_CHAT_DEPLOYMENT", "gpt-4");
    env::set_var("AZURE_OPENAI_EMBED_DEPLOYMENT", "text-embedding-3-large");
}

#[cfg(test)]
mod server_config_tests {
    use super::*;

    #[test]
    fn test_server_config_defaults() {
        clear_env_vars();

        let config = ServerConfig::from_env().unwrap();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_request_size, 10485760); // 10MB
        assert_eq!(config.timeout_seconds, 30);

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_server_config_from_env() {
        clear_env_vars();

        env::set_var("SERVER_HOST", "0.0.0.0");
        env::set_var("SERVER_PORT", "3000");
        env::set_var("SERVER_MAX_REQUEST_SIZE", "5242880"); // 5MB
        env::set_var("SERVER_TIMEOUT_SECONDS", "60");

        let config = ServerConfig::from_env().unwrap();

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.max_request_size, 5242880);
        assert_eq!(config.timeout_seconds, 60);

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_server_config_invalid_port() {
        clear_env_vars();
        env::set_var("SERVER_PORT", "invalid");

        let result = ServerConfig::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid SERVER_PORT"));

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_server_config_validation_invalid_host() {
        let config = ServerConfig {
            host: "invalid-host".to_string(),
            port: 8080,
            max_request_size: 10485760,
            timeout_seconds: 30,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid server host"));
    }

    #[test]
    fn test_server_config_validation_zero_port() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
            max_request_size: 10485760,
            timeout_seconds: 30,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Server port cannot be 0"));
    }

    #[test]
    fn test_server_config_validation_request_size_too_small() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_request_size: 512, // Less than 1KB
            timeout_seconds: 30,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be at least 1024 bytes"));
    }

    #[test]
    fn test_server_config_validation_request_size_too_large() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_request_size: 200_000_000, // More than 100MB
            timeout_seconds: 30,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot exceed 100MB"));
    }

    #[test]
    fn test_server_config_validation_timeout_too_long() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_request_size: 10485760,
            timeout_seconds: 400, // More than 300 seconds
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot exceed 300 seconds"));
    }
}

#[cfg(test)]
mod azure_openai_config_tests {
    use super::*;

    #[test]
    fn test_azure_openai_config_from_env() {
        clear_env_vars();
        set_valid_env_vars();

        let config = AzureOpenAIConfig::from_env().unwrap();

        assert_eq!(config.endpoint, "https://test.openai.azure.com");
        assert_eq!(config.api_key, "sk-1234567890abcdef1234567890abcdef");
        assert_eq!(config.api_version, "2024-02-01");
        assert_eq!(config.chat_deployment, "gpt-4");
        assert_eq!(config.embed_deployment, "text-embedding-3-large");
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_seconds, 60);

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_azure_openai_config_missing_endpoint() {
        clear_env_vars();

        let result = AzureOpenAIConfig::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("AZURE_OPENAI_ENDPOINT is required"));

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_azure_openai_config_missing_api_key() {
        clear_env_vars();
        env::set_var("AZURE_OPENAI_ENDPOINT", "https://test.openai.azure.com");
        env::set_var("AZURE_OPENAI_CHAT_DEPLOYMENT", "gpt-4");
        env::set_var("AZURE_OPENAI_EMBED_DEPLOYMENT", "text-embedding-3-large");

        let result = AzureOpenAIConfig::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("AZURE_OPENAI_API_KEY is required"));

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_azure_openai_config_validation_invalid_url() {
        let config = AzureOpenAIConfig {
            endpoint: "not-a-url".to_string(),
            api_key: "sk-1234567890abcdef1234567890abcdef".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid Azure OpenAI endpoint"));
    }

    #[test]
    fn test_azure_openai_config_validation_non_https() {
        let config = AzureOpenAIConfig {
            endpoint: "http://test.openai.azure.com".to_string(),
            api_key: "sk-1234567890abcdef1234567890abcdef".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must use HTTPS"));
    }

    #[test]
    fn test_azure_openai_config_validation_empty_api_key() {
        let config = AzureOpenAIConfig {
            endpoint: "https://test.openai.azure.com".to_string(),
            api_key: "   ".to_string(), // Only whitespace
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("API key cannot be empty"));
    }

    #[test]
    fn test_azure_openai_config_validation_short_api_key() {
        let config = AzureOpenAIConfig {
            endpoint: "https://test.openai.azure.com".to_string(),
            api_key: "short".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("appears to be invalid"));
    }

    #[test]
    fn test_azure_openai_config_base_url() {
        let config = AzureOpenAIConfig {
            endpoint: "https://test.openai.azure.com/".to_string(), // With trailing slash
            api_key: "sk-1234567890abcdef1234567890abcdef".to_string(),
            api_version: "2024-02-01".to_string(),
            chat_deployment: "gpt-4".to_string(),
            embed_deployment: "text-embedding-3-large".to_string(),
            max_retries: 3,
            timeout_seconds: 60,
        };

        assert_eq!(config.base_url(), "https://test.openai.azure.com/openai");
    }
}

#[cfg(test)]
mod qdrant_config_tests {
    use super::*;

    #[test]
    fn test_qdrant_config_defaults() {
        clear_env_vars();

        let config = QdrantConfig::from_env().unwrap();

        assert_eq!(config.url, "http://localhost:6333");
        assert_eq!(config.api_key, None);
        assert_eq!(config.collection_name, "document_chunks");
        assert_eq!(config.vector_size, 3072);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_qdrant_config_from_env() {
        clear_env_vars();

        env::set_var("QDRANT_URL", "https://qdrant.example.com");
        env::set_var("QDRANT_API_KEY", "test-api-key");
        env::set_var("QDRANT_COLLECTION_NAME", "my_collection");
        env::set_var("QDRANT_VECTOR_SIZE", "1536");
        env::set_var("QDRANT_TIMEOUT_SECONDS", "45");
        env::set_var("QDRANT_MAX_RETRIES", "5");

        let config = QdrantConfig::from_env().unwrap();

        assert_eq!(config.url, "https://qdrant.example.com");
        assert_eq!(config.api_key, Some("test-api-key".to_string()));
        assert_eq!(config.collection_name, "my_collection");
        assert_eq!(config.vector_size, 1536);
        assert_eq!(config.timeout_seconds, 45);
        assert_eq!(config.max_retries, 5);

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_qdrant_config_validation_invalid_url() {
        let config = QdrantConfig {
            url: "not-a-url".to_string(),
            api_key: None,
            collection_name: "test_collection".to_string(),
            vector_size: 1536,
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid Qdrant URL"));
    }

    #[test]
    fn test_qdrant_config_validation_empty_collection_name() {
        let config = QdrantConfig {
            url: "http://localhost:6333".to_string(),
            api_key: None,
            collection_name: "   ".to_string(), // Only whitespace
            vector_size: 1536,
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("collection name cannot be empty"));
    }

    #[test]
    fn test_qdrant_config_validation_invalid_collection_name() {
        let config = QdrantConfig {
            url: "http://localhost:6333".to_string(),
            api_key: None,
            collection_name: "invalid@collection!name".to_string(),
            vector_size: 1536,
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("can only contain alphanumeric"));
    }

    #[test]
    fn test_qdrant_config_validation_zero_vector_size() {
        let config = QdrantConfig {
            url: "http://localhost:6333".to_string(),
            api_key: None,
            collection_name: "test_collection".to_string(),
            vector_size: 0,
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("QDRANT_VECTOR_SIZE must be greater than 0"));
    }

    #[test]
    fn test_qdrant_config_validation_vector_size_too_large() {
        let config = QdrantConfig {
            url: "http://localhost:6333".to_string(),
            api_key: None,
            collection_name: "test_collection".to_string(),
            vector_size: 100000, // Larger than 65536
            timeout_seconds: 30,
            max_retries: 3,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot exceed 65536"));
    }
}

#[cfg(test)]
mod app_config_tests {
    use super::*;

    #[test]
    fn test_app_config_from_env_success() {
        clear_env_vars();
        set_valid_env_vars();

        let config = AppConfig::from_env().unwrap();

        // Verify all components are loaded
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.azure_openai.endpoint, "https://test.openai.azure.com");
        assert_eq!(config.qdrant.url, "http://localhost:6333");

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_app_config_from_env_missing_required() {
        clear_env_vars();
        // Don't set required Azure OpenAI variables

        let result = AppConfig::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("AZURE_OPENAI_ENDPOINT is required"));

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_app_config_server_address() {
        clear_env_vars();
        set_valid_env_vars();
        env::set_var("SERVER_HOST", "0.0.0.0");
        env::set_var("SERVER_PORT", "3000");

        let config = AppConfig::from_env().unwrap();
        let addr = config.server_address().unwrap();

        assert_eq!(addr.to_string(), "0.0.0.0:3000");

        clear_env_vars(); // Clean up after test
    }

    #[test]
    fn test_app_config_server_address_invalid_host() {
        // Create a config directly with invalid host to test server_address method
        let config = AppConfig {
            server: ServerConfig {
                host: "invalid-host".to_string(),
                port: 8080,
                max_request_size: 10485760,
                timeout_seconds: 30,
            },
            azure_openai: AzureOpenAIConfig {
                endpoint: "https://test.openai.azure.com".to_string(),
                api_key: "sk-1234567890abcdef1234567890abcdef".to_string(),
                api_version: "2024-02-01".to_string(),
                chat_deployment: "gpt-4".to_string(),
                embed_deployment: "text-embedding-3-large".to_string(),
                max_retries: 3,
                timeout_seconds: 60,
            },
            qdrant: QdrantConfig {
                url: "http://localhost:6333".to_string(),
                api_key: None,
                collection_name: "test_collection".to_string(),
                vector_size: 1536,
                timeout_seconds: 30,
                max_retries: 3,
            },
        };

        let result = config.server_address();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid server host"));
    }

    #[test]
    fn test_app_config_validation_propagates_errors() {
        // Create a config with invalid Azure OpenAI endpoint
        let config = AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                max_request_size: 10485760,
                timeout_seconds: 30,
            },
            azure_openai: AzureOpenAIConfig {
                endpoint: "not-a-url".to_string(),
                api_key: "sk-1234567890abcdef1234567890abcdef".to_string(),
                api_version: "2024-02-01".to_string(),
                chat_deployment: "gpt-4".to_string(),
                embed_deployment: "text-embedding-3-large".to_string(),
                max_retries: 3,
                timeout_seconds: 60,
            },
            qdrant: QdrantConfig {
                url: "http://localhost:6333".to_string(),
                api_key: None,
                collection_name: "test_collection".to_string(),
                vector_size: 1536,
                timeout_seconds: 30,
                max_retries: 3,
            },
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Azure OpenAI config validation failed"));
    }
}
