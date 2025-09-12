use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{Ready, ready},
    rc::Rc,
    time::Instant,
};
use tracing::{info, warn};

/// Middleware for logging HTTP requests and responses
pub struct RequestLoggerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestLoggerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct RequestLoggerMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let start_time = Instant::now();

        Box::pin(async move {
            let method = req.method().to_string();
            let path = req.path().to_string();
            let query = req.query_string();
            let user_agent = req.headers().get("user-agent").and_then(|h| h.to_str().ok()).unwrap_or("unknown");
            let remote_addr = req.connection_info().peer_addr().unwrap_or("unknown").to_string();

            // Log incoming request
            info!(
                "Incoming request: {} {} {} from {} ({})",
                method,
                path,
                if query.is_empty() { "" } else { &format!("?{}", query) },
                remote_addr,
                user_agent
            );

            // Process request
            let result = service.call(req).await;

            // Calculate processing time
            let duration = start_time.elapsed();

            match &result {
                | Ok(response) => {
                    let status = response.status();
                    let status_code = status.as_u16();

                    if status.is_success() {
                        info!(
                            "Request completed: {} {} -> {} ({:.2}ms)",
                            method,
                            path,
                            status_code,
                            duration.as_secs_f64() * 1000.0
                        );
                    } else if status.is_client_error() {
                        warn!(
                            "Client error: {} {} -> {} ({:.2}ms)",
                            method,
                            path,
                            status_code,
                            duration.as_secs_f64() * 1000.0
                        );
                    } else if status.is_server_error() {
                        warn!(
                            "Server error: {} {} -> {} ({:.2}ms)",
                            method,
                            path,
                            status_code,
                            duration.as_secs_f64() * 1000.0
                        );
                    }
                },
                | Err(err) => {
                    warn!(
                        "Request failed: {} {} -> Error: {} ({:.2}ms)",
                        method,
                        path,
                        err,
                        duration.as_secs_f64() * 1000.0
                    );
                },
            }

            result
        })
    }
}
