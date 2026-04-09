//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

mod assets;
#[cfg(feature = "browser")]
mod browser;
mod browser_detect;
#[cfg(feature = "browser")]
mod browser_pool;
mod citations;

pub mod defaults;
pub mod engine;
mod error;
mod helpers;
mod html;
mod http;
mod map;
mod markdown;
mod normalize;
mod pruning;
mod robots;
mod scrape;
mod sitemap;
pub mod tower;
pub mod traits;
mod types;

#[cfg(feature = "browser")]
pub use browser_pool::{BrowserPool, BrowserPoolConfig, PooledPage};
pub use citations::{CitationReference, CitationResult};
#[cfg(feature = "ai")]
pub use defaults::LlmExtractor;
pub use defaults::{
    AdaptiveStrategy, BestFirstStrategy, BfsStrategy, Bm25Filter, DfsStrategy, DiskCache,
    InMemoryFrontier, NoopCache, NoopEmitter, NoopFilter, NoopRateLimiter, NoopStore,
    PerDomainThrottle,
};
pub use engine::{CrawlEngine, CrawlEngineBuilder};
pub use error::CrawlError;
pub use traits::{
    CompleteEvent, ContentFilter, CrawlCache, CrawlStats, CrawlStore, CrawlStrategy, ErrorEvent,
    EventEmitter, Frontier, FrontierEntry, PageEvent, RateLimiter,
};
pub use types::{
    ArticleMetadata, AssetCategory, AuthConfig, BrowserConfig, BrowserMode, BrowserWait,
    CachedPage, CookieInfo, CrawlConfig, CrawlEvent, CrawlPageResult, CrawlResult, DownloadedAsset,
    ExtractionMeta, FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry, ImageInfo,
    ImageSource, JsonLdEntry, LinkInfo, LinkType, MapResult, MarkdownResult, PageMetadata,
    ProxyConfig, ResponseMeta, ScrapeResult, SitemapUrl,
};
