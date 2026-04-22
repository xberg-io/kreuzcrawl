//! Adapter trait and input/output types for pluggable scraping backends.

use std::time::Duration;

use async_trait::async_trait;

use crate::Result;

/// Input supplied to a scrape adapter for a single URL.
#[derive(Debug, Clone)]
pub struct ScrapeInput {
    /// The URL to fetch and extract content from.
    pub url: String,
    /// Pre-fetched HTML, if running in cached mode.
    pub cached_html: Option<String>,
}

/// Output produced by a scrape adapter for a single URL.
#[derive(Debug, Clone)]
pub struct ScrapeOutput {
    /// HTTP status code returned by the server (or simulated for cached runs).
    pub status_code: u16,
    /// Extracted content in the configured output format (markdown, plain, djot).
    pub content: Option<String>,
    /// Raw HTML as received or loaded from cache.
    pub html: String,
    /// Size of the extracted content in bytes.
    pub content_size: usize,
    /// Whether a headless browser was used for this request.
    pub browser_used: bool,
    /// Whether the adapter detected that JavaScript rendering was required.
    pub js_render_hint: bool,
    /// Error message if the scrape partially failed but still returned data.
    pub error: Option<String>,
}

/// A pluggable scraping backend.
///
/// Implementors wrap a specific framework (e.g., the native kreuzcrawl engine,
/// an external HTTP service, or a third-party library) so that the harness can
/// drive any backend through a uniform interface.
#[async_trait]
pub trait ScrapeAdapter: Send + Sync {
    /// Short, human-readable name for this adapter (e.g., `"kreuzcrawl-native"`).
    fn name(&self) -> &str;

    /// Version string for the underlying framework, if available.
    fn version(&self) -> String {
        "unknown".to_string()
    }

    /// Called once before any scrape operations.  May be used to warm up
    /// connection pools, launch browser processes, etc.
    async fn setup(&self) -> Result<()> {
        Ok(())
    }

    /// Called once after all scrape operations complete.  Must release all
    /// resources acquired during [`setup`](Self::setup).
    async fn teardown(&self) -> Result<()> {
        Ok(())
    }

    /// Whether this adapter supports batched scraping via [`batch_scrape`](Self::batch_scrape).
    fn supports_batch(&self) -> bool {
        false
    }

    /// Scrape a single URL and return the extracted output.
    ///
    /// `cached_html` is `Some` when the harness is running in cached mode and
    /// the adapter should parse the supplied HTML instead of making a network
    /// request.
    async fn scrape(
        &self,
        url: &str,
        cached_html: Option<&str>,
        timeout: Duration,
    ) -> Result<ScrapeOutput>;

    /// Scrape multiple URLs in a single adapter call.
    ///
    /// The default implementation serialises calls to [`scrape`](Self::scrape).
    /// Adapters that can parallelise internally should override this.
    async fn batch_scrape(
        &self,
        entries: &[ScrapeInput],
        timeout: Duration,
    ) -> Result<Vec<ScrapeOutput>> {
        let mut outputs = Vec::with_capacity(entries.len());
        for entry in entries {
            let out = self
                .scrape(&entry.url, entry.cached_html.as_deref(), timeout)
                .await?;
            outputs.push(out);
        }
        Ok(outputs)
    }
}
