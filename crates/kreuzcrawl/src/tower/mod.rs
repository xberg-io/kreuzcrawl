//! Tower-based service stack for HTTP request processing.
//!
//! Provides composable middleware layers following the Tower Service pattern,
//! consistent with liter-llm and kreuzberg.
//!
//! The Tower service stack (cache, rate_limit, service, ua_rotation layers)
//! requires `Send` bounds and is not available on `wasm32` targets.

#[cfg(not(target_arch = "wasm32"))]
mod cache;
#[cfg(not(target_arch = "wasm32"))]
mod rate_limit;
#[cfg(not(target_arch = "wasm32"))]
mod service;
#[cfg(all(not(target_arch = "wasm32"), feature = "tracing"))]
mod tracing_layer;
mod types;
#[cfg(not(target_arch = "wasm32"))]
mod ua_rotation;

#[cfg(not(target_arch = "wasm32"))]
pub use cache::CrawlCacheLayer;
#[cfg(not(target_arch = "wasm32"))]
pub use rate_limit::PerDomainRateLimitLayer;
#[cfg(not(target_arch = "wasm32"))]
pub use service::HttpFetchService;
#[cfg(all(not(target_arch = "wasm32"), feature = "tracing"))]
pub use tracing_layer::CrawlTracingLayer;
#[cfg(not(target_arch = "wasm32"))]
pub use types::CrawlRequest;
pub use types::CrawlResponse;
#[cfg(not(target_arch = "wasm32"))]
pub use ua_rotation::UaRotationLayer;
