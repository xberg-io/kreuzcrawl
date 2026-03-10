//! Multi-page crawl operation with BFS link traversal.

use std::collections::{HashSet, VecDeque};

use regex::Regex;
use scraper::Html;
use url::Url;

use crate::error::CrawlError;
use crate::html::{
    detect_charset, detect_meta_refresh, extract_feeds, extract_images, extract_json_ld,
    extract_links, extract_metadata, is_binary_content_type, is_binary_url, is_html_content,
    is_pdf_content, is_pdf_url,
};
use crate::http::{build_client, extract_cookies, http_fetch};
use crate::normalize::{normalize_url, normalize_url_for_dedup, resolve_redirect, strip_fragment};
use crate::robots::{RobotsRules, is_path_allowed, parse_robots_txt};
use crate::types::{
    CookieInfo, CrawlConfig, CrawlEvent, CrawlPageResult, CrawlResult, LinkType, PageMetadata,
};

/// Crawl a website starting from the given URL using breadth-first traversal.
///
/// Follows links up to `max_depth` hops, respecting `max_pages`, domain restrictions,
/// include/exclude path patterns, and redirect handling. Returns all crawled pages
/// with their metadata, links, images, feeds, and JSON-LD data.
pub async fn crawl(url: &str, config: &CrawlConfig) -> Result<CrawlResult, CrawlError> {
    crawl_with_sender(url, config, None).await
}

/// Internal BFS crawl that optionally streams `CrawlEvent::Page` events via `tx`.
///
/// When `tx` is `None` the function behaves identically to the public [`crawl`].
/// When `tx` is `Some`, each page is sent through the channel as it is processed
/// so that callers can consume results incrementally.
pub(crate) async fn crawl_with_sender(
    url: &str,
    config: &CrawlConfig,
    tx: Option<tokio::sync::mpsc::Sender<CrawlEvent>>,
) -> Result<CrawlResult, CrawlError> {
    let parsed_url = Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
    let client = build_client(config)?;
    let base_host = parsed_url.host_str().unwrap_or("").to_owned();

    let max_depth = config.max_depth.unwrap_or(0);
    let max_pages = config.max_pages.unwrap_or(usize::MAX);
    let max_redirects = config.max_redirects.unwrap_or(10);

    let mut pages: Vec<CrawlPageResult> = Vec::new();
    let mut normalized_urls: Vec<String> = Vec::new();
    let mut visited_dedup: HashSet<String> = HashSet::new(); // uses dedup-normalized URLs
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut redirect_count: usize = 0;
    let mut error: Option<String> = None;
    let mut was_skipped = false;
    let mut all_cookies: Vec<CookieInfo> = Vec::new();

    // Handle redirects for the initial URL
    let mut current_url = url.to_owned();
    let mut seen_redirects: HashSet<String> = HashSet::new();
    seen_redirects.insert(current_url.clone());

    loop {
        let resp = match http_fetch(&current_url, config, &client).await {
            Ok(r) => r,
            Err(e) => {
                // Treat fetch errors as success-with-error for crawl
                error = Some(format!("{e}"));
                break;
            }
        };

        if config.cookies_enabled {
            all_cookies.extend(extract_cookies(&resp.headers));
        }

        let status = resp.status;
        let is_redirect = matches!(status, 301 | 302 | 303 | 307 | 308);

        if is_redirect {
            if redirect_count >= max_redirects {
                error = Some("too many redirects".to_owned());
                break;
            }
            if let Some(location) = resp.headers.get("location").and_then(|v| v.to_str().ok()) {
                let target = resolve_redirect(&current_url, location);
                if seen_redirects.contains(&target) {
                    error = Some("redirect loop detected".to_owned());
                    break;
                }
                seen_redirects.insert(target.clone());
                redirect_count += 1;
                current_url = target;
                continue;
            }
        }

        // Check for Refresh header redirect (B7: case-insensitive ASCII search on original, B9: max_redirects guard)
        if let Some(refresh) = resp.headers.get("refresh").and_then(|v| v.to_str().ok())
            && let Some(pos) = find_ascii_case_insensitive(refresh, "url=")
        {
            if redirect_count >= max_redirects {
                error = Some("too many redirects".to_owned());
                break;
            }
            let target_path = refresh[pos + 4..].trim();
            let target = resolve_redirect(&current_url, target_path);
            if !seen_redirects.contains(&target) {
                seen_redirects.insert(target.clone());
                redirect_count += 1;
                current_url = target;
                continue;
            }
        }

        // Check for meta refresh (B9: max_redirects guard)
        if is_html_content(&resp.content_type, &resp.body) {
            let doc = Html::parse_document(&resp.body);
            if let Some(refresh_target) = detect_meta_refresh(&doc) {
                if redirect_count >= max_redirects {
                    error = Some("too many redirects".to_owned());
                    break;
                }
                let target = resolve_redirect(&current_url, &refresh_target);
                if !seen_redirects.contains(&target) {
                    seen_redirects.insert(target.clone());
                    redirect_count += 1;
                    current_url = target;
                    continue;
                }
            }
        }

        // Check for error status on final page (after redirect)
        if status >= 400 && redirect_count > 0 {
            error = Some(format!("HTTP {status}"));
            break;
        }

        break;
    }

    let final_url = current_url;

    // If we have an error already (from redirects), return early
    if error.is_some() {
        return Ok(CrawlResult::new(
            pages,
            final_url,
            redirect_count,
            false,
            error,
            all_cookies,
            normalized_urls,
        ));
    }

    // Pre-compile include/exclude regexes once before the BFS loop (D5: error on invalid patterns)
    let exclude_regexes: Vec<Regex> =
        compile_regexes(config.exclude_paths.as_deref().unwrap_or(&[]))?;
    let include_regexes: Vec<Regex> =
        compile_regexes(config.include_paths.as_deref().unwrap_or(&[]))?;

    // B8: fetch and parse robots.txt if configured
    let robots_rules: Option<RobotsRules> = if config.respect_robots_txt {
        fetch_robots_rules(&final_url, config, &client).await
    } else {
        None
    };

    // Start BFS from final_url
    queue.push_back((final_url.clone(), 0));
    let dedup_key = normalize_url_for_dedup(&final_url);
    visited_dedup.insert(dedup_key);

    while let Some((page_url, depth)) = queue.pop_front() {
        if pages.len() >= max_pages {
            break;
        }

        // Check include/exclude paths using pre-compiled regexes
        if let Ok(pu) = Url::parse(&page_url) {
            let path = pu.path();
            if !exclude_regexes.is_empty() && exclude_regexes.iter().any(|re| re.is_match(path)) {
                continue;
            }
            if !include_regexes.is_empty()
                && depth > 0
                && !include_regexes.iter().any(|re| re.is_match(path))
            {
                continue;
            }

            // B8: check robots.txt rules
            if let Some(ref rules) = robots_rules
                && !is_path_allowed(path, rules)
            {
                continue;
            }
        }

        let resp = match http_fetch(&page_url, config, &client).await {
            Ok(r) => r,
            Err(_) => continue,
        };

        if config.cookies_enabled {
            all_cookies.extend(extract_cookies(&resp.headers));
        }

        let status_code = resp.status;
        let content_type = resp.content_type;
        // S4: apply max_body_size truncation
        let mut body = resp.body;
        if let Some(max_size) = config.max_body_size
            && body.len() > max_size
        {
            body.truncate(max_size);
        }
        let body_size = body.len();

        let is_binary = is_binary_content_type(&content_type) || is_binary_url(&page_url);
        let is_pdf = is_pdf_content(&content_type, &body) || is_pdf_url(&page_url);
        let page_was_skipped = is_binary || is_pdf;

        if page_was_skipped {
            was_skipped = true;
        }

        let detected_charset = detect_charset(&content_type, &body);

        let is_html = is_html_content(&content_type, &body);
        let page_parsed = Url::parse(&page_url).unwrap_or_else(|_| parsed_url.clone());

        // Scope the non-Send `Html` document so it is dropped before any `.await`
        let (metadata, links, images, feeds, json_ld) = {
            let doc = Html::parse_document(&body);
            let metadata = if is_html && !page_was_skipped {
                extract_metadata(&doc, &body)
            } else {
                PageMetadata::default()
            };
            let links = if is_html && !page_was_skipped {
                extract_links(&doc, &page_parsed)
            } else {
                Vec::new()
            };
            let images = if is_html && !page_was_skipped {
                extract_images(&doc, &page_parsed)
            } else {
                Vec::new()
            };
            let feeds = if is_html && !page_was_skipped {
                extract_feeds(&doc)
            } else {
                Vec::new()
            };
            let json_ld = if is_html && !page_was_skipped {
                extract_json_ld(&doc)
            } else {
                Vec::new()
            };
            (metadata, links, images, feeds, json_ld)
        };

        let norm_url = normalize_url(&page_url);
        let stayed_on_domain = page_parsed
            .host_str()
            .map(|h| h == base_host)
            .unwrap_or(false);

        normalized_urls.push(norm_url.clone());

        // Collect child URLs to enqueue before moving links into the page result
        let mut child_urls: Vec<(String, usize)> = Vec::new();
        if depth < max_depth && !page_was_skipped {
            for link in &links {
                if link.link_type == LinkType::Internal || link.link_type == LinkType::Document {
                    let link_url = strip_fragment(&link.url);

                    // Check stay_on_domain (B4: proper subdomain check)
                    if config.stay_on_domain
                        && let Ok(lu) = Url::parse(&link_url)
                    {
                        let link_host = lu.host_str().unwrap_or("");
                        if link_host != base_host
                            && (!config.allow_subdomains
                                || !link_host.ends_with(&format!(".{base_host}")))
                        {
                            continue;
                        }
                    }

                    let dedup_key = normalize_url_for_dedup(&link_url);
                    if !visited_dedup.contains(&dedup_key) {
                        visited_dedup.insert(dedup_key);
                        child_urls.push((link_url, depth + 1));
                    }
                }
            }
        }

        let page = CrawlPageResult {
            url: page_url.clone(),
            normalized_url: norm_url,
            status_code,
            content_type,
            html: body,
            body_size,
            metadata,
            links,
            images,
            feeds,
            json_ld,
            depth,
            stayed_on_domain,
            was_skipped: page_was_skipped,
            is_pdf,
            detected_charset,
        };

        // B6: send page event through the channel if streaming
        if let Some(ref sender) = tx
            && sender
                .send(CrawlEvent::Page(Box::new(page.clone())))
                .await
                .is_err()
        {
            // Receiver dropped; stop crawling
            break;
        }

        pages.push(page);

        // Add child links to queue
        for child in child_urls {
            queue.push_back(child);
        }
    }

    // Deduplicate cookies by (name, domain, path)
    let mut seen_cookies: HashSet<(String, String, String)> = HashSet::new();
    all_cookies.retain(|c| {
        seen_cookies.insert((
            c.name.clone(),
            c.domain.clone().unwrap_or_default(),
            c.path.clone().unwrap_or_default(),
        ))
    });

    Ok(CrawlResult::new(
        pages,
        final_url,
        redirect_count,
        was_skipped,
        error,
        all_cookies,
        normalized_urls,
    ))
}

/// Find the byte offset of `needle` (ASCII only) in `haystack` using case-insensitive matching.
///
/// Returns `Some(pos)` where `pos` is the byte offset in the original `haystack` string,
/// safe for slicing because `needle` is pure ASCII.
fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    if needle_bytes.len() > haystack_bytes.len() {
        return None;
    }
    for i in 0..=(haystack_bytes.len() - needle_bytes.len()) {
        if haystack_bytes[i..i + needle_bytes.len()]
            .iter()
            .zip(needle_bytes.iter())
            .all(|(h, n)| h.to_ascii_lowercase() == *n)
        {
            return Some(i);
        }
    }
    None
}

/// Compile a slice of regex pattern strings, returning an error if any pattern is invalid.
fn compile_regexes(patterns: &[String]) -> Result<Vec<Regex>, CrawlError> {
    patterns
        .iter()
        .map(|pat| {
            Regex::new(pat)
                .map_err(|e| CrawlError::Other(format!("invalid regex pattern \"{pat}\": {e}")))
        })
        .collect()
}

/// Fetch and parse robots.txt for the given URL's origin.
///
/// Returns `None` if the fetch fails (gracefully degraded).
async fn fetch_robots_rules(
    url: &str,
    config: &CrawlConfig,
    client: &reqwest::Client,
) -> Option<RobotsRules> {
    let parsed = Url::parse(url).ok()?;
    let robots_url = format!("{}://{}/robots.txt", parsed.scheme(), parsed.host_str()?);
    let ua = config
        .user_agent
        .as_deref()
        .unwrap_or(concat!("kreuzcrawl/", env!("CARGO_PKG_VERSION")));
    let resp = http_fetch(&robots_url, config, client).await.ok()?;
    if resp.status >= 400 {
        return None;
    }
    Some(parse_robots_txt(&resp.body, ua))
}
