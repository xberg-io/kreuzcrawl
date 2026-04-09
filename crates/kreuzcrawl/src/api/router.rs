//! API router setup and configuration.

use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::TraceLayer,
};

use crate::engine::CrawlEngine;

use super::{handlers, state::ApiState};

/// Maximum request body size (10 MB).
const MAX_REQUEST_BODY_BYTES: usize = 10 * 1024 * 1024;

/// Maximum time a request handler may run (5 minutes).
const REQUEST_TIMEOUT: Duration = Duration::from_secs(300);

/// Create the API router with all routes configured.
///
/// This is public to allow users to embed the router in their own applications
/// or to use Tower's `oneshot` for testing.
///
/// # Arguments
///
/// * `engine` - A shared [`CrawlEngine`] that powers scrape, crawl, and map operations.
pub fn create_router(engine: Arc<CrawlEngine>) -> Router {
    let state = Arc::new(ApiState::new(engine));

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Firecrawl v1 endpoints
        .route("/v1/scrape", post(handlers::scrape_handler))
        .route("/v1/crawl", post(handlers::crawl_handler))
        .route(
            "/v1/crawl/{id}",
            get(handlers::crawl_status_handler).delete(handlers::crawl_cancel_handler),
        )
        .route("/v1/map", post(handlers::map_handler))
        .route("/v1/batch/scrape", post(handlers::batch_scrape_handler))
        .route("/v1/batch/scrape/{id}", get(handlers::batch_status_handler))
        .route("/v1/download", post(handlers::download_handler))
        // Operational endpoints
        .route("/health", get(handlers::health_handler))
        .route("/version", get(handlers::version_handler))
        // Middleware stack (outermost first)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(SetSensitiveHeadersLayer::new([AUTHORIZATION]))
        .layer(middleware::from_fn(request_timeout))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BODY_BYTES))
        .layer(cors_layer)
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Middleware that enforces a global request timeout.
///
/// If the inner handler does not complete within [`REQUEST_TIMEOUT`], this
/// returns `408 Request Timeout`.
async fn request_timeout(req: Request, next: Next) -> impl IntoResponse {
    match tokio::time::timeout(REQUEST_TIMEOUT, next.run(req)).await {
        Ok(response) => response,
        Err(_elapsed) => StatusCode::REQUEST_TIMEOUT.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_router() {
        let engine = CrawlEngine::builder()
            .rate_limiter(crate::NoopRateLimiter)
            .build()
            .expect("default engine");
        let _router = create_router(Arc::new(engine));
    }
}
