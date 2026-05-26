//! Pluggable bypass-provider trait.
//!
//! When set on `CrawlConfig.bypass`, the engine routes URL fetches through
//! this provider instead of the native HTTP / chromiumoxide backends. This
//! is the integration surface for caller-supplied bypass vendors —
//! kreuzcrawl ships no vendor adapters of its own.

use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;

use crate::error::CrawlError;
use crate::http::HttpResponse;

/// Caller-supplied bypass backend. Implementations are responsible for
/// vendor authentication, request shaping, response decoding, and mapping
/// vendor errors into `CrawlError`.
#[async_trait]
pub trait BypassProvider: Send + Sync + fmt::Debug {
    /// Fetch the target URL through the provider, returning a rendered
    /// HTTP response. The body should be the page HTML as the vendor
    /// returns it — the downstream extraction pipeline expects the same
    /// shape as a native or chromiumoxide fetch.
    async fn fetch(&self, url: &str) -> Result<HttpResponse, CrawlError>;

    /// Stable, lowercase vendor identifier used for span attributes and
    /// metrics labels. Must not change across releases.
    fn vendor_name(&self) -> &'static str;
}

/// Convenience type alias used on `CrawlConfig.bypass`.
pub type DynBypassProvider = Arc<dyn BypassProvider>;
