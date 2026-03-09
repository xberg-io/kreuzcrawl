//! Single-page scrape operation.

use scraper::Html;
use url::Url;

use crate::assets;
use crate::error::CrawlError;
use crate::html::{
    apply_remove_tags, compute_word_count, detect_charset, detect_nofollow, detect_noindex,
    extract_favicons, extract_feeds, extract_headings, extract_hreflangs, extract_images,
    extract_json_ld, extract_links, extract_main_content, extract_metadata, is_binary_content_type,
    is_binary_url, is_html_content, is_pdf_content,
};
use crate::http::{build_client, extract_response_meta, fetch_with_retry, http_fetch};
use crate::normalize::robots_url;
use crate::robots::{is_path_allowed, parse_robots_txt};
use crate::types::{CrawlConfig, PageMetadata, ScrapeResult};

/// Scrape a single URL and return structured data about the page.
///
/// Fetches the URL, optionally checks robots.txt, extracts metadata, links,
/// images, feeds, and JSON-LD data. Handles authentication, content filtering,
/// and charset detection.
pub async fn scrape(url: &str, config: &CrawlConfig) -> Result<ScrapeResult, CrawlError> {
    let parsed_url = Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
    let client = build_client(config)?;

    let auth_header_sent =
        config.auth_basic.is_some() || config.auth_bearer.is_some() || config.auth_header.is_some();

    // Check robots.txt
    let mut is_allowed = true;
    let mut crawl_delay = None;
    if config.respect_robots_txt {
        let robots = robots_url(&parsed_url);
        if let Ok(robots_resp) = http_fetch(&robots, config, &client).await {
            let ua = config.user_agent.as_deref().unwrap_or("*");
            let rules = parse_robots_txt(&robots_resp.body, ua);
            is_allowed = is_path_allowed(parsed_url.path(), &rules);
            crawl_delay = rules.crawl_delay;
        }
    }

    // Fetch page (even if not allowed, we still fetch to report status).
    // When respect_robots_txt is enabled, a 404 is not an error -- the page
    // simply doesn't exist, but the robots check result is still meaningful.
    let resp = match fetch_with_retry(url, config, &client).await {
        Ok(r) => r,
        Err(CrawlError::NotFound(_)) if config.respect_robots_txt => {
            return Ok(ScrapeResult {
                status_code: 404,
                content_type: String::new(),
                html: String::new(),
                body_size: 0,
                metadata: PageMetadata::default(),
                links: Vec::new(),
                images: Vec::new(),
                feeds: Vec::new(),
                json_ld: Vec::new(),
                is_allowed,
                crawl_delay,
                noindex_detected: false,
                nofollow_detected: false,
                x_robots_tag: None,
                is_pdf: false,
                was_skipped: false,
                detected_charset: None,
                main_content_only: config.main_content_only,
                auth_header_sent,
                response_meta: None,
                assets: Vec::new(),
            });
        }
        Err(e) => return Err(e),
    };

    let status_code = resp.status;
    let content_type = resp.content_type;
    let headers = resp.headers;
    let mut body = resp.body;

    let detected_charset = detect_charset(&content_type, &body);
    let is_pdf = is_pdf_content(&content_type, &body);

    // Handle max body size
    let mut body_size = body.len();
    if let Some(max_size) = config.max_body_size
        && body.len() > max_size
    {
        body.truncate(max_size);
        body_size = max_size;
    }

    // Check for X-Robots-Tag
    let x_robots_tag = headers
        .get("x-robots-tag")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let mut noindex_detected = false;
    let mut nofollow_detected = false;

    if let Some(ref xrt) = x_robots_tag {
        let lower = xrt.to_lowercase();
        if lower.contains("noindex") {
            noindex_detected = true;
        }
        if lower.contains("nofollow") {
            nofollow_detected = true;
        }
    }

    let was_skipped = is_binary_content_type(&content_type) || is_binary_url(url) || is_pdf;

    // Remove tags if specified
    if let Some(ref tags) = config.remove_tags {
        body = apply_remove_tags(&body, tags);
    }

    // Extract main content only
    let main_content_active = config.main_content_only;
    if main_content_active {
        body = extract_main_content(&body);
    }

    // Parse HTML for metadata, links, images, feeds, json-ld
    let is_html = is_html_content(&content_type, &body);
    let doc = Html::parse_document(&body);

    if !noindex_detected {
        noindex_detected = detect_noindex(&doc);
    }
    if !nofollow_detected {
        nofollow_detected = detect_nofollow(&doc);
    }

    let response_meta = extract_response_meta(&headers);

    let mut metadata = if is_html {
        extract_metadata(&doc, &body)
    } else {
        PageMetadata::default()
    };

    if is_html {
        let hreflangs = extract_hreflangs(&doc);
        if !hreflangs.is_empty() {
            metadata.hreflangs = Some(hreflangs);
        }
        let favicons = extract_favicons(&doc);
        if !favicons.is_empty() {
            metadata.favicons = Some(favicons);
        }
        let headings = extract_headings(&doc);
        if !headings.is_empty() {
            metadata.headings = Some(headings);
        }
        metadata.word_count = Some(compute_word_count(&doc));
    }

    let links = if is_html {
        extract_links(&doc, &parsed_url)
    } else {
        Vec::new()
    };
    let images = if is_html {
        extract_images(&doc, &parsed_url)
    } else {
        Vec::new()
    };
    let feeds = if is_html {
        extract_feeds(&doc)
    } else {
        Vec::new()
    };
    let json_ld = if is_html {
        extract_json_ld(&doc)
    } else {
        Vec::new()
    };

    // Download assets if configured
    let downloaded_assets = if config.download_assets && is_html {
        let asset_refs = assets::discover_assets(&doc, &parsed_url);
        assets::download_assets(asset_refs, config, &client).await
    } else {
        Vec::new()
    };

    Ok(ScrapeResult {
        status_code,
        content_type,
        html: body,
        body_size,
        metadata,
        links,
        images,
        feeds,
        json_ld,
        is_allowed,
        crawl_delay,
        noindex_detected,
        nofollow_detected,
        x_robots_tag,
        is_pdf,
        was_skipped,
        detected_charset,
        main_content_only: main_content_active,
        auth_header_sent,
        response_meta: Some(response_meta),
        assets: downloaded_assets,
    })
}
