//! Public types used across the kreuzcrawl crate.

mod config;
mod discovery;
mod metadata;
mod results;

pub use config::{
    AuthConfig, BrowserConfig, BrowserMode, BrowserWait, CrawlConfig, ExtractionMeta, ProxyConfig,
};
pub use discovery::{
    AssetCategory, CookieInfo, CrawlEvent, DownloadedAsset, FeedInfo, FeedType, ImageInfo,
    ImageSource, JsonLdEntry, LinkInfo, LinkType,
};
pub use metadata::{
    ArticleMetadata, FaviconInfo, HeadingInfo, HreflangEntry, PageMetadata, ResponseMeta,
};
pub use results::{
    ActionResult, CachedPage, CrawlPageResult, CrawlResult, DownloadedDocument, InteractionResult,
    MapResult, MarkdownResult, ScrapeResult, SitemapUrl,
};
