//! A no-op crawl store that discards all data.

use async_trait::async_trait;

use crate::error::CrawlError;
use crate::traits::{CrawlStats, CrawlStore};
use crate::types::{CrawlPageResult, ScrapeResult};

/// A store that does nothing -- crawl results are discarded.
#[derive(Debug, Clone, Default)]
pub struct NoopStore;

#[async_trait]
impl CrawlStore for NoopStore {
    async fn store_page(&self, _url: &str, _result: &ScrapeResult) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn store_crawl_page(
        &self,
        _url: &str,
        _result: &CrawlPageResult,
    ) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn store_error(&self, _url: &str, _error: &CrawlError) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn on_complete(&self, _stats: &CrawlStats) -> Result<(), CrawlError> {
        Ok(())
    }
}
