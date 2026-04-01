//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

mod assets;
mod batch;
#[cfg(feature = "browser")]
mod browser;
mod browser_detect;
#[cfg(feature = "browser")]
mod browser_pool;
pub mod defaults;
pub mod engine;
mod error;
mod helpers;
mod html;
mod http;
mod map;
mod normalize;
mod robots;
mod scrape;
mod sitemap;
pub mod traits;
mod types;

#[cfg(feature = "browser")]
pub use browser_pool::{BrowserPool, BrowserPoolConfig, PooledPage};
pub use defaults::{
    BestFirstStrategy, BfsStrategy, Bm25Filter, DfsStrategy, InMemoryFrontier, NoopEmitter,
    NoopFilter, NoopMiddleware, NoopRateLimiter, NoopStore, PerDomainThrottle, SystemResolver,
};
pub use engine::{CrawlEngine, CrawlEngineBuilder};
pub use error::CrawlError;
pub use traits::{
    CompleteEvent, ContentFilter, CrawlMiddleware, CrawlStats, CrawlStore, CrawlStrategy,
    DnsResolver, ErrorEvent, EventEmitter, Frontier, FrontierEntry, PageEvent, RateLimiter,
    RequestContext, ResponseContext,
};
pub use types::{
    ArticleMetadata, AssetCategory, AuthConfig, BrowserConfig, BrowserMode, BrowserWait,
    CookieInfo, CrawlConfig, CrawlEvent, CrawlPageResult, CrawlResult, DownloadedAsset,
    FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry, ImageInfo, ImageSource,
    JsonLdEntry, LinkInfo, LinkType, MapResult, PageMetadata, ResponseMeta, ScrapeResult,
    SitemapUrl,
};
