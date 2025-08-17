#[derive(Debug, Default, serde::Deserialize)]
struct Payload {
    #[serde(default)]
    name: String,
}

#[derive(Debug, Default, serde::Serialize)]
struct Body {
    #[serde(default)]
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO)
                             .with_target(false)
                             .without_time()
                             .init();

    lambda_http::run(lambda_http::service_fn(handler)).await
}

async fn handler(event: lambda_http::Request)
                 -> Result<lambda_http::Response<lambda_http::Body>, lambda_runtime::Error> {
    let payload: Payload = lambda_http::RequestPayloadExt::payload(&event).unwrap_or_else(|_| None)
                                                                          .unwrap_or_default();

    let body = Body { message: format!("Hello {}, this is an AWS Lambda HTTP POST request",
                                       payload.name).to_owned(), };

    let response = lambda_http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(lambda_http::Body::from(serde_json::to_string(&body)?))
        .map_err(|e| lambda_runtime::Error::from(e))?;

    Ok(response)
}
