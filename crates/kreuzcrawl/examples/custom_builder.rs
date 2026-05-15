//! Smoke test that the public builder composes with a public strategy + a public default impl.
//! Compiled by `cargo build --example custom_builder -p kreuzcrawl` to catch visibility regressions
//! from an external-consumer vantage point.

use kreuzcrawl::{BfsStrategy, CrawlEngine, NoopStore};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _engine = CrawlEngine::builder().strategy(BfsStrategy).store(NoopStore).build()?;
    Ok(())
}
