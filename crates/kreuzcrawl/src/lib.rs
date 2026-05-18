//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

#[cfg(feature = "api")]
pub(crate) mod api;
mod assets;
pub(crate) mod bindings;
#[cfg(feature = "browser")]
mod browser;
mod browser_detect;
#[cfg(feature = "browser")]
mod browser_pool;
#[cfg(feature = "browser")]
pub(crate) mod browser_profile;
pub(crate) mod citations;

pub(crate) mod defaults;
pub(crate) mod engine;
mod error;
#[cfg(not(target_arch = "wasm32"))]
mod helpers;
mod html;
mod http;
#[cfg(feature = "interact")]
pub(crate) mod interact;
mod map;
mod markdown;
#[cfg(feature = "mcp")]
pub(crate) mod mcp;
#[cfg(feature = "browser-native")]
mod native_browser;
mod normalize;
mod pruning;
#[cfg(feature = "ai")]
pub(crate) mod research;
mod robots;
mod scrape;
mod sitemap;
pub(crate) mod tower;
pub mod traits;
mod types;
#[cfg(feature = "warc")]
pub(crate) mod warc;

#[cfg(feature = "api")]
pub use api::serve_with_config as serve_api;
pub use bindings::{
    BatchCrawlResult, BatchScrapeResult, CrawlEngineHandle, batch_crawl, batch_scrape, crawl, create_engine, map_urls,
    scrape,
};
pub use citations::{CitationReference, CitationResult};
pub use defaults::{
    AdaptiveStrategy, BestFirstStrategy, BfsStrategy, DfsStrategy, InMemoryFrontier, NoopCache, NoopEmitter,
    NoopFilter, NoopStore, PerDomainThrottle,
};
pub use engine::{CrawlEngine, CrawlEngineBuilder};
pub use error::CrawlError;
#[cfg(feature = "mcp")]
pub use mcp::{start_mcp_server, start_mcp_server_with_config};
pub use types::{
    ArticleMetadata, AssetCategory, AuthConfig, BrowserBackend, BrowserConfig, BrowserExtras, BrowserMode, BrowserWait,
    CachedPage, ContentConfig, CookieInfo, CrawlConfig, CrawlPageResult, CrawlResult, DownloadedAsset,
    DownloadedDocument, ExtractionMeta, FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry, ImageInfo,
    ImageSource, JsonLdEntry, LinkInfo, LinkType, MapResult, MarkdownResult, PageMetadata, ProxyConfig, ResponseMeta,
    ScrapeResult, SitemapUrl,
};
