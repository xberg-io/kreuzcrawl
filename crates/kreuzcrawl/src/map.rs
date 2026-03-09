//! Site mapping operation that discovers URLs via sitemaps and link extraction.

use std::collections::HashSet;

use regex::Regex;
use scraper::Html;
use url::Url;

use crate::error::CrawlError;
use crate::html::{extract_links, is_html_content};
use crate::http::{fetch_with_retry, http_fetch};
use crate::normalize::{normalize_url, resolve_redirect};
use crate::robots::parse_robots_txt;
use crate::sitemap::{decompress_gzip, fetch_sitemap_tree, is_sitemap_index, parse_sitemap_xml};
use crate::types::{CrawlConfig, MapResult, SitemapUrl};

/// Map a website to discover its URLs.
///
/// Tries the following strategies in order:
/// 1. Robots.txt sitemap directives (if `respect_robots_txt` is enabled)
/// 2. `/sitemap.xml` fallback
/// 3. Direct fetch of the URL (handles XML sitemaps, gzip, or HTML link extraction)
///
/// Applies `exclude_paths`, `map_search`, and `map_limit` filters to the result.
pub async fn map(url: &str, config: &CrawlConfig) -> Result<MapResult, CrawlError> {
    let parsed_url = Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;

    // Try robots.txt for sitemap directives
    if config.respect_robots_txt {
        let robots_url = format!(
            "{}://{}/robots.txt",
            parsed_url.scheme(),
            parsed_url.authority()
        );
        if let Ok(robots_resp) = http_fetch(&robots_url, config).await {
            let ua = config.user_agent.as_deref().unwrap_or("*");
            let rules = parse_robots_txt(&robots_resp.body, ua);
            if !rules.sitemaps.is_empty() {
                let mut all_urls = Vec::new();
                for sitemap_ref in &rules.sitemaps {
                    let sitemap_url = resolve_redirect(url, sitemap_ref);
                    // Resolve the path against the base URL host
                    let resolved = if let Ok(su) = Url::parse(&sitemap_url) {
                        if su.host_str() != parsed_url.host_str() {
                            let mut base = parsed_url.clone();
                            base.set_path(su.path());
                            base.set_query(su.query());
                            base.to_string()
                        } else {
                            sitemap_url.clone()
                        }
                    } else {
                        sitemap_url
                    };
                    all_urls.extend(fetch_sitemap_tree(&resolved, config).await);
                }
                if !all_urls.is_empty() {
                    return Ok(filter_map_result(all_urls, config));
                }
            }
        }
    }

    // Try /sitemap.xml as fallback
    let sitemap_url = format!(
        "{}://{}/sitemap.xml",
        parsed_url.scheme(),
        parsed_url.authority()
    );
    if let Ok(sitemap_resp) = http_fetch(&sitemap_url, config).await
        && (sitemap_resp.body.contains("<urlset") || sitemap_resp.body.contains("<sitemapindex"))
    {
        let urls = fetch_sitemap_tree(&sitemap_url, config).await;
        if !urls.is_empty() {
            return Ok(filter_map_result(urls, config));
        }
    }

    // Fetch the page directly and try to parse as sitemap or extract links
    let resp = fetch_with_retry(url, config).await?;

    let is_xml = resp.content_type.contains("xml") || resp.body.trim_start().starts_with("<?xml");

    // Check for gzip content (by header, URL extension, or magic bytes)
    let is_gzip = resp.content_type.contains("gzip")
        || resp.content_type.contains("x-gzip")
        || url.to_lowercase().ends_with(".gz")
        || (resp.body_bytes.len() >= 2 && resp.body_bytes[0] == 0x1f && resp.body_bytes[1] == 0x8b);
    if is_gzip && let Ok(decompressed) = decompress_gzip(&resp.body_bytes) {
        let urls = parse_sitemap_xml(&decompressed);
        if !urls.is_empty() {
            return Ok(filter_map_result(urls, config));
        }
    }

    if is_xml {
        if is_sitemap_index(&resp.body) {
            // It's a sitemap index -- delegate
            let urls = fetch_sitemap_tree(url, config).await;
            return Ok(filter_map_result(urls, config));
        }
        // Try as regular sitemap
        let urls = parse_sitemap_xml(&resp.body);
        if !urls.is_empty() {
            return Ok(filter_map_result(urls, config));
        }
    }

    // Fall back to link extraction from HTML
    if is_html_content(&resp.content_type, &resp.body) {
        let doc = Html::parse_document(&resp.body);
        let links = extract_links(&doc, &parsed_url);
        let mut url_set: Vec<SitemapUrl> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        for link in &links {
            if link.link_type == "internal" || link.link_type == "document" {
                let norm = normalize_url(&link.url);
                if seen.insert(norm.clone()) {
                    // Strip fragment
                    let clean = if let Ok(mut u) = Url::parse(&link.url) {
                        u.set_fragment(None);
                        u.to_string()
                    } else {
                        link.url.clone()
                    };
                    url_set.push(SitemapUrl {
                        url: clean,
                        lastmod: None,
                        changefreq: None,
                        priority: None,
                    });
                }
            } else if link.link_type == "external" {
                let norm = normalize_url(&link.url);
                if seen.insert(norm) {
                    url_set.push(SitemapUrl {
                        url: link.url.clone(),
                        lastmod: None,
                        changefreq: None,
                        priority: None,
                    });
                }
            }
        }
        return Ok(filter_map_result(url_set, config));
    }

    Ok(MapResult { urls: Vec::new() })
}

/// Apply exclude paths, search filter, and limit to the map result.
pub(crate) fn filter_map_result(mut urls: Vec<SitemapUrl>, config: &CrawlConfig) -> MapResult {
    // Apply exclude paths
    if let Some(ref excludes) = config.exclude_paths {
        urls.retain(|su| {
            if let Ok(u) = Url::parse(&su.url) {
                let path = u.path();
                !excludes
                    .iter()
                    .any(|pat| Regex::new(pat).map(|re| re.is_match(path)).unwrap_or(false))
            } else {
                true
            }
        });
    }

    // Apply search filter
    if let Some(ref search) = config.map_search {
        let lower = search.to_lowercase();
        urls.retain(|su| su.url.to_lowercase().contains(&lower));
    }

    // Apply limit
    if let Some(limit) = config.map_limit {
        urls.truncate(limit);
    }

    MapResult { urls }
}
