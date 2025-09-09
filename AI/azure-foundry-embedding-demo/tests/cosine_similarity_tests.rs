use azure_foundry_embedding_demo::AzureEmbeddingClient;

#[test]
fn test_cosine_similarity_basic() {
    let a = vec![1.0_f32, 2.0, 3.0];
    let b = vec![4.0_f32, 5.0, 6.0];
    let similarity = AzureEmbeddingClient::cosine_similarity(&a, &b);
    assert!(similarity > 0.0);
    assert!(similarity <= 1.0);
}

#[test]
fn test_cosine_similarity_identical() {
    let a = vec![1.0_f32, 2.0, 3.0];
    let similarity = AzureEmbeddingClient::cosine_similarity(&a, &a);
    assert!((similarity - 1.0).abs() < 1e-6);
}
