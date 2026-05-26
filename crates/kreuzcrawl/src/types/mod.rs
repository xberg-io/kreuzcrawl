//! Public types used across the kreuzcrawl crate.

mod bypass;
mod config;
mod discovery;
mod metadata;
mod results;
#[cfg(not(target_arch = "wasm32"))]
mod streaming;

pub use bypass::{BypassProvider, DynBypassProvider};
pub use config::{
    AuthConfig, BrowserBackend, BrowserConfig, BrowserMode, BrowserWait, ContentConfig, CrawlConfig, ExtractionMeta,
    ProxyConfig,
};
#[cfg(not(target_arch = "wasm32"))]
pub use discovery::CrawlEvent;
pub use discovery::{
    AssetCategory, CookieInfo, DownloadedAsset, FeedInfo, FeedType, ImageInfo, ImageSource, JsonLdEntry, LinkInfo,
    LinkType,
};
pub use metadata::{ArticleMetadata, FaviconInfo, HeadingInfo, HreflangEntry, PageMetadata, ResponseMeta};
pub use results::{
    ActionResult, BrowserExtras, CachedPage, CrawlPageResult, CrawlResult, DownloadedDocument, InteractionResult,
    MapResult, MarkdownResult, ScrapeResult, SitemapUrl,
};
#[cfg(not(target_arch = "wasm32"))]
pub use streaming::{BatchCrawlStreamRequest, CrawlStreamRequest};
