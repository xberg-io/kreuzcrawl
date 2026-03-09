//! Sitemap XML parsing and recursive fetching.

use quick_xml::Reader;
use quick_xml::events::Event;
use url::Url;

use crate::http::http_fetch;
use crate::normalize::resolve_redirect;
use crate::types::{CrawlConfig, SitemapUrl};

/// Parse a sitemap XML document and extract URL entries.
pub(crate) fn parse_sitemap_xml(body: &str) -> Vec<SitemapUrl> {
    let mut urls = Vec::new();

    let mut reader = Reader::from_str(body);
    let mut buf = Vec::new();
    let mut in_url = false;
    let mut in_loc = false;
    let mut in_lastmod = false;
    let mut in_changefreq = false;
    let mut in_priority = false;
    let mut current_loc = String::new();
    let mut current_lastmod: Option<String> = None;
    let mut current_changefreq: Option<String> = None;
    let mut current_priority: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();
                match name.as_str() {
                    "url" => {
                        in_url = true;
                        current_loc.clear();
                        current_lastmod = None;
                        current_changefreq = None;
                        current_priority = None;
                    }
                    "loc" if in_url => in_loc = true,
                    "lastmod" if in_url => in_lastmod = true,
                    "changefreq" if in_url => in_changefreq = true,
                    "priority" if in_url => in_priority = true,
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();
                match name.as_str() {
                    "url" => {
                        if in_url && !current_loc.is_empty() {
                            urls.push(SitemapUrl {
                                url: current_loc.clone(),
                                lastmod: current_lastmod.clone(),
                                changefreq: current_changefreq.clone(),
                                priority: current_priority.clone(),
                            });
                        }
                        in_url = false;
                    }
                    "loc" => in_loc = false,
                    "lastmod" => in_lastmod = false,
                    "changefreq" => in_changefreq = false,
                    "priority" => in_priority = false,
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Ok(text) = e.xml_content() {
                    let text = text.trim().to_owned();
                    if in_loc {
                        current_loc = text;
                    } else if in_lastmod {
                        current_lastmod = Some(text);
                    } else if in_changefreq {
                        current_changefreq = Some(text);
                    } else if in_priority {
                        current_priority = Some(text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    urls
}

/// Parse a sitemap index XML document and return the child sitemap URLs.
pub(crate) fn parse_sitemap_index(body: &str) -> Vec<String> {
    let mut child_urls = Vec::new();
    let mut reader = Reader::from_str(body);
    let mut buf = Vec::new();
    let mut in_sitemap = false;
    let mut in_loc = false;
    let mut current_loc = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();
                match name.as_str() {
                    "sitemap" => {
                        in_sitemap = true;
                        current_loc.clear();
                    }
                    "loc" if in_sitemap => in_loc = true,
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();
                match name.as_str() {
                    "sitemap" => {
                        if in_sitemap && !current_loc.is_empty() {
                            child_urls.push(current_loc.clone());
                        }
                        in_sitemap = false;
                    }
                    "loc" => in_loc = false,
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_loc && let Ok(text) = e.xml_content() {
                    current_loc = text.trim().to_owned();
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    child_urls
}

/// Check whether the body looks like a sitemap index (contains `<sitemapindex`).
pub(crate) fn is_sitemap_index(body: &str) -> bool {
    body.contains("<sitemapindex") || body.contains("<sitemapindex>")
}

/// Recursively fetch a sitemap tree, following sitemap index references.
///
/// If the URL points to a sitemap index, fetches each child sitemap and
/// collects all URL entries. Handles gzip-compressed sitemaps.
pub(crate) async fn fetch_sitemap_tree(sitemap_url: &str, config: &CrawlConfig) -> Vec<SitemapUrl> {
    let resp = match http_fetch(sitemap_url, config).await {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };

    let body = &resp.body;

    // Handle gzip
    let decompressed;
    let xml_body = if resp.content_type.contains("gzip") || resp.content_type.contains("x-gzip") {
        match decompress_gzip(&resp.body_bytes) {
            Ok(d) => {
                decompressed = d;
                &decompressed
            }
            Err(_) => body,
        }
    } else {
        body
    };

    if is_sitemap_index(xml_body) {
        let child_urls = parse_sitemap_index(xml_body);
        let base = Url::parse(sitemap_url).ok();
        let mut all_urls = Vec::new();
        for child_url in &child_urls {
            // Resolve child URL path against the base URL's host
            let resolved = if let Some(ref base_parsed) = base {
                if let Ok(child_parsed) = Url::parse(child_url) {
                    // If child URL is on a different host, rewrite to use base host
                    if child_parsed.host_str() != base_parsed.host_str() {
                        let mut resolved_url = base_parsed.clone();
                        resolved_url.set_path(child_parsed.path());
                        resolved_url.set_query(child_parsed.query());
                        resolved_url.to_string()
                    } else {
                        child_url.clone()
                    }
                } else {
                    resolve_redirect(sitemap_url, child_url)
                }
            } else {
                child_url.clone()
            };
            let child_resp = match http_fetch(&resolved, config).await {
                Ok(r) => r,
                Err(_) => continue,
            };
            let child_body = &child_resp.body;
            let child_decompressed;
            let child_xml = if child_resp.content_type.contains("gzip") {
                match decompress_gzip(&child_resp.body_bytes) {
                    Ok(d) => {
                        child_decompressed = d;
                        &child_decompressed
                    }
                    Err(_) => child_body,
                }
            } else {
                child_body
            };
            all_urls.extend(parse_sitemap_xml(child_xml));
        }
        all_urls
    } else {
        parse_sitemap_xml(xml_body)
    }
}

/// Decompress gzip-encoded data into a UTF-8 string.
pub(crate) fn decompress_gzip(data: &[u8]) -> Result<String, std::io::Error> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(data);
    let mut result = String::new();
    decoder.read_to_string(&mut result)?;
    Ok(result)
}
