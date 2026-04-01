//! Crawl strategies: BFS, DFS, and best-first URL selection.

use crate::traits::{CrawlStats, CrawlStrategy, FrontierEntry};

/// A breadth-first crawl strategy.
///
/// Always selects the first (oldest) entry from the frontier, giving BFS behavior.
#[derive(Debug, Clone, Default, Copy)]
pub struct BfsStrategy;

impl CrawlStrategy for BfsStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() { None } else { Some(0) }
    }
}

/// A depth-first crawl strategy.
///
/// Always selects the last (newest) entry from the working set, giving LIFO / DFS behavior.
#[derive(Debug, Clone, Default, Copy)]
pub struct DfsStrategy;

impl CrawlStrategy for DfsStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() {
            None
        } else {
            Some(candidates.len() - 1)
        }
    }
}

/// A best-first crawl strategy.
///
/// Selects the candidate with the highest `priority` value. Consumers can override
/// [`CrawlStrategy::score_url`] to provide custom scoring; the default scores by
/// inverse depth: `1.0 / (depth + 1.0)`.
#[derive(Debug, Clone, Default, Copy)]
pub struct BestFirstStrategy;

impl CrawlStrategy for BestFirstStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() {
            return None;
        }
        let mut best_idx = 0;
        let mut best_priority = candidates[0].priority;
        for (i, entry) in candidates.iter().enumerate().skip(1) {
            if entry.priority > best_priority {
                best_priority = entry.priority;
                best_idx = i;
            }
        }
        Some(best_idx)
    }

    fn score_url(&self, _url: &str, depth: usize) -> f64 {
        1.0 / (depth as f64 + 1.0)
    }

    fn should_continue(&self, _stats: &CrawlStats) -> bool {
        true
    }
}
