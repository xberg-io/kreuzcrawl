//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

mod assets;
mod crawl;
mod error;
mod html;
mod http;
mod map;
mod normalize;
mod robots;
mod scrape;
mod sitemap;
mod types;

pub use crawl::crawl;
pub use error::CrawlError;
pub use map::map;
pub use scrape::scrape;
pub use types::{
    ArticleMetadata, AssetCategory, CookieInfo, CrawlConfig, CrawlPageResult, CrawlResult,
    DownloadedAsset, FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry, ImageInfo,
    ImageSource, JsonLdEntry, LinkInfo, LinkType, MapResult, PageMetadata, ResponseMeta,
    ScrapeResult, SitemapUrl,
};
