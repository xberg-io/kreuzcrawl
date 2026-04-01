//! In-memory frontier backed by a `VecDeque` and `AHashSet`.

use std::collections::VecDeque;
use std::sync::Mutex;

use ahash::AHashSet;
use async_trait::async_trait;

use crate::error::CrawlError;
use crate::traits::{Frontier, FrontierEntry};

/// A simple in-memory URL frontier with deduplication.
///
/// Uses BFS ordering (FIFO) by default.
#[derive(Debug)]
pub struct InMemoryFrontier {
    queue: Mutex<VecDeque<FrontierEntry>>,
    seen: Mutex<AHashSet<String>>,
}

impl InMemoryFrontier {
    /// Create a new empty `InMemoryFrontier`.
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            seen: Mutex::new(AHashSet::new()),
        }
    }
}

impl Default for InMemoryFrontier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Frontier for InMemoryFrontier {
    async fn push(&self, entry: FrontierEntry) -> Result<(), CrawlError> {
        self.queue.lock().unwrap().push_back(entry);
        Ok(())
    }

    async fn pop(&self) -> Result<Option<FrontierEntry>, CrawlError> {
        Ok(self.queue.lock().unwrap().pop_front())
    }

    async fn pop_batch(&self, n: usize) -> Result<Vec<FrontierEntry>, CrawlError> {
        let mut queue = self.queue.lock().unwrap();
        let mut batch = Vec::with_capacity(n.min(queue.len()));
        for _ in 0..n {
            match queue.pop_front() {
                Some(entry) => batch.push(entry),
                None => break,
            }
        }
        Ok(batch)
    }

    async fn is_seen(&self, url: &str) -> Result<bool, CrawlError> {
        Ok(self.seen.lock().unwrap().contains(url))
    }

    async fn mark_seen(&self, url: &str) -> Result<(), CrawlError> {
        self.seen.lock().unwrap().insert(url.to_owned());
        Ok(())
    }

    async fn len(&self) -> Result<usize, CrawlError> {
        Ok(self.queue.lock().unwrap().len())
    }
}
