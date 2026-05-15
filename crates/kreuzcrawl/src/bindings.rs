//! Bridge between kreuzcrawl's trait-based engine and polyglot bindings.
//!
//! The core [`CrawlEngine`] uses `Arc<dyn Trait>` for pluggable components,
//! which cannot cross FFI boundaries. This module provides a config-only
//! construction path with default implementations, plus async adapter
//! functions that alef can generate bindings for.

use crate::engine::CrawlEngine;
use crate::error::CrawlError;
use crate::types::{CrawlConfig, CrawlResult, MapResult, ScrapeResult};
use serde::{Deserialize, Serialize};

/// Opaque handle to a configured crawl engine.
///
/// Constructed via [`create_engine`] with an optional [`CrawlConfig`].
/// Default implementations for all pluggable components are used internally.
#[derive(Clone)]
pub struct CrawlEngineHandle {
    inner: CrawlEngine,
}

/// Result from a single URL in a batch scrape operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchScrapeResult {
    /// The URL that was scraped.
    pub url: String,
    /// The scrape result, if successful.
    pub result: Option<ScrapeResult>,
    /// The error message, if the scrape failed.
    pub error: Option<String>,
}

/// Result from a single URL in a batch crawl operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCrawlResult {
    /// The seed URL that was crawled.
    pub url: String,
    /// The crawl result, if successful.
    pub result: Option<CrawlResult>,
    /// The error message, if the crawl failed.
    pub error: Option<String>,
}

/// Create a new crawl engine with the given configuration.
///
/// If `config` is `None`, uses [`CrawlConfig::default()`].
/// Returns an error if the configuration is invalid.
pub fn create_engine(config: Option<CrawlConfig>) -> Result<CrawlEngineHandle, CrawlError> {
    let mut builder = CrawlEngine::builder();
    if let Some(config) = config {
        config.validate()?;
        builder = builder.config(config);
    }
    let engine = builder.build()?;
    Ok(CrawlEngineHandle { inner: engine })
}

/// Scrape a single URL, returning extracted page data.
pub async fn scrape(engine: &CrawlEngineHandle, url: &str) -> Result<ScrapeResult, CrawlError> {
    engine.inner.scrape(url).await
}

/// Crawl a website starting from `url`, following links up to the configured depth.
pub async fn crawl(engine: &CrawlEngineHandle, url: &str) -> Result<CrawlResult, CrawlError> {
    engine.inner.crawl(url).await
}

/// Discover all pages on a website by following links and sitemaps.
pub async fn map_urls(engine: &CrawlEngineHandle, url: &str) -> Result<MapResult, CrawlError> {
    engine.inner.map(url).await
}

/// Scrape multiple URLs concurrently.
pub async fn batch_scrape(engine: &CrawlEngineHandle, urls: Vec<String>) -> Result<Vec<BatchScrapeResult>, CrawlError> {
    if urls.is_empty() {
        return Err(CrawlError::InvalidConfig("batch_urls must not be empty".into()));
    }
    let url_refs: Vec<&str> = urls.iter().map(String::as_str).collect();
    let results = engine.inner.batch_scrape(&url_refs).await;
    Ok(results
        .into_iter()
        .map(|(url, result)| match result {
            Ok(r) => BatchScrapeResult {
                url,
                result: Some(r),
                error: None,
            },
            Err(e) => BatchScrapeResult {
                url,
                result: None,
                error: Some(e.to_string()),
            },
        })
        .collect())
}

/// Crawl multiple seed URLs concurrently, each following links to configured depth.
pub async fn batch_crawl(engine: &CrawlEngineHandle, urls: Vec<String>) -> Result<Vec<BatchCrawlResult>, CrawlError> {
    if urls.is_empty() {
        return Err(CrawlError::InvalidConfig("batch_urls must not be empty".into()));
    }
    let url_refs: Vec<&str> = urls.iter().map(String::as_str).collect();
    let results = engine.inner.batch_crawl(&url_refs).await;
    Ok(results
        .into_iter()
        .map(|(url, result)| match result {
            Ok(r) => BatchCrawlResult {
                url,
                result: Some(r),
                error: None,
            },
            Err(e) => BatchCrawlResult {
                url,
                result: None,
                error: Some(e.to_string()),
            },
        })
        .collect())
}
