//! CrawlEngine composes trait implementations into a crawl pipeline.

mod batch;
mod builder;
mod crawl_loop;

use std::sync::Arc;

use crate::error::CrawlError;
use crate::http::build_client;
use crate::tower::CrawlRequest;
use crate::traits::*;
use crate::types::*;

pub use builder::CrawlEngineBuilder;

/// The main crawl engine, composed of pluggable trait implementations.
#[derive(Clone)]
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
    fn build_service(
        &self,
        client: &reqwest::Client,
    ) -> tower::util::BoxCloneService<CrawlRequest, crate::tower::CrawlResponse, CrawlError> {
        use tower::ServiceBuilder;

        let service = ServiceBuilder::new()
            .layer(crate::tower::PerDomainRateLimitLayer::new(
                self.rate_limiter.clone(),
            ))
            .layer(crate::tower::CrawlCacheLayer::new(self.cache.clone()))
            .layer(self.ua_rotation.clone())
            .service(crate::tower::HttpFetchService::new(
                client.clone(),
                self.config.clone(),
            ));

        #[cfg(feature = "tracing")]
        let service = tower::ServiceBuilder::new()
            .layer(crate::tower::CrawlTracingLayer::new())
            .service(service);

        tower::util::BoxCloneService::new(service)
    }

    /// Scrape a single URL, returning the extracted data.
    ///
    /// Routes the request through the Tower service stack (rate limiting,
    /// UA rotation) then runs the extraction pipeline on the response.
    pub async fn scrape(&self, url: &str) -> Result<ScrapeResult, CrawlError> {
        let client = build_client(&self.config)?;
        let mut service = self.build_service(&client);

        use tower::Service;
        let response = match service.call(CrawlRequest::new(url)).await {
            Ok(resp) => resp,
            Err(CrawlError::NotFound(_)) if self.config.respect_robots_txt => {
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
                    main_content_only: self.config.main_content_only,
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
            Err(e) => return Err(e),
        };

        crate::scrape::scrape_from_crawl_response(url, &response, &self.config).await
    }

    /// Discover all pages on a website by following links and sitemaps.
    pub async fn map(&self, url: &str) -> Result<MapResult, CrawlError> {
        crate::map::map(url, &self.config).await
    }
}
