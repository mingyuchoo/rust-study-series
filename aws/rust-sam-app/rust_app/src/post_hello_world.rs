#[derive(Debug, Default, serde::Deserialize)]
struct Args {
    #[serde(default)]
    name: String,
}

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
    request: lambda_http::Request,
) -> Result<lambda_http::Response<lambda_http::Body>, lambda_runtime::Error> {
    let args: Args = lambda_http::RequestPayloadExt::payload(&request)
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    let message = format!("{}", args.name);

    let response = lambda_http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;

    Ok(response)
}
