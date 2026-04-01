//! Single-page scrape operation.

use scraper::Html;
use url::Url;

use crate::assets;
use crate::browser_detect;
use crate::error::CrawlError;
use crate::html::{
    apply_remove_tags, detect_charset, detect_nofollow, detect_noindex, extract_main_content,
    extract_page_data, is_binary_content_type, is_binary_url, is_html_content, is_pdf_content,
};
use crate::http::{
    HttpResponse, build_client, extract_response_meta, fetch_with_retry, http_fetch,
};
use crate::normalize::robots_url;
use crate::robots::{is_path_allowed, parse_robots_txt};
#[cfg(feature = "browser")]
use crate::types::BrowserMode;
use crate::types::{CrawlConfig, PageMetadata, ScrapeResult};

/// Scrape a single URL and return structured data about the page.
///
/// Fetches the URL, optionally checks robots.txt, extracts metadata, links,
/// images, feeds, and JSON-LD data. Handles authentication, content filtering,
/// and charset detection.
pub async fn scrape(url: &str, config: &CrawlConfig) -> Result<ScrapeResult, CrawlError> {
    let parsed_url = Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
    let client = build_client(config)?;

    let auth_header_sent = config.auth.is_some();

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
    #[allow(unused_mut)]
    let mut browser_used = false;

    // In Always mode with browser feature, skip HTTP and go straight to browser.
    #[cfg(feature = "browser")]
    let resp = if matches!(config.browser.mode, BrowserMode::Always) {
        match crate::browser::browser_fetch(url, config, None, config.browser_pool.as_deref()).await
        {
            Ok(r) => {
                browser_used = true;
                r
            }
            Err(e) => return Err(e),
        }
    } else {
        match fetch_with_retry(url, config, &client).await {
            Ok(r) => r,
            // WAF-blocked: try browser fallback in Auto mode.
            Err(CrawlError::WafBlocked(_)) if matches!(config.browser.mode, BrowserMode::Auto) => {
                match crate::browser::browser_fetch(
                    url,
                    config,
                    None,
                    config.browser_pool.as_deref(),
                )
                .await
                {
                    Ok(r) => {
                        browser_used = true;
                        r
                    }
                    Err(e) => return Err(e),
                }
            }
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
                    js_render_hint: false,
                    browser_used: false,
                });
            }
            Err(e) => return Err(e),
        }
    };

    #[cfg(not(feature = "browser"))]
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
                js_render_hint: false,
                browser_used: false,
            });
        }
        Err(e) => return Err(e),
    };

    let mut result = scrape_from_response(
        &parsed_url,
        resp,
        config,
        is_allowed,
        crawl_delay,
        auth_header_sent,
        &client,
    )
    .await?;
    result.browser_used = browser_used;

    // Auto re-fetch with browser when JS rendering is detected and browser feature is enabled.
    #[cfg(feature = "browser")]
    if result.js_render_hint && !browser_used && matches!(config.browser.mode, BrowserMode::Auto) {
        // Note: prior HTTP cookies are not forwarded to the browser.
        // Cookie state from the HTTP response is lost on browser re-fetch.
        if let Ok(browser_resp) =
            crate::browser::browser_fetch(url, config, None, config.browser_pool.as_deref()).await
        {
            // Re-run extraction on browser-rendered HTML. This replaces the original result.
            let mut browser_result = scrape_from_response(
                &parsed_url,
                browser_resp,
                config,
                is_allowed,
                crawl_delay,
                auth_header_sent,
                &client,
            )
            .await?;
            browser_result.browser_used = true;
            browser_result.js_render_hint = true;
            return Ok(browser_result);
        }
    }

    Ok(result)
}

/// Build a `ScrapeResult` from an `HttpResponse` — shared logic between HTTP and browser paths.
///
/// This extracts metadata, links, images, feeds, JSON-LD, and assets from the response,
/// applying the same processing pipeline regardless of how the HTML was fetched.
async fn scrape_from_response(
    parsed_url: &Url,
    resp: HttpResponse,
    config: &CrawlConfig,
    is_allowed: bool,
    crawl_delay: Option<u64>,
    auth_header_sent: bool,
    client: &reqwest::Client,
) -> Result<ScrapeResult, CrawlError> {
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

    let was_skipped =
        is_binary_content_type(&content_type) || is_binary_url(parsed_url.as_str()) || is_pdf;

    // Remove tags if specified
    if !config.remove_tags.is_empty() {
        body = apply_remove_tags(&body, &config.remove_tags);
    }

    // Extract main content only
    let main_content_active = config.main_content_only;
    if main_content_active {
        body = extract_main_content(&body);
    }

    // Parse HTML and extract all data synchronously (Html is not Send,
    // so all doc usage must complete before any .await point).
    let is_html = is_html_content(&content_type, &body);
    let response_meta = extract_response_meta(&headers);

    let (extraction, asset_refs) = {
        let doc = Html::parse_document(&body);

        if !noindex_detected {
            noindex_detected = detect_noindex(&doc);
        }
        if !nofollow_detected {
            nofollow_detected = detect_nofollow(&doc);
        }

        let extraction = extract_page_data(&doc, &body, parsed_url, is_html, true);

        let asset_refs = if config.download_assets && is_html {
            assets::discover_assets(&doc, parsed_url)
        } else {
            Vec::new()
        };

        (extraction, asset_refs)
    };
    // doc is now dropped — future is Send from here

    // Detect if page content suggests JavaScript rendering is needed.
    let word_count = extraction.metadata.word_count.unwrap_or(0);
    let js_render_hint = is_html && browser_detect::detect_js_render_needed(&body, word_count);

    // Download discovered assets
    let downloaded_assets = if !asset_refs.is_empty() {
        assets::download_assets(asset_refs, config, client).await
    } else {
        Vec::new()
    };

    Ok(ScrapeResult {
        status_code,
        content_type,
        html: body,
        body_size,
        metadata: extraction.metadata,
        links: extraction.links,
        images: extraction.images,
        feeds: extraction.feeds,
        json_ld: extraction.json_ld,
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
        js_render_hint,
        browser_used: false,
    })
}
