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
use crate::http::{extract_cookies, http_fetch};
use crate::normalize::{normalize_url, normalize_url_for_dedup, resolve_redirect};
use crate::types::{CookieInfo, CrawlConfig, CrawlPageResult, CrawlResult, PageMetadata};

/// Crawl a website starting from the given URL using breadth-first traversal.
///
/// Follows links up to `max_depth` hops, respecting `max_pages`, domain restrictions,
/// include/exclude path patterns, and redirect handling. Returns all crawled pages
/// with their metadata, links, images, feeds, and JSON-LD data.
pub async fn crawl(url: &str, config: &CrawlConfig) -> Result<CrawlResult, CrawlError> {
    let parsed_url = Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
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
        let resp = match http_fetch(&current_url, config).await {
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

        // Check for Refresh header redirect
        if let Some(refresh) = resp.headers.get("refresh").and_then(|v| v.to_str().ok())
            && let Some(pos) = refresh.to_lowercase().find("url=")
        {
            let target_path = refresh[pos + 4..].trim();
            let target = resolve_redirect(&current_url, target_path);
            if !seen_redirects.contains(&target) {
                seen_redirects.insert(target.clone());
                redirect_count += 1;
                current_url = target;
                continue;
            }
        }

        // Check for meta refresh
        if is_html_content(&resp.content_type, &resp.body) {
            let doc = Html::parse_document(&resp.body);
            if let Some(refresh_target) = detect_meta_refresh(&doc) {
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

    // Start BFS from final_url
    queue.push_back((final_url.clone(), 0));
    let dedup_key = normalize_url_for_dedup(&final_url);
    visited_dedup.insert(dedup_key);

    while let Some((page_url, depth)) = queue.pop_front() {
        if pages.len() >= max_pages {
            break;
        }

        // Check include/exclude paths
        if let Ok(pu) = Url::parse(&page_url) {
            let path = pu.path();
            if let Some(ref excludes) = config.exclude_paths {
                let should_exclude = excludes
                    .iter()
                    .any(|pat| Regex::new(pat).map(|re| re.is_match(path)).unwrap_or(false));
                if should_exclude {
                    continue;
                }
            }
            if let Some(ref includes) = config.include_paths {
                // Root page (depth 0) is always included
                if depth > 0 {
                    let should_include = includes
                        .iter()
                        .any(|pat| Regex::new(pat).map(|re| re.is_match(path)).unwrap_or(false));
                    if !should_include {
                        continue;
                    }
                }
            }
        }

        let resp = match http_fetch(&page_url, config).await {
            Ok(r) => r,
            Err(_) => continue,
        };

        if config.cookies_enabled {
            all_cookies.extend(extract_cookies(&resp.headers));
        }

        let content_type = resp.content_type.clone();
        let status_code = resp.status;
        let body = resp.body.clone();
        let body_size = body.len();

        let is_binary = is_binary_content_type(&content_type) || is_binary_url(&page_url);
        let is_pdf = is_pdf_content(&content_type, &body) || is_pdf_url(&page_url);
        let page_was_skipped = is_binary || is_pdf;

        if page_was_skipped {
            was_skipped = true;
        }

        let detected_charset = detect_charset(&content_type, &body);

        let is_html = is_html_content(&content_type, &body);
        let doc = Html::parse_document(&body);
        let page_parsed = Url::parse(&page_url).unwrap_or_else(|_| parsed_url.clone());

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
                if link.link_type == "internal" || link.link_type == "document" {
                    // Strip fragment from URL before queueing
                    let link_url = if let Ok(mut u) = Url::parse(&link.url) {
                        u.set_fragment(None);
                        u.to_string()
                    } else {
                        link.url.clone()
                    };

                    // Check stay_on_domain
                    if config.stay_on_domain
                        && let Ok(lu) = Url::parse(&link_url)
                    {
                        let link_host = lu.host_str().unwrap_or("");
                        if link_host != base_host
                            && (!config.allow_subdomains || !link_host.ends_with(&base_host))
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

        pages.push(CrawlPageResult {
            url: page_url.clone(),
            normalized_url: norm_url,
            status_code,
            content_type: content_type.clone(),
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
        });

        // Add child links to queue
        for child in child_urls {
            queue.push_back(child);
        }
    }

    // Deduplicate cookies by name
    let mut seen_cookie_names: HashSet<String> = HashSet::new();
    all_cookies.retain(|c| seen_cookie_names.insert(c.name.clone()));

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
