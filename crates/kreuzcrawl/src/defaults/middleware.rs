//! Crawl middleware implementations.

use async_trait::async_trait;

use crate::error::CrawlError;
use crate::traits::{CrawlMiddleware, RequestContext, ResponseContext};

/// Middleware that does nothing -- passes everything through unchanged.
#[derive(Debug, Clone, Default)]
pub struct NoopMiddleware;

#[async_trait]
impl CrawlMiddleware for NoopMiddleware {
    async fn before_request(&self, _ctx: &mut RequestContext) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn after_response(&self, _ctx: &mut ResponseContext) -> Result<(), CrawlError> {
        Ok(())
    }
}
