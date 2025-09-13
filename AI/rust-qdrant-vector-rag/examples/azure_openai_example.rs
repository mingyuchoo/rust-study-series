use rust_qdrant_vector_rag::clients::AzureOpenAIClient;
use rust_qdrant_vector_rag::config::AzureOpenAIConfig;
use rust_qdrant_vector_rag::services::embedding::{EmbeddingService, EmbeddingServiceImpl};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Azure OpenAI Client Example");
    println!("============================");

    // Check if we have the required environment variables
    let endpoint = env::var("AZURE_OPENAI_ENDPOINT").unwrap_or_else(|_| "https://your-resource.openai.azure.com".to_string());
    let api_key = env::var("AZURE_OPENAI_API_KEY").unwrap_or_else(|_| "your-api-key-here".to_string());
    let embed_deployment = env::var("AZURE_OPENAI_EMBED_DEPLOYMENT").unwrap_or_else(|_| "text-embedding-3-large".to_string());
    let chat_deployment = env::var("AZURE_OPENAI_CHAT_DEPLOYMENT").unwrap_or_else(|_| "gpt-4".to_string());

    if api_key == "your-api-key-here" {
        println!("⚠️  This example requires real Azure OpenAI credentials.");
        println!("   Set the following environment variables:");
        println!("   - AZURE_OPENAI_ENDPOINT");
        println!("   - AZURE_OPENAI_API_KEY");
        println!("   - AZURE_OPENAI_EMBED_DEPLOYMENT");
        println!("   - AZURE_OPENAI_CHAT_DEPLOYMENT");
        println!();
        println!("   Example:");
        println!("   export AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com");
        println!("   export AZURE_OPENAI_API_KEY=your-api-key");
        println!("   export AZURE_OPENAI_EMBED_DEPLOYMENT=text-embedding-3-large");
        println!("   export AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4");
        println!();
        println!("   Then run: cargo run --example azure_openai_example");
        return Ok(());
    }

    // Create configuration
    let config = AzureOpenAIConfig {
        endpoint,
        api_key,
        api_version: "2024-02-01".to_string(),
        chat_deployment,
        embed_deployment,
        max_retries: 3,
        timeout_seconds: 60,
    };

    println!("🔧 Creating Azure OpenAI client...");

    // Create the client
    let azure_client = match AzureOpenAIClient::new(config) {
        | Ok(client) => {
            println!("✅ Azure OpenAI client created successfully");
            client
        },
        | Err(e) => {
            println!("❌ Failed to create Azure OpenAI client: {}", e);
            return Err(e.into());
        },
    };

    // Test connectivity
    println!("🔍 Testing connectivity...");
    match azure_client.test_connectivity().await {
        | Ok(()) => println!("✅ Connectivity test passed"),
        | Err(e) => {
            println!("❌ Connectivity test failed: {}", e);
            return Err(e.into());
        },
    }

    // Create embedding service
    println!("🔧 Creating embedding service...");
    let embedding_service = EmbeddingServiceImpl::new(azure_client.clone());

    // Test single embedding generation
    println!("📝 Generating embedding for single text...");
    let test_text = "This is a test sentence for embedding generation.";
    match embedding_service.generate_embedding(test_text).await {
        | Ok(embedding) => {
            println!("✅ Generated embedding with {} dimensions", embedding.len());
            println!("   First 5 values: {:?}", &embedding[.. 5.min(embedding.len())]);
        },
        | Err(e) => {
            println!("❌ Failed to generate embedding: {}", e);
            return Err(e.into());
        },
    }

    // Test batch embedding generation
    println!("📝 Generating embeddings for batch of texts...");
    let test_texts = vec![
        "First test sentence.",
        "Second test sentence with different content.",
        "Third sentence about machine learning and AI.",
    ];

    match embedding_service.generate_embeddings_batch(test_texts.clone()).await {
        | Ok(embeddings) => {
            println!("✅ Generated {} embeddings in batch", embeddings.len());
            for (i, embedding) in embeddings.iter().enumerate() {
                println!("   Text {}: {} dimensions", i + 1, embedding.len());
            }
        },
        | Err(e) => {
            println!("❌ Failed to generate batch embeddings: {}", e);
            return Err(e.into());
        },
    }

    // Test chat completion
    println!("💬 Testing chat completion...");
    let messages = vec![rust_qdrant_vector_rag::clients::azure_openai::ChatMessage {
        role: "user".to_string(),
        content: "Explain what vector embeddings are in one sentence.".to_string(),
    }];

    match azure_client.generate_chat_completion(messages, Some(100), Some(0.7)).await {
        | Ok(response) => {
            println!("✅ Generated chat completion:");
            println!("   Response: {}", response);
        },
        | Err(e) => {
            println!("❌ Failed to generate chat completion: {}", e);
            return Err(e.into());
        },
    }

    println!();
    println!("🎉 All tests completed successfully!");
    println!("   The Azure OpenAI client integration is working correctly.");

    Ok(())
}
