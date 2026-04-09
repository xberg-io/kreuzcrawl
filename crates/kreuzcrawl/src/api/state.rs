//! Shared API state.

use std::sync::Arc;
use std::time::Duration;

use crate::engine::CrawlEngine;

use super::jobs::JobRegistry;

/// Default time-to-live for jobs (1 hour).
const DEFAULT_JOB_MAX_AGE: Duration = Duration::from_secs(3600);

/// Shared state for all API handlers.
#[derive(Clone)]
pub struct ApiState {
    /// The crawl engine used to execute scrape, crawl, and map operations.
    pub engine: Arc<CrawlEngine>,
    /// Registry of asynchronous crawl and batch scrape jobs.
    pub jobs: Arc<JobRegistry>,
}

impl ApiState {
    /// Create a new `ApiState` wrapping the given engine.
    ///
    /// This also spawns a background task that periodically evicts expired jobs
    /// from the registry (default TTL: 1 hour).
    pub fn new(engine: Arc<CrawlEngine>) -> Self {
        let jobs = Arc::new(JobRegistry::new());
        jobs.spawn_eviction_task(DEFAULT_JOB_MAX_AGE);
        Self { engine, jobs }
    }
}
