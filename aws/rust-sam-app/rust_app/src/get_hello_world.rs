#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    lambda_http::run(lambda_http::service_fn(handler)).await
}

async fn handler(
    event: lambda_http::Request,
) -> Result<lambda_http::Response<lambda_http::Body>, lambda_http::Error> {
    let who = lambda_http::RequestExt::query_string_parameters_ref(&event)
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    let response = lambda_http::Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;

    Ok(response)
}
