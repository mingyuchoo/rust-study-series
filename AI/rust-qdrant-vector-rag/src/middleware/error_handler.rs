use crate::models::ServiceError;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, ResponseError};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::rc::Rc;
use std::time::Instant;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Middleware for handling errors and converting them to proper HTTP responses
pub struct ErrorHandlerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandlerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<B>;
    type Transform = ErrorHandlerMiddlewareService<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorHandlerMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct ErrorHandlerMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> ErrorHandlerMiddlewareService<S> {
    /// Convert Actix-Web errors to ServiceError when possible
    fn convert_actix_error(err: &Error) -> Option<ServiceError> {
        // Handle JSON payload errors
        if let Some(json_err) = err.as_error::<actix_web::error::JsonPayloadError>() {
            return Some(match json_err {
                | actix_web::error::JsonPayloadError::Overflow {
                    limit: _,
                } => ServiceError::validation("Request payload too large"),
                | actix_web::error::JsonPayloadError::ContentType => ServiceError::validation("Invalid content type, expected application/json"),
                | actix_web::error::JsonPayloadError::Deserialize(de_err) => ServiceError::validation(format!("Invalid JSON format: {}", de_err)),
                | _ => ServiceError::validation("Invalid JSON payload"),
            });
        }

        // Handle multipart form errors
        if let Some(_) = err.as_error::<actix_multipart::MultipartError>() {
            return Some(ServiceError::validation("Invalid multipart form data"));
        }

        // Handle path extraction errors
        if let Some(_) = err.as_error::<actix_web::error::PathError>() {
            return Some(ServiceError::validation("Invalid URL path parameters"));
        }

        // Handle query string errors
        if let Some(_) = err.as_error::<actix_web::error::QueryPayloadError>() {
            return Some(ServiceError::validation("Invalid query parameters"));
        }

        // Handle timeout errors
        if err.to_string().contains("timeout") {
            return Some(ServiceError::external_api("Request timeout"));
        }

        // Handle connection errors
        if err.to_string().contains("connection") {
            return Some(ServiceError::network("Connection error"));
        }

        None
    }
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<B>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let start_time = Instant::now();
            let path = req.path().to_string();
            let method = req.method().to_string();
            let request_id = Uuid::new_v4().to_string();
            let user_agent = req.headers().get("user-agent").and_then(|h| h.to_str().ok()).unwrap_or("unknown").to_string();
            let remote_addr = req.connection_info().peer_addr().unwrap_or("unknown").to_string();

            debug!(
                request_id = %request_id,
                method = %method,
                path = %path,
                remote_addr = %remote_addr,
                user_agent = %user_agent,
                "Processing request"
            );

            match service.call(req).await {
                | Ok(mut response) => {
                    let duration = start_time.elapsed();
                    let status = response.status();

                    // Add request ID to response headers
                    if let Ok(header_value) = actix_web::http::header::HeaderValue::from_str(&request_id) {
                        response
                            .headers_mut()
                            .insert(actix_web::http::header::HeaderName::from_static("x-request-id"), header_value);
                    }

                    // Log response with structured data
                    if status.is_success() {
                        info!(
                            request_id = %request_id,
                            method = %method,
                            path = %path,
                            status = %status.as_u16(),
                            duration_ms = %duration.as_millis(),
                            "Request completed successfully"
                        );
                    } else if status.is_client_error() {
                        warn!(
                            request_id = %request_id,
                            method = %method,
                            path = %path,
                            status = %status.as_u16(),
                            duration_ms = %duration.as_millis(),
                            remote_addr = %remote_addr,
                            "Client error response"
                        );
                    } else if status.is_server_error() {
                        error!(
                            request_id = %request_id,
                            method = %method,
                            path = %path,
                            status = %status.as_u16(),
                            duration_ms = %duration.as_millis(),
                            remote_addr = %remote_addr,
                            "Server error response"
                        );
                    }

                    Ok(response)
                },
                | Err(err) => {
                    let duration = start_time.elapsed();

                    // Convert Actix-Web errors to our ServiceError format when possible
                    let service_error = Self::convert_actix_error(&err);

                    // Log with structured error context
                    let error_context = service_error.as_ref().map(|e| e.context()).unwrap_or_default();

                    error!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        duration_ms = %duration.as_millis(),
                        remote_addr = %remote_addr,
                        user_agent = %user_agent,
                        error = %err,
                        error_type = ?service_error,
                        error_context = ?error_context,
                        "Request failed with error"
                    );

                    // Return structured error response instead of generic Actix error
                    match service_error {
                        | Some(service_err) => {
                            let mut response = service_err.error_response();
                            if let Ok(header_value) = actix_web::http::header::HeaderValue::from_str(&request_id) {
                                response
                                    .headers_mut()
                                    .insert(actix_web::http::header::HeaderName::from_static("x-request-id"), header_value);
                            }
                            Err(actix_web::error::ErrorInternalServerError("Service error"))
                        },
                        | None => Err(err), // Pass through unhandled errors
                    }
                },
            }
        })
    }
}
