//! Builder for [`CrawlEngine`].

use std::sync::Arc;

use crate::defaults;
use crate::error::CrawlError;
use crate::traits::*;
use crate::types::*;

use super::CrawlEngine;

/// Builder for [`CrawlEngine`].
///
/// Any field left unset will be filled with a default implementation
/// from the crate's internal `defaults` module.
pub struct CrawlEngineBuilder {
    config: Option<CrawlConfig>,
    frontier: Option<Arc<dyn Frontier>>,
    rate_limiter: Option<Arc<dyn RateLimiter>>,
    store: Option<Arc<dyn CrawlStore>>,
    event_emitter: Option<Arc<dyn EventEmitter>>,
    strategy: Option<Arc<dyn CrawlStrategy>>,
    content_filter: Option<Arc<dyn ContentFilter>>,
    cache: Option<Arc<dyn CrawlCache>>,
}

impl CrawlEngineBuilder {
    /// Create a new builder with no fields set.
    pub fn new() -> Self {
        Self {
            config: None,
            frontier: None,
            rate_limiter: None,
            store: None,
            event_emitter: None,
            strategy: None,
            content_filter: None,
            cache: None,
        }
    }

    /// Set the crawl configuration.
    pub fn config(mut self, config: CrawlConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the frontier implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn frontier(mut self, frontier: impl Frontier + 'static) -> Self {
        self.frontier = Some(Arc::new(frontier));
        self
    }

    /// Set the rate limiter implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn rate_limiter(mut self, rate_limiter: impl RateLimiter + 'static) -> Self {
        self.rate_limiter = Some(Arc::new(rate_limiter));
        self
    }

    /// Set the store implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn store(mut self, store: impl CrawlStore + 'static) -> Self {
        self.store = Some(Arc::new(store));
        self
    }

    /// Set the event emitter implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn event_emitter(mut self, event_emitter: impl EventEmitter + 'static) -> Self {
        self.event_emitter = Some(Arc::new(event_emitter));
        self
    }

    /// Set the crawl strategy implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn strategy(mut self, strategy: impl CrawlStrategy + 'static) -> Self {
        self.strategy = Some(Arc::new(strategy));
        self
    }

    /// Set the content filter implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn content_filter(mut self, content_filter: impl ContentFilter + 'static) -> Self {
        self.content_filter = Some(Arc::new(content_filter));
        self
    }

    /// Set the persistent cache implementation.
    #[allow(dead_code)]
    #[cfg_attr(alef, alef(skip))]
    pub fn cache(mut self, cache: impl CrawlCache + 'static) -> Self {
        self.cache = Some(Arc::new(cache));
        self
    }

    /// Build the [`CrawlEngine`] with the configured options.
    ///
    /// Config validation is deferred to the first operation (scrape, crawl, etc.) so that
    /// the engine can always be constructed and individual operations report validation errors.
    pub fn build(self) -> Result<CrawlEngine, CrawlError> {
        let config = self.config.unwrap_or_default();
        let rate_limit_ms = config.rate_limit_ms.unwrap_or(200);
        #[cfg(not(target_arch = "wasm32"))]
        let ua_rotation = crate::tower::UaRotationLayer::new(config.user_agents.clone());
        Ok(CrawlEngine {
            config,
            frontier: self
                .frontier
                .unwrap_or_else(|| Arc::new(defaults::InMemoryFrontier::new())),
            rate_limiter: self.rate_limiter.unwrap_or_else(|| {
                Arc::new(defaults::PerDomainThrottle::new(std::time::Duration::from_millis(
                    rate_limit_ms,
                )))
            }),
            store: self.store.unwrap_or_else(|| Arc::new(defaults::NoopStore)),
            event_emitter: self.event_emitter.unwrap_or_else(|| Arc::new(defaults::NoopEmitter)),
            strategy: self.strategy.unwrap_or_else(|| Arc::new(defaults::BfsStrategy)),
            content_filter: self.content_filter.unwrap_or_else(|| Arc::new(defaults::NoopFilter)),
            cache: self.cache.unwrap_or_else(|| Arc::new(defaults::NoopCache)),
            #[cfg(not(target_arch = "wasm32"))]
            ua_rotation,
        })
    }
}

impl Default for CrawlEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
