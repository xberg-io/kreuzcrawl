//! Feed, favicon, hreflang, and heading extraction from HTML documents.

use scraper::Html;

use crate::types::{FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry};

use super::selectors::{SEL_FAVICON, SEL_FEED_ALTERNATE, SEL_HEADINGS, SEL_HREFLANG};

/// Extract feed links (RSS, Atom, JSON Feed) from a parsed HTML document.
pub(crate) fn extract_feeds(doc: &Html) -> Vec<FeedInfo> {
    let mut feeds = Vec::new();

    for el in doc.select(&SEL_FEED_ALTERNATE) {
        let link_type = el.value().attr("type").unwrap_or("");
        let href = el.value().attr("href").unwrap_or("").to_owned();
        let title = el.value().attr("title").map(String::from);

        let feed_type = match link_type {
            "application/rss+xml" => Some(FeedType::Rss),
            "application/atom+xml" => Some(FeedType::Atom),
            "application/json" | "application/feed+json" => Some(FeedType::JsonFeed),
            _ => None,
        };

        if let Some(ft) = feed_type {
            feeds.push(FeedInfo {
                url: href,
                title,
                feed_type: ft,
            });
        }
    }
    feeds
}

/// Extract hreflang alternate links from a parsed HTML document.
pub(crate) fn extract_hreflangs(doc: &Html) -> Vec<HreflangEntry> {
    let mut entries = Vec::new();
    for el in doc.select(&SEL_HREFLANG) {
        let lang = el.value().attr("hreflang").unwrap_or("").to_owned();
        let url = el.value().attr("href").unwrap_or("").to_owned();
        if !lang.is_empty() && !url.is_empty() {
            entries.push(HreflangEntry { lang, url });
        }
    }
    entries
}

/// Extract favicon and icon links from a parsed HTML document.
pub(crate) fn extract_favicons(doc: &Html) -> Vec<FaviconInfo> {
    let mut favicons = Vec::new();
    for el in doc.select(&SEL_FAVICON) {
        let url = el.value().attr("href").unwrap_or("").to_owned();
        if url.is_empty() {
            continue;
        }
        let rel = el.value().attr("rel").unwrap_or("").to_owned();
        let sizes = el.value().attr("sizes").map(String::from);
        let mime_type = el.value().attr("type").map(String::from);
        favicons.push(FaviconInfo {
            url,
            rel,
            sizes,
            mime_type,
        });
    }
    favicons
}

/// Extract heading elements (h1-h6) from a parsed HTML document.
pub(crate) fn extract_headings(doc: &Html) -> Vec<HeadingInfo> {
    let mut headings = Vec::new();
    for el in doc.select(&SEL_HEADINGS) {
        let tag_name = el.value().name();
        let level = match tag_name {
            "h1" => 1,
            "h2" => 2,
            "h3" => 3,
            "h4" => 4,
            "h5" => 5,
            "h6" => 6,
            _ => continue,
        };
        let text = el.text().collect::<String>().trim().to_owned();
        headings.push(HeadingInfo { level, text });
    }
    headings
}
