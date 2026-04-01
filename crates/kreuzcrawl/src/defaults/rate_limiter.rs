//! Rate limiter implementations.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use ahash::AHashMap;
use async_trait::async_trait;

use crate::error::CrawlError;
use crate::traits::RateLimiter;

/// Maximum backoff duration for 429 responses.
const MAX_BACKOFF: Duration = Duration::from_secs(60);

/// A rate limiter that does nothing, allowing all requests through immediately.
#[derive(Debug, Clone, Default)]
pub struct NoopRateLimiter;

#[async_trait]
impl RateLimiter for NoopRateLimiter {
    async fn acquire(&self, _domain: &str) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn record_response(&self, _domain: &str, _status: u16) -> Result<(), CrawlError> {
        Ok(())
    }

    async fn set_crawl_delay(&self, _domain: &str, _delay: Duration) -> Result<(), CrawlError> {
        Ok(())
    }
}

/// Per-domain state tracked by [`PerDomainThrottle`].
#[derive(Debug, Clone)]
struct DomainState {
    last_request: Instant,
    crawl_delay: Option<Duration>,
}

/// A per-domain token bucket rate limiter.
///
/// Enforces a configurable delay between requests to the same domain.
/// Respects robots.txt crawl-delay via `set_crawl_delay`.
#[derive(Debug)]
pub struct PerDomainThrottle {
    default_delay: Duration,
    /// Per-domain state: last request time and optional crawl-delay override.
    state: Mutex<AHashMap<String, DomainState>>,
}

impl PerDomainThrottle {
    /// Create a new limiter with the given default delay between requests.
    pub fn new(default_delay: Duration) -> Self {
        Self {
            default_delay,
            state: Mutex::new(AHashMap::new()),
        }
    }
}

#[async_trait]
impl RateLimiter for PerDomainThrottle {
    async fn acquire(&self, domain: &str) -> Result<(), CrawlError> {
        let sleep_duration = {
            let mut state = self.state.lock().unwrap();
            let now = Instant::now();
            let domain_state = state.entry(domain.to_owned()).or_insert(DomainState {
                last_request: now - self.default_delay,
                crawl_delay: None,
            });

            let effective_delay = domain_state.crawl_delay.unwrap_or(self.default_delay);
            let elapsed = now.duration_since(domain_state.last_request);

            if elapsed < effective_delay {
                let duration = effective_delay - elapsed;
                // Set last_request optimistically BEFORE sleeping.
                // This prevents other tasks from seeing stale state.
                domain_state.last_request = now + duration;
                Some(duration)
            } else {
                domain_state.last_request = now;
                None
            }
        };

        if let Some(duration) = sleep_duration {
            tokio::time::sleep(duration).await;
            // No need to re-lock — the optimistic update already claimed this time slot
        }

        Ok(())
    }

    async fn record_response(&self, domain: &str, status: u16) -> Result<(), CrawlError> {
        if status == 429 {
            let mut state = self.state.lock().unwrap();
            if let Some(domain_state) = state.get_mut(domain) {
                let current = domain_state.crawl_delay.unwrap_or(self.default_delay);
                let new_delay = (current * 2).min(MAX_BACKOFF);
                domain_state.crawl_delay = Some(new_delay);
            }
        }
        Ok(())
    }

    async fn set_crawl_delay(&self, domain: &str, delay: Duration) -> Result<(), CrawlError> {
        let mut state = self.state.lock().unwrap();
        let domain_state = state.entry(domain.to_owned()).or_insert(DomainState {
            last_request: Instant::now() - self.default_delay,
            crawl_delay: None,
        });
        domain_state.crawl_delay = Some(delay);
        Ok(())
    }
}
