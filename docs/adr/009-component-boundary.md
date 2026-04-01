# ADR-009: Component Boundary — Trait-Based Extension Architecture

**Status**: Accepted

**Date**: 2026-03-31

## Context

kreuzcrawl is a crawling and extraction engine within the kreuzberg.dev ecosystem. It serves two roles simultaneously:

1. **A standalone open-source library** — released with polyglot bindings, useful for developers building crawl pipelines
2. **The crawling foundation for kreuzberg-cloud** — the commercial platform extends it with production infrastructure

This dual role creates a boundary problem: the engine must be powerful enough standalone, yet extensible enough for kreuzberg-cloud to inject distributed infrastructure, proxy management, AI-powered features, and multi-tenant isolation without modifying the engine itself.

Kreuzberg Cloud already uses kreuzberg (the document extraction library) as a pure dependency via the same pattern (kreuzberg-cloud ADR-0004). We apply the identical approach here.

### Design Principles

1. **Trait-based extension points**: Every infrastructure concern is a Rust trait with a robust default implementation. The defaults must work well for single-worker use — not stubs.
2. **The open-source version must stand on its own**: A developer can `cargo add kreuzcrawl` and have a fully functional crawler with rate limiting, multiple strategies, content filtering, and optionally AI-powered extraction.
3. **No commercial logic in the engine**: If a feature requires multi-tenancy, billing, distributed coordination, or proprietary infrastructure, it belongs outside kreuzcrawl.
4. **Library, not a service**: kreuzcrawl has no HTTP server, no CLI, no config files. Consumers wrap it in whatever service layer they need.

## Decision

### Eight Extension Traits

kreuzcrawl defines eight traits that compose via a `CrawlEngine` builder. Each has a default implementation that ships with the crate.

#### 1. `Frontier` — URL queue + deduplication

```rust
#[async_trait]
pub trait Frontier: Send + Sync {
    async fn push(&self, entry: FrontierEntry) -> Result<(), CrawlError>;
    async fn pop(&self) -> Result<Option<FrontierEntry>, CrawlError>;
    async fn pop_batch(&self, n: usize) -> Result<Vec<FrontierEntry>, CrawlError>;
    async fn len(&self) -> Result<usize, CrawlError>;
    async fn is_seen(&self, url: &str) -> Result<bool, CrawlError>;
    async fn mark_seen(&self, url: &str) -> Result<(), CrawlError>;
}
```

**Default**: `InMemoryFrontier` — `VecDeque` + `HashSet`. Handles site-scale crawls (< 100K URLs). Current engine behavior extracted into the trait.

#### 2. `RateLimiter` — per-domain throttling

```rust
#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn acquire(&self, domain: &str) -> Result<(), CrawlError>;
    async fn record_response(&self, domain: &str, status: u16);
    async fn set_crawl_delay(&self, domain: &str, delay: Duration);
}
```

**Default**: `TokenBucketLimiter` — per-domain token bucket with configurable delay (default 200ms). Respects robots.txt `crawl-delay`. Backs off on 429/503.

#### 3. `CrawlStore` — result persistence

```rust
#[async_trait]
pub trait CrawlStore: Send + Sync {
    async fn store_page(&self, url: &str, result: &ScrapeResult) -> Result<(), CrawlError>;
    async fn store_error(&self, url: &str, error: &CrawlError) -> Result<(), CrawlError>;
    async fn on_complete(&self, stats: &CrawlStats) -> Result<(), CrawlError>;
}
```

**Default**: `NoopStore` — results returned in-memory via `CrawlResult` or the streaming API. Nothing persisted to disk.

#### 4. `CrawlMiddleware` — request/response interceptors

```rust
#[async_trait]
pub trait CrawlMiddleware: Send + Sync {
    async fn before_request(&self, ctx: &mut RequestContext) -> Result<(), CrawlError>;
    async fn after_response(&self, ctx: &mut ResponseContext) -> Result<(), CrawlError>;
}
```

**Default**: `RetryMiddleware` — retry with exponential backoff and jitter. Refactored from the existing `fetch_with_retry` logic.

#### 5. `EventEmitter` — crawl lifecycle events

```rust
#[async_trait]
pub trait EventEmitter: Send + Sync {
    async fn on_page(&self, event: &PageEvent);
    async fn on_error(&self, event: &ErrorEvent);
    async fn on_complete(&self, event: &CompleteEvent);
    async fn on_discovered(&self, url: &str, depth: usize);
}
```

**Default**: `ChannelEmitter` — sends events to `tokio::sync::mpsc`, powering the existing `crawl_stream()` API.

#### 6. `DnsResolver` — DNS resolution

```rust
#[async_trait]
pub trait DnsResolver: Send + Sync {
    async fn resolve(&self, host: &str) -> Result<Vec<IpAddr>, CrawlError>;
}
```

**Default**: `SystemResolver` — delegates to the OS resolver via reqwest.

#### 7. `CrawlStrategy` — crawl ordering and stopping

```rust
pub trait CrawlStrategy: Send + Sync {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize>;
    fn score_url(&self, url: &str, depth: usize, parent: Option<&ScrapeResult>) -> f64 {
        1.0 / (depth as f64 + 1.0)
    }
    fn should_continue(&self, stats: &CrawlStats) -> bool {
        true
    }
}
```

**Default implementations** (all shipped with the crate):
- `BfsStrategy` — breadth-first (FIFO). Current behavior, the default.
- `DfsStrategy` — depth-first (LIFO).
- `BestFirstStrategy` — priority queue ordered by `score_url`. Enables focused crawling.

#### 8. `ContentFilter` — post-extraction filtering

```rust
#[async_trait]
pub trait ContentFilter: Send + Sync {
    async fn filter(&self, page: &CrawlPageResult) -> Result<Option<CrawlPageResult>, CrawlError>;
}
```

**Default implementations** (all shipped with the crate):
- `NoopFilter` — pass everything through.
- `BM25Filter` — keyword relevance scoring against a query string. Pure Rust, no external dependencies.

### CrawlEngine Builder

All traits compose via a builder. Unset traits use their defaults.

```rust
let engine = CrawlEngine::builder()
    .config(config)
    .frontier(Arc::new(InMemoryFrontier::new()))
    .rate_limiter(Arc::new(TokenBucketLimiter::default()))
    .strategy(Arc::new(BfsStrategy))
    .content_filter(Arc::new(NoopFilter))
    .store(Arc::new(NoopStore))
    .middleware(Arc::new(RetryMiddleware::default()))
    .event_emitter(Arc::new(ChannelEmitter::new(tx)))
    .dns_resolver(Arc::new(SystemResolver))
    .build();

let result = engine.crawl("https://example.com").await?;
```

The existing top-level functions (`scrape()`, `crawl()`, `crawl_stream()`, `batch_scrape()`, `map()`) continue to work as convenience wrappers that construct a `CrawlEngine` with all defaults internally. **Zero breaking changes.**

### What Is NOT a Trait

These remain concrete implementations — they are core differentiators, not extension points:

- **HTML extraction pipeline** — 40+ field extraction (metadata, links, images, feeds, JSON-LD)
- **robots.txt parser** — RFC 9309 compliance
- **URL normalization** — deterministic algorithm
- **Sitemap parser** — standard XML/gzip format
- **Error classification** — WAF, DNS, SSL, timeout detection

### Feature Flags

```toml
[features]
default = []
browser = ["dep:chromiumoxide"]       # Headless Chrome fallback + BrowserPool
markdown = ["dep:html-to-markdown"]   # HTML-to-markdown conversion
ai = ["dep:liter-llm"]               # LLM extraction, content filtering, adaptive crawling
extraction = ["dep:kreuzberg"]        # Non-HTML document extraction (PDF, DOCX, etc.)
full = ["browser", "markdown", "ai", "extraction"]
```

### Feature Placement

| In kreuzcrawl (open source) | Rationale |
|------------------------------|-----------|
| BFS / DFS / Best-First strategies | Core engine algorithms |
| Per-domain rate limiting with backoff | Polite crawling — every crawler needs this |
| HTML-to-markdown (feature-gated) | Critical for AI/LLM pipelines |
| BM25 content filtering | Pure Rust, standard IR algorithm |
| LLM extraction + content filter (feature-gated) | Via liter-llm; makes OSS competitive |
| Adaptive crawling (statistical signals) | Coverage/saturation scoring without LLM |
| HTTP conditional cache (ETag / Last-Modified) | Standard HTTP behavior |
| Anti-bot detection (signal) | WAF/blocking detection already exists |
| Basic proxy support (single proxy config) | Passthrough, not rotation |
| Browser fallback + BrowserPool (feature-gated) | SPA support |
| Document extraction (feature-gated) | Via kreuzberg integration |
| URL seeding via sitemaps + links | Part of existing `map()` API |
| Concurrent page fetching | Bounded by semaphore, pulls from frontier |
| All 8 trait definitions | The extension API surface |

## Consequences

### Positive

- **Fully functional standalone**: Every trait has a working default. No "upgrade to unlock" experience.
- **Compile-time boundary enforcement**: Traits make the separation architectural, not just policy.
- **Ecosystem-friendly**: Third parties can implement traits for their own infrastructure (SQS frontier, Kafka emitter, etc.)
- **Consistent with kreuzberg pattern**: Same "pure library + commercial layers" model.
- **Independently testable**: Each trait implementation is unit testable. Engine tests use defaults.

### Negative

- **Trait design is load-bearing**: Getting signatures wrong means breaking changes. Must stabilize before 1.0.
- **Dynamic dispatch cost**: `dyn Trait` adds indirection. Negligible for I/O-bound crawling but noted.
- **Feature pressure**: Temptation to move commercial features into the OSS engine for convenience.

### Neutral

- **Version coordination**: Consumers must track kreuzcrawl releases and test compatibility.
- **Trait evolution**: New traits and new methods with defaults are additive (minor version bumps).

## Alternatives Considered

### 1. No trait system — consumers wrap kreuzcrawl opaquely

**Rejected**: Would require reimplementing crawl orchestration to inject infrastructure concerns. Duplicates logic.

### 2. Plugin system with dynamic loading (dylib)

**Rejected**: Rust's `dylib` ABI instability makes this fragile. Compile-time traits are idiomatic and zero-cost.

### 3. Configuration-only extension (no traits, just config flags)

**Rejected**: A Redis frontier or NATS event emitter can't be expressed as a config flag. Traits are necessary.

### 4. Separate open-source and commercial engines

**Rejected**: Duplicates the core. Maintenance nightmare. Violates DRY.

## Implementation Notes

### Source Layout

```
crates/kreuzcrawl/src/
  traits.rs                      # All 8 trait definitions + supporting types
  engine.rs                      # CrawlEngine struct + builder
  defaults/
    mod.rs                       # Re-exports
    frontier.rs                  # InMemoryFrontier
    rate_limiter.rs              # TokenBucketLimiter
    store.rs                     # NoopStore
    middleware.rs                 # RetryMiddleware
    emitter.rs                   # ChannelEmitter
    resolver.rs                  # SystemResolver
    strategy.rs                  # BfsStrategy, DfsStrategy, BestFirstStrategy
    filter.rs                    # NoopFilter, BM25Filter
```

### Migration Path

1. Define traits and defaults that replicate current behavior exactly
2. Refactor `crawl_with_sender()` to use trait calls instead of hardcoded data structures
3. Add concurrent page fetching (currently serial)
4. Existing public API unchanged throughout — backward compatible

### Trait Stability

Traits are **unstable until kreuzcrawl 1.0**. After 1.0:
- New methods with default implementations → minor version
- Changed method signatures → major version
- New traits → minor version
