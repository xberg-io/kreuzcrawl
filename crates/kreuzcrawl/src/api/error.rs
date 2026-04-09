//! API error handling.

use crate::error::CrawlError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::types::ApiResponse;

/// API-specific error wrapper.
#[derive(Debug)]
pub struct ApiError {
    /// HTTP status code.
    pub status: StatusCode,
    /// Machine-readable error code.
    pub code: &'static str,
    /// Human-readable error message.
    pub message: String,
}

impl ApiError {
    /// Create a new API error from a status code, error code, and message.
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }

    /// Create a 400 Bad Request error.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)
    }

    /// Create a 404 Not Found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "NOT_FOUND", message)
    }

    /// Create a 500 Internal Server Error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "SERVER_ERROR", message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ApiResponse::<()>::err(self.code, &self.message);
        (self.status, Json(body)).into_response()
    }
}

impl From<CrawlError> for ApiError {
    fn from(error: CrawlError) -> Self {
        let (status, code) = match &error {
            CrawlError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            CrawlError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            CrawlError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            CrawlError::WafBlocked(_) => (StatusCode::FORBIDDEN, "WAF_BLOCKED"),
            CrawlError::Timeout(_) | CrawlError::BrowserTimeout(_) => {
                (StatusCode::GATEWAY_TIMEOUT, "TIMEOUT")
            }
            CrawlError::RateLimited(_) => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED"),
            CrawlError::ServerError(_) => (StatusCode::BAD_GATEWAY, "SERVER_ERROR"),
            CrawlError::BadGateway(_) => (StatusCode::BAD_GATEWAY, "BAD_GATEWAY"),
            CrawlError::Gone(_) => (StatusCode::GONE, "GONE"),
            CrawlError::InvalidConfig(_) => (StatusCode::UNPROCESSABLE_ENTITY, "INVALID_CONFIG"),
            CrawlError::Connection(_) => (StatusCode::INTERNAL_SERVER_ERROR, "CONNECTION_ERROR"),
            CrawlError::Dns(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DNS_ERROR"),
            CrawlError::Ssl(_) => (StatusCode::INTERNAL_SERVER_ERROR, "SSL_ERROR"),
            CrawlError::DataLoss(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATA_LOSS"),
            CrawlError::BrowserError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "BROWSER_ERROR"),
            CrawlError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "SERVER_ERROR"),
        };

        Self {
            status,
            code,
            message: error.to_string(),
        }
    }
}
