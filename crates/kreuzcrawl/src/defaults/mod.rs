//! Default trait implementations for kreuzcrawl.

mod cache;
mod emitter;
mod filter;
mod frontier;
mod llm_extractor;
mod rate_limiter;
mod store;
mod strategy;

pub use cache::NoopCache;
pub use emitter::NoopEmitter;
pub use filter::NoopFilter;
pub use frontier::InMemoryFrontier;
#[cfg(test)]
pub use rate_limiter::NoopRateLimiter;
pub use rate_limiter::PerDomainThrottle;
pub use store::NoopStore;
pub use strategy::{AdaptiveStrategy, BestFirstStrategy, BfsStrategy, DfsStrategy};
