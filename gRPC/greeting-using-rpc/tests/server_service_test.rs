use greeting_using_rpc::error::AppResult;
use greeting_using_rpc::server_service::process_greeting_request;

#[tokio::test]
async fn test_process_greeting_request_valid() -> AppResult<()> {
    // Test with a valid name
    let name = "Test User".to_string();
    let response = process_greeting_request(name.clone()).await?;
    
    // Verify the response
    assert_eq!(response.message, format!("Hello {}!", name));
    Ok(())
}

#[tokio::test]
async fn test_process_greeting_request_empty() {
    // Test with an empty name (should return an error)
    let result = process_greeting_request("".to_string()).await;
    
    // Verify we got an error
    assert!(result.is_err());
    
    // Verify it's the right kind of error
    if let Err(err) = result {
        let err_string = err.to_string();
        assert!(err_string.contains("Name cannot be empty"));
    }
}
