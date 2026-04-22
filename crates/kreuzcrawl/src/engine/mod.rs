//! CrawlEngine composes trait implementations into a crawl pipeline.

#[cfg(not(target_arch = "wasm32"))]
mod batch;
mod builder;
#[cfg(not(target_arch = "wasm32"))]
mod crawl_loop;

use std::sync::Arc;

use crate::error::CrawlError;
#[cfg(not(target_arch = "wasm32"))]
use crate::tower::CrawlRequest;
use crate::traits::*;
use crate::types::*;

pub use builder::CrawlEngineBuilder;

/// The main crawl engine, composed of pluggable trait implementations.
#[derive(Clone)]
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub struct CrawlEngine {
    pub(crate) config: CrawlConfig,
    pub(crate) frontier: Arc<dyn Frontier>,
    pub(crate) rate_limiter: Arc<dyn RateLimiter>,
    pub(crate) store: Arc<dyn CrawlStore>,
    pub(crate) event_emitter: Arc<dyn EventEmitter>,
    pub(crate) strategy: Arc<dyn CrawlStrategy>,
    pub(crate) content_filter: Arc<dyn ContentFilter>,
    pub(crate) cache: Arc<dyn CrawlCache>,
    /// Shared UA rotation layer — preserves rotation counter across service builds.
    #[cfg(not(target_arch = "wasm32"))]
    ua_rotation: crate::tower::UaRotationLayer,
}

impl CrawlEngine {
    /// Create a new [`CrawlEngineBuilder`].
    pub fn builder() -> CrawlEngineBuilder {
        CrawlEngineBuilder::new()
    }

    /// Build the Tower service stack for HTTP fetching.
    ///
    /// Layers (outermost to innermost):
    /// 1. Per-domain rate limiting
    /// 2. HTTP response caching
    /// 3. User-agent rotation
    /// 4. Base HTTP fetch
    #[cfg(not(target_arch = "wasm32"))]
    fn build_service(
        &self,
        client: &reqwest::Client,
    ) -> tower::util::BoxCloneService<CrawlRequest, crate::tower::CrawlResponse, CrawlError> {
        use tower::ServiceBuilder;

        let service = ServiceBuilder::new()
            .layer(crate::tower::PerDomainRateLimitLayer::new(self.rate_limiter.clone()))
            .layer(crate::tower::CrawlCacheLayer::new(self.cache.clone()))
            .layer(self.ua_rotation.clone())
            .service(crate::tower::HttpFetchService::new(client.clone(), self.config.clone()));

        #[cfg(feature = "tracing")]
        let service = tower::ServiceBuilder::new()
            .layer(crate::tower::CrawlTracingLayer::new())
            .service(service);

        tower::util::BoxCloneService::new(service)
    }

    /// Fetch a URL through the appropriate path (Tower stack or browser) and
    /// return the `CrawlResponse` together with a flag indicating whether the
    /// browser was used.
    ///
    /// This is intentionally `#[cfg(not(target_arch = "wasm32"))]`-only: wasm
    /// has its own simpler inline path inside `scrape`.
    #[cfg(not(target_arch = "wasm32"))]
    async fn fetch_response(&self, url: &str) -> Result<(crate::tower::CrawlResponse, bool), CrawlError> {
        use crate::tower::CrawlResponse;

        /// Convert an `HttpResponse` (from the browser path) into the `CrawlResponse`
        /// shape expected by the extraction pipeline. Browser fetches do not carry
        /// HTTP response headers, so we supply an empty map.
        #[cfg(feature = "browser")]
        fn browser_http_to_crawl(r: crate::http::HttpResponse) -> CrawlResponse {
            CrawlResponse {
                status: r.status,
                content_type: r.content_type,
                body: r.body,
                body_bytes: r.body_bytes,
                headers: std::collections::HashMap::new(),
            }
        }

        // BrowserMode::Always — skip HTTP entirely.
        #[cfg(feature = "browser")]
        if self.config.browser.mode == crate::types::BrowserMode::Always {
            let pool = self.config.browser_pool.as_deref();
            let http_resp = crate::browser::browser_fetch(url, &self.config, None, pool).await?;
            return Ok((browser_http_to_crawl(http_resp), true));
        }

        // Tower HTTP stack (with optional WAF fallback).
        let client = crate::http::build_client(&self.config)?;
        let mut service = self.build_service(&client);
        use tower::Service;
        match service.call(CrawlRequest::new(url)).await {
            Ok(resp) => Ok((resp, false)),
            Err(CrawlError::NotFound(_)) if self.config.respect_robots_txt => {
                // Return a sentinel response; the scrape() caller recognises status 404
                // and short-circuits with a minimal result (preserving prior behaviour).
                Ok((
                    CrawlResponse {
                        status: 404,
                        content_type: String::new(),
                        body: String::new(),
                        body_bytes: Vec::new(),
                        headers: std::collections::HashMap::new(),
                    },
                    false,
                ))
            }
            // WAF fallback: delegate to browser when mode is Auto.
            #[cfg(feature = "browser")]
            Err(CrawlError::WafBlocked(_)) if self.config.browser.mode == crate::types::BrowserMode::Auto => {
                let pool = self.config.browser_pool.as_deref();
                let http_resp = crate::browser::browser_fetch(url, &self.config, None, pool).await?;
                Ok((browser_http_to_crawl(http_resp), true))
            }
            Err(e) => Err(e),
        }
    }

    /// Scrape a single URL, returning the extracted data.
    ///
    /// On native targets, routes the request through the Tower service stack
    /// (rate limiting, UA rotation) then runs the extraction pipeline.
    /// On wasm, performs a direct HTTP fetch without the Tower stack.
    ///
    /// Browser fallback behaviour (native + `browser` feature only):
    /// - `BrowserMode::Always`: skips HTTP entirely, goes straight to headless Chrome.
    /// - `BrowserMode::Auto` + WAF blocked: falls back to headless Chrome when the
    ///   Tower stack returns `CrawlError::WafBlocked`.
    /// - `BrowserMode::Auto` + JS detected: after extraction, if `js_render_hint` is
    ///   `true` and the browser has not been used yet, re-fetches with headless Chrome
    ///   and re-runs the extraction pipeline on the rendered HTML.
    pub async fn scrape(&self, url: &str) -> Result<ScrapeResult, CrawlError> {
        self.config.validate()?;

        #[cfg(not(target_arch = "wasm32"))]
        let (response, browser_used_for_fetch) = {
            let (resp, used_browser) = self.fetch_response(url).await?;
            // Preserve the original early-return for 404 when robots.txt is respected:
            // skip extraction entirely and return a minimal result.
            if resp.status == 404 && self.config.respect_robots_txt {
                return Ok(ScrapeResult {
                    status_code: 404,
                    content_type: String::new(),
                    html: String::new(),
                    body_size: 0,
                    metadata: PageMetadata::default(),
                    links: Vec::new(),
                    images: Vec::new(),
                    feeds: Vec::new(),
                    json_ld: Vec::new(),
                    is_allowed: true,
                    crawl_delay: None,
                    noindex_detected: false,
                    nofollow_detected: false,
                    x_robots_tag: None,
                    is_pdf: false,
                    was_skipped: false,
                    detected_charset: None,
                    auth_header_sent: self.config.auth.is_some(),
                    response_meta: None,
                    assets: Vec::new(),
                    js_render_hint: false,
                    browser_used: false,
                    markdown: None,
                    extracted_data: None,
                    extraction_meta: None,
                    screenshot: None,
                    downloaded_document: None,
                });
            }
            (resp, used_browser)
        };

        #[cfg(target_arch = "wasm32")]
        let (response, browser_used_for_fetch) = {
            let client = crate::http::build_client(&self.config)?;
            let resp =
                crate::http::fetch_with_retry(url, &self.config, &std::collections::HashMap::new(), &client).await?;
            let headers = std::collections::HashMap::new();
            // fetch_with_retry returns HttpResponse; convert to CrawlResponse
            let crawl_resp = crate::tower::CrawlResponse {
                status: resp.status,
                content_type: resp.content_type,
                body: resp.body,
                body_bytes: resp.body_bytes,
                headers,
            };
            (crawl_resp, false)
        };

        let mut result = crate::scrape::scrape_from_crawl_response(url, &response, &self.config).await?;
        result.browser_used = browser_used_for_fetch;

        // JS-render fallback: if extraction detected JS-heavy content and we have
        // not already used the browser, re-fetch with headless Chrome and re-extract.
        #[cfg(all(not(target_arch = "wasm32"), feature = "browser"))]
        if result.js_render_hint && !result.browser_used && self.config.browser.mode == crate::types::BrowserMode::Auto
        {
            let pool = self.config.browser_pool.as_deref();
            let http_resp = crate::browser::browser_fetch(url, &self.config, None, pool).await?;
            let crawl_resp = crate::tower::CrawlResponse {
                status: http_resp.status,
                content_type: http_resp.content_type,
                body: http_resp.body,
                body_bytes: http_resp.body_bytes,
                headers: std::collections::HashMap::new(),
            };
            result = crate::scrape::scrape_from_crawl_response(url, &crawl_resp, &self.config).await?;
            result.browser_used = true;
        }

        Ok(result)
    }

    /// Discover all pages on a website by following links and sitemaps.
    pub async fn map(&self, url: &str) -> Result<MapResult, CrawlError> {
        self.config.validate()?;
        crate::map::map(url, &self.config).await
    }
}

/// Wasm-specific sequential batch implementations.
#[cfg(target_arch = "wasm32")]
impl CrawlEngine {
    /// Crawl a website starting from `url`. On wasm, performs a single-page scrape
    /// since the full crawl loop requires concurrency primitives not available on wasm.
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, CrawlError> {
        // Simplified single-page crawl for wasm
        let scrape = self.scrape(url).await?;
        let page = CrawlPageResult {
            url: url.to_owned(),
            normalized_url: crate::normalize::normalize_url(url),
            status_code: scrape.status_code,
            content_type: scrape.content_type,
            html: scrape.html,
            body_size: scrape.body_size,
            metadata: scrape.metadata,
            links: scrape.links,
            images: scrape.images,
            feeds: scrape.feeds,
            json_ld: scrape.json_ld,
            depth: 0,
            stayed_on_domain: true,
            was_skipped: scrape.was_skipped,
            is_pdf: scrape.is_pdf,
            detected_charset: scrape.detected_charset,
            markdown: scrape.markdown,
            extracted_data: scrape.extracted_data,
            extraction_meta: scrape.extraction_meta,
            downloaded_document: scrape.downloaded_document,
        };
        Ok(CrawlResult::new(
            vec![page],
            url.to_owned(),
            0,
            false,
            None,
            Vec::new(),
            vec![crate::normalize::normalize_url(url)],
        ))
    }

    /// Scrape multiple URLs sequentially (no concurrency on wasm).
    pub async fn batch_scrape(&self, urls: &[&str]) -> Vec<(String, Result<ScrapeResult, CrawlError>)> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            let result = self.scrape(url).await;
            results.push((url.to_string(), result));
        }
        results
    }

    /// Crawl multiple seed URLs sequentially (no concurrency on wasm).
    pub async fn batch_crawl(&self, urls: &[&str]) -> Vec<(String, Result<CrawlResult, CrawlError>)> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            let result = self.crawl(url).await;
            results.push((url.to_string(), result));
        }
        results
    }
}
