//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

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
    CookieInfo, CrawlConfig, CrawlPageResult, CrawlResult, FeedInfo, FeedType, ImageInfo,
    JsonLdEntry, LinkInfo, MapResult, PageMetadata, ScrapeResult, SitemapUrl,
};
