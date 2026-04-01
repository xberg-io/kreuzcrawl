//! Trait-based extension points for the crawl engine.

use std::net::IpAddr;
use std::time::Duration;

use async_trait::async_trait;

use crate::error::CrawlError;
use crate::types::{CrawlPageResult, ScrapeResult};

/// An entry in the URL frontier queue.
#[derive(Debug, Clone)]
pub struct FrontierEntry {
    pub url: String,
    pub depth: usize,
    /// Priority score for this entry. Higher values mean higher priority.
    pub priority: f64,
}

/// Statistics about an ongoing or completed crawl.
#[derive(Debug, Clone, Default)]
pub struct CrawlStats {
    pub pages_crawled: usize,
    pub pages_failed: usize,
    pub urls_discovered: usize,
    pub urls_filtered: usize,
    pub elapsed: Duration,
}

/// Context passed to middleware before a request.
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub url: String,
    pub headers: std::collections::HashMap<String, String>,
}

/// Context passed to middleware after a response.
#[derive(Debug)]
pub struct ResponseContext {
    pub url: String,
    pub status: u16,
    pub content_type: String,
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
}

/// Events emitted during crawl lifecycle.
#[derive(Debug, Clone)]
pub struct PageEvent {
    pub url: String,
    pub status_code: u16,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct ErrorEvent {
    pub url: String,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct CompleteEvent {
    pub pages_crawled: usize,
}

/// URL queue and deduplication.
#[async_trait]
pub trait Frontier: Send + Sync {
    /// Push a new entry onto the frontier.
    async fn push(&self, entry: FrontierEntry) -> Result<(), CrawlError>;

    /// Pop the next entry from the frontier.
    async fn pop(&self) -> Result<Option<FrontierEntry>, CrawlError>;

    /// Pop up to `n` entries from the frontier.
    async fn pop_batch(&self, n: usize) -> Result<Vec<FrontierEntry>, CrawlError> {
        let mut batch = Vec::with_capacity(n);
        for _ in 0..n {
            match self.pop().await? {
                Some(entry) => batch.push(entry),
                None => break,
            }
        }
        Ok(batch)
    }

    /// Return the number of entries in the frontier.
    async fn len(&self) -> Result<usize, CrawlError>;

    /// Check whether the frontier is empty.
    async fn is_empty(&self) -> Result<bool, CrawlError> {
        Ok(self.len().await? == 0)
    }

    /// Check whether a URL has already been seen.
    async fn is_seen(&self, url: &str) -> Result<bool, CrawlError>;

    /// Mark a URL as seen.
    async fn mark_seen(&self, url: &str) -> Result<(), CrawlError>;
}

/// Per-domain rate limiting / throttling.
#[async_trait]
pub trait RateLimiter: Send + Sync {
    /// Wait until a request to the given domain is permitted.
    async fn acquire(&self, domain: &str) -> Result<(), CrawlError>;

    /// Record a response status for adaptive back-off.
    async fn record_response(&self, domain: &str, status: u16) -> Result<(), CrawlError>;

    /// Set the crawl-delay for a domain (e.g. from robots.txt).
    async fn set_crawl_delay(&self, domain: &str, delay: Duration) -> Result<(), CrawlError>;
}

/// Persistence for crawl results.
#[async_trait]
pub trait CrawlStore: Send + Sync {
    /// Store a successfully scraped page.
    async fn store_page(&self, url: &str, result: &ScrapeResult) -> Result<(), CrawlError>;

    /// Store a crawl page result.
    async fn store_crawl_page(&self, url: &str, result: &CrawlPageResult)
    -> Result<(), CrawlError>;

    /// Store an error encountered while crawling a URL.
    async fn store_error(&self, url: &str, error: &CrawlError) -> Result<(), CrawlError>;

    /// Called once when the crawl completes.
    async fn on_complete(&self, stats: &CrawlStats) -> Result<(), CrawlError>;
}

/// Request/response interceptors.
#[async_trait]
pub trait CrawlMiddleware: Send + Sync {
    /// Called before a request is sent. May mutate headers, URL, etc.
    async fn before_request(&self, ctx: &mut RequestContext) -> Result<(), CrawlError>;

    /// Called after a response is received. May mutate body, headers, etc.
    async fn after_response(&self, ctx: &mut ResponseContext) -> Result<(), CrawlError>;
}

/// Crawl lifecycle event emitter.
#[async_trait]
pub trait EventEmitter: Send + Sync {
    /// A page was crawled.
    async fn on_page(&self, event: &PageEvent);

    /// An error occurred.
    async fn on_error(&self, event: &ErrorEvent);

    /// The crawl completed.
    async fn on_complete(&self, event: &CompleteEvent);

    /// A new URL was discovered.
    async fn on_discovered(&self, url: &str, depth: usize);
}

/// DNS resolution.
#[async_trait]
pub trait DnsResolver: Send + Sync {
    /// Resolve a hostname to one or more IP addresses.
    async fn resolve(&self, host: &str) -> Result<Vec<IpAddr>, CrawlError>;
}

/// Crawl strategy for URL selection and scoring.
///
/// This is a synchronous trait -- implementations must be `Send + Sync`.
pub trait CrawlStrategy: Send + Sync {
    /// Select the next URL to crawl from a set of candidates.
    /// Returns the index into `candidates`, or `None` if none should be selected.
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize>;

    /// Score a URL for prioritisation.
    fn score_url(&self, url: &str, depth: usize) -> f64 {
        let _ = url;
        1.0 / (depth as f64 + 1.0)
    }

    /// Whether the crawl should continue given current stats.
    fn should_continue(&self, stats: &CrawlStats) -> bool {
        let _ = stats;
        true
    }
}

/// Post-extraction content filter.
#[async_trait]
pub trait ContentFilter: Send + Sync {
    /// Filter a crawled page. Return `None` to discard it.
    async fn filter(&self, page: CrawlPageResult) -> Result<Option<CrawlPageResult>, CrawlError>;
}
