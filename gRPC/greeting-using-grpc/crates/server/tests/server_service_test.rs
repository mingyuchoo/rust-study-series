use greeting_common::AppResult;
use greeting_server::process_greeting_request;

#[test]
fn test_process_greeting_request_valid() -> AppResult<()> {
    let name = "Test User".to_string();
    let response = process_greeting_request(name.clone())?;
    assert_eq!(response.message, format!("Hello {}!", name));
    Ok(())
}

#[test]
fn test_process_greeting_request_empty() {
    let result = process_greeting_request("".to_string());
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Name cannot be empty"));
    }
}
