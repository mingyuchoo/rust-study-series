use crate::middleware::{ErrorHandlerMiddleware, RequestLoggerMiddleware};
use actix_web::{App, HttpResponse, Result, test, web};

/// Test handler that returns success
async fn success_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "success"
    })))
}

/// Test handler that returns an error
async fn error_handler() -> Result<HttpResponse> { Err(actix_web::error::ErrorInternalServerError("Test error")) }

#[actix_web::test]
async fn test_request_logger_middleware_success() {
    let app = test::init_service(App::new().wrap(RequestLoggerMiddleware).route("/success", web::get().to(success_handler))).await;

    let req = test::TestRequest::get().uri("/success").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["message"], "success");
}

#[actix_web::test]
async fn test_request_logger_middleware_with_query_params() {
    let app = test::init_service(App::new().wrap(RequestLoggerMiddleware).route("/success", web::get().to(success_handler))).await;

    let req = test::TestRequest::get()
        .uri("/success?param1=value1&param2=value2")
        .insert_header(("User-Agent", "test-agent"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_error_handler_middleware() {
    let app = test::init_service(App::new().wrap(ErrorHandlerMiddleware).route("/error", web::get().to(error_handler))).await;

    let req = test::TestRequest::get().uri("/error").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_server_error());
}

#[actix_web::test]
async fn test_combined_middleware() {
    let app = test::init_service(
        App::new()
            .wrap(ErrorHandlerMiddleware)
            .wrap(RequestLoggerMiddleware)
            .route("/success", web::get().to(success_handler))
            .route("/error", web::get().to(error_handler)),
    )
    .await;

    // Test success case
    let req = test::TestRequest::get().uri("/success").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test error case
    let req = test::TestRequest::get().uri("/error").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_server_error());
}
