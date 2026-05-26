//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

#[cfg(feature = "api")]
pub(crate) mod api;
mod assets;
pub(crate) mod bindings;
#[cfg(feature = "browser")]
mod browser;
mod browser_detect;
#[cfg(feature = "browser")]
pub mod browser_pool;
#[cfg(feature = "browser")]
pub(crate) mod browser_profile;
#[cfg(feature = "browser")]
pub mod browser_session_pool;
pub(crate) mod citations;
#[cfg(feature = "browser")]
mod stealth;

pub(crate) mod defaults;
mod document;
pub(crate) mod engine;
mod error;
#[cfg(not(target_arch = "wasm32"))]
mod helpers;
mod html;
mod http;
pub mod interact;
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
    BatchCrawlResult, BatchCrawlResults, BatchScrapeResult, BatchScrapeResults, CrawlEngineHandle, batch_crawl,
    batch_scrape, crawl, create_engine, interact, map_urls, scrape,
};
#[cfg(not(target_arch = "wasm32"))]
pub use bindings::{batch_crawl_stream, crawl_stream};
#[cfg(feature = "browser")]
pub use browser_pool::{BrowserPool, BrowserPoolConfig};
#[cfg(feature = "browser")]
pub use browser_session_pool::{BrowserSessionPool, SessionKey};
pub use citations::{CitationReference, CitationResult, generate_citations};
pub use defaults::{
    AdaptiveStrategy, BestFirstStrategy, BfsStrategy, DfsStrategy, InMemoryFrontier, NoopCache, NoopEmitter,
    NoopFilter, NoopStore, PerDomainThrottle,
};
pub use engine::{CrawlEngine, CrawlEngineBuilder};
pub use error::CrawlError;
pub use http::HttpResponse;
pub use interact::{
    MAX_ACTIONS, MAX_SCRIPT_LEN, MAX_SCROLL_AMOUNT, MAX_SELECTOR_LEN, MAX_SINGLE_WAIT_MS, MAX_TEXT_LEN,
    MAX_TOTAL_WAIT_SECS, PageAction, ScrollDirection, validate_actions,
};
#[cfg(feature = "browser-native")]
pub use kreuzcrawl_browser::adapter::{NativeBrowserExecutor, NativeBrowserExecutorConfig};
#[cfg(feature = "mcp")]
pub use mcp::{start_mcp_server, start_mcp_server_with_config};
pub use types::{
    ActionResult, ArticleMetadata, AssetCategory, AuthConfig, BrowserBackend, BrowserConfig, BrowserExtras,
    BrowserMode, BrowserWait, BypassProvider, CachedPage, ContentConfig, CookieInfo, CrawlConfig, CrawlPageResult,
    CrawlResult, DownloadedAsset, DownloadedDocument, DynBypassProvider, ExtractionMeta, FaviconInfo, FeedInfo,
    FeedType, HeadingInfo, HreflangEntry, ImageInfo, ImageSource, InteractionResult, JsonLdEntry, LinkInfo, LinkType,
    MapResult, MarkdownResult, PageMetadata, ProxyConfig, ResponseMeta, ScrapeResult, SitemapUrl,
};
#[cfg(not(target_arch = "wasm32"))]
pub use types::{BatchCrawlStreamRequest, CrawlEvent, CrawlStreamRequest};
