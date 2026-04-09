//! Core crawl loop implementation.
//!
//! This module contains the internal crawl orchestration logic used by
//! [`CrawlEngine::crawl`] and [`CrawlEngine::crawl_stream`].

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use regex::Regex;
use scraper::Html;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use url::Url;

use std::collections::HashMap;

use tower::Service;

use crate::error::CrawlError;
use crate::helpers::{compile_regexes, fetch_robots_rules, find_ascii_case_insensitive};
use crate::html::{
    HtmlExtraction, detect_charset, detect_meta_refresh, extract_page_data, is_binary_content_type,
    is_binary_url, is_html_content, is_pdf_content, is_pdf_url,
};
use crate::http::{build_client, extract_cookies_from_hashmap};
use crate::normalize::{normalize_url, normalize_url_for_dedup, resolve_redirect, strip_fragment};
use crate::robots::{RobotsRules, is_path_allowed};
use crate::tower::CrawlRequest;
use crate::traits::*;
use crate::types::*;

use super::CrawlEngine;

/// Default concurrency limit when `max_concurrent` is not set.
const DEFAULT_MAX_CONCURRENT: usize = 10;

/// Fallback URL used when a fetched URL fails to parse during extraction.
/// This should never happen in practice since the URL was already fetched successfully.
static FALLBACK_URL: std::sync::LazyLock<Url> =
    std::sync::LazyLock::new(|| Url::parse("http://invalid").expect("static fallback URL"));

/// Result of a concurrent fetch task, holding everything needed to process a completed fetch.
struct FetchResult {
    entry: FrontierEntry,
    status_code: u16,
    content_type: String,
    body: String,
    headers: HashMap<String, Vec<String>>,
    extraction: HtmlExtraction,
    is_binary: bool,
    is_pdf: bool,
    detected_charset: Option<String>,
}

/// Result of blocking HTML extraction within a fetch task.
struct PageExtraction {
    extraction: HtmlExtraction,
    is_binary: bool,
    is_pdf: bool,
    detected_charset: Option<String>,
}

/// Mutable state accumulated during a crawl.
struct CrawlState {
    pages: Vec<CrawlPageResult>,
    normalized_urls: Vec<String>,
    redirect_count: usize,
    error: Option<String>,
    was_skipped: bool,
    all_cookies: Vec<CookieInfo>,
    pages_failed: usize,
    urls_discovered: usize,
    urls_filtered: usize,
}

impl CrawlState {
    fn new(capacity: usize) -> Self {
        Self {
            pages: Vec::with_capacity(capacity),
            normalized_urls: Vec::with_capacity(capacity),
            redirect_count: 0,
            error: None,
            was_skipped: false,
            all_cookies: Vec::new(),
            pages_failed: 0,
            urls_discovered: 0,
            urls_filtered: 0,
        }
    }

    fn into_result(self, final_url: String) -> CrawlResult {
        CrawlResult::new(
            self.pages,
            final_url,
            self.redirect_count,
            self.was_skipped,
            self.error,
            self.all_cookies,
            self.normalized_urls,
        )
    }
}

/// Perform HTML extraction in a blocking context.
///
/// `Html::parse_document` is `!Send`, so this must run via `spawn_blocking`.
fn blocking_extract_page(url: &str, content_type: &str, body: &str) -> PageExtraction {
    let parsed_url = Url::parse(url).unwrap_or_else(|_| FALLBACK_URL.clone());
    let is_binary = is_binary_content_type(content_type) || is_binary_url(url);
    let is_pdf = is_pdf_content(content_type, body) || is_pdf_url(url);
    let is_html = is_html_content(content_type, body);

    let doc = Html::parse_document(body);
    let extraction = extract_page_data(
        &doc,
        body,
        &parsed_url,
        is_html && !is_binary && !is_pdf,
        false,
    );
    let detected_charset = detect_charset(content_type, body);

    PageExtraction {
        extraction,
        is_binary,
        is_pdf,
        detected_charset,
    }
}

impl CrawlEngine {
    /// Internal crawl implementation that uses the engine's trait objects.
    ///
    /// When `tx` is `Some`, each page is sent through the channel as it is processed
    /// so that callers can consume results incrementally via [`crawl_stream`](Self::crawl_stream).
    pub(crate) async fn crawl_with_sender(
        &self,
        url: &str,
        tx: Option<tokio::sync::mpsc::Sender<CrawlEvent>>,
    ) -> Result<CrawlResult, CrawlError> {
        let parsed_url =
            Url::parse(url).map_err(|e| CrawlError::Other(format!("invalid URL: {e}")))?;
        let client = build_client(&self.config)?;
        let base_host = parsed_url.host_str().unwrap_or("").to_owned();
        let base_host_suffix = format!(".{base_host}");
        let max_depth = self.config.max_depth.unwrap_or(0);
        let max_pages = self.config.max_pages.unwrap_or(usize::MAX);
        let max_redirects = self.config.max_redirects;

        let capacity = max_pages.min(1024);
        let mut state = CrawlState::new(capacity);
        let start_time = Instant::now();

        // ── Phase 1: resolve initial redirects ──────────────────────────
        let mut service = self.build_service(&client);
        let final_url = self
            .resolve_initial_redirects(url, max_redirects, &mut service, &mut state)
            .await;

        // If we have an error already (from redirects), return early
        if state.error.is_some() {
            return Ok(state.into_result(final_url));
        }

        // ── Phase 2: prepare filters and robots rules ───────────────────
        let exclude_regexes: Vec<Regex> = compile_regexes(&self.config.exclude_paths)?;
        let include_regexes: Vec<Regex> = compile_regexes(&self.config.include_paths)?;

        let robots_rules: Option<RobotsRules> = if self.config.respect_robots_txt {
            fetch_robots_rules(&final_url, &self.config, &client).await
        } else {
            None
        };

        // Pass robots.txt crawl-delay to RateLimiter
        if let Some(rules) = &robots_rules
            && let Some(delay) = rules.crawl_delay
            && let Ok(parsed) = Url::parse(&final_url)
            && let Some(domain) = parsed.host_str()
        {
            self.rate_limiter
                .set_crawl_delay(domain, Duration::from_secs(delay))
                .await?;
        }

        // ── Phase 3: seed the working set and mark as seen via Frontier ─
        // We maintain a local working_set (Vec) rather than popping from frontier because:
        // 1. The CrawlStrategy needs random access to all candidates via select_next(&[...])
        // 2. The frontier is shared across potential concurrent batch_crawl operations
        // 3. This design keeps the hot path lock-free (no frontier mutex per iteration)
        let mut working_set: Vec<FrontierEntry> = Vec::new();

        let dedup_key = normalize_url_for_dedup(&final_url);
        self.frontier.mark_seen(&dedup_key).await?;
        working_set.push(FrontierEntry {
            url: final_url.clone(),
            depth: 0,
            priority: 1.0,
        });

        // ── Phase 4: main crawl loop ────────────────────────────────────
        self.run_crawl_loop(
            &mut state,
            &mut working_set,
            &mut service,
            &exclude_regexes,
            &include_regexes,
            &robots_rules,
            &base_host,
            &base_host_suffix,
            max_depth,
            max_pages,
            start_time,
            &tx,
        )
        .await?;

        // Safety: ensure we never return more than max_pages
        if state.pages.len() > max_pages {
            state.pages.truncate(max_pages);
        }

        // Build final stats and notify store/emitter
        let stats = CrawlStats {
            pages_crawled: state.pages.len(),
            pages_failed: state.pages_failed,
            urls_discovered: state.urls_discovered,
            urls_filtered: state.urls_filtered,
            elapsed: start_time.elapsed(),
        };
        let _ = self.store.on_complete(&stats).await;
        self.event_emitter
            .on_complete(&CompleteEvent {
                pages_crawled: state.pages.len(),
            })
            .await;

        // Deduplicate cookies by (name, domain, path)
        let mut seen_cookies: HashSet<(String, Option<String>, Option<String>)> = HashSet::new();
        state
            .all_cookies
            .retain(|c| seen_cookies.insert((c.name.clone(), c.domain.clone(), c.path.clone())));

        Ok(state.into_result(final_url))
    }

    /// Follow HTTP, Refresh header, and meta refresh redirects until a final page is reached.
    async fn resolve_initial_redirects(
        &self,
        url: &str,
        max_redirects: usize,
        service: &mut tower::util::BoxCloneService<
            CrawlRequest,
            crate::tower::CrawlResponse,
            CrawlError,
        >,
        state: &mut CrawlState,
    ) -> String {
        let mut current_url = url.to_owned();
        let mut seen_redirects: HashSet<String> = HashSet::with_capacity(max_redirects + 1);
        seen_redirects.insert(current_url.clone());

        loop {
            let resp = match service.call(CrawlRequest::new(&current_url)).await {
                Ok(r) => r,
                Err(e) => {
                    state.error = Some(format!("{e}"));
                    break;
                }
            };

            if self.config.cookies_enabled {
                state
                    .all_cookies
                    .extend(extract_cookies_from_hashmap(&resp.headers));
            }

            let status = resp.status;

            // Check HTTP redirect (3xx with Location header)
            if let Some(target) = self.check_redirect(
                status,
                &resp.headers,
                &current_url,
                &seen_redirects,
                max_redirects,
                state,
            ) {
                seen_redirects.insert(target.clone());
                state.redirect_count += 1;
                current_url = target;
                continue;
            }
            if state.error.is_some() {
                break;
            }

            // Check Refresh header redirect
            if let Some(refresh) = resp.headers.get("refresh").and_then(|v| v.first())
                && let Some(pos) = find_ascii_case_insensitive(refresh, "url=")
            {
                let target_path = refresh[pos + 4..].trim();
                let target = resolve_redirect(&current_url, target_path);
                if let Some(t) =
                    self.try_follow_redirect(&target, &seen_redirects, max_redirects, state)
                {
                    seen_redirects.insert(t.clone());
                    state.redirect_count += 1;
                    current_url = t;
                    continue;
                }
                if state.error.is_some() {
                    break;
                }
            }

            // Check meta refresh
            if is_html_content(&resp.content_type, &resp.body) {
                let doc = Html::parse_document(&resp.body);
                if let Some(refresh_target) = detect_meta_refresh(&doc) {
                    let target = resolve_redirect(&current_url, &refresh_target);
                    if let Some(t) =
                        self.try_follow_redirect(&target, &seen_redirects, max_redirects, state)
                    {
                        seen_redirects.insert(t.clone());
                        state.redirect_count += 1;
                        current_url = t;
                        continue;
                    }
                    if state.error.is_some() {
                        break;
                    }
                }
            }

            // Check for error status on final page (after redirect)
            if status >= 400 && state.redirect_count > 0 {
                state.error = Some(format!("HTTP {status}"));
                break;
            }

            break;
        }

        current_url
    }

    /// Check if a response is an HTTP redirect and return the target URL.
    fn check_redirect(
        &self,
        status: u16,
        headers: &HashMap<String, Vec<String>>,
        current_url: &str,
        seen_redirects: &HashSet<String>,
        max_redirects: usize,
        state: &mut CrawlState,
    ) -> Option<String> {
        let is_redirect = matches!(status, 301 | 302 | 303 | 307 | 308);
        if !is_redirect {
            return None;
        }

        if state.redirect_count >= max_redirects {
            state.error = Some("too many redirects".to_owned());
            return None;
        }

        if let Some(location) = headers.get("location").and_then(|v| v.first()) {
            let target = resolve_redirect(current_url, location);
            if seen_redirects.contains(&target) {
                state.error = Some("redirect loop detected".to_owned());
                return None;
            }
            return Some(target);
        }

        None
    }

    /// Attempt to follow a non-HTTP redirect (Refresh header or meta refresh).
    /// Returns `Some(target)` if the redirect should be followed, `None` otherwise.
    /// Sets `state.error` if max redirects exceeded.
    fn try_follow_redirect(
        &self,
        target: &str,
        seen_redirects: &HashSet<String>,
        max_redirects: usize,
        state: &mut CrawlState,
    ) -> Option<String> {
        if state.redirect_count >= max_redirects {
            state.error = Some("too many redirects".to_owned());
            return None;
        }
        if seen_redirects.contains(target) {
            return None;
        }
        Some(target.to_owned())
    }

    /// Main crawl loop: spawn fetch tasks, process results, discover links.
    #[allow(clippy::too_many_arguments)]
    async fn run_crawl_loop(
        &self,
        state: &mut CrawlState,
        working_set: &mut Vec<FrontierEntry>,
        service: &mut tower::util::BoxCloneService<
            CrawlRequest,
            crate::tower::CrawlResponse,
            CrawlError,
        >,
        exclude_regexes: &[Regex],
        include_regexes: &[Regex],
        robots_rules: &Option<RobotsRules>,
        base_host: &str,
        base_host_suffix: &str,
        max_depth: usize,
        max_pages: usize,
        start_time: Instant,
        tx: &Option<tokio::sync::mpsc::Sender<CrawlEvent>>,
    ) -> Result<(), CrawlError> {
        let max_concurrent = self.config.max_concurrent.unwrap_or(DEFAULT_MAX_CONCURRENT);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut join_set: JoinSet<Result<FetchResult, (FrontierEntry, CrawlError)>> =
            JoinSet::new();
        let mut cancelled = false;

        while !cancelled && (!working_set.is_empty() || !join_set.is_empty()) {
            // 1. Fill JoinSet from working_set, up to max_concurrent
            while join_set.len() < max_concurrent && !working_set.is_empty() {
                if state.pages.len() + join_set.len() >= max_pages {
                    break;
                }

                let stats = CrawlStats {
                    pages_crawled: state.pages.len(),
                    pages_failed: state.pages_failed,
                    urls_discovered: state.urls_discovered,
                    urls_filtered: state.urls_filtered,
                    elapsed: start_time.elapsed(),
                };
                if !self.strategy.should_continue(&stats) {
                    break;
                }

                let Some(idx) = self.strategy.select_next(working_set) else {
                    break;
                };
                let entry = working_set.swap_remove(idx);

                if !self.should_fetch_url(
                    &entry,
                    exclude_regexes,
                    include_regexes,
                    robots_rules,
                    &mut state.urls_filtered,
                ) {
                    continue;
                }

                let permit = semaphore
                    .clone()
                    .acquire_owned()
                    .await
                    .map_err(|_| CrawlError::Other("semaphore closed".into()))?;

                let mut svc = service.clone();

                join_set.spawn(async move {
                    let _permit = permit;

                    let resp = svc
                        .call(CrawlRequest::new(&entry.url))
                        .await
                        .map_err(|e| (entry.clone(), e))?;

                    let status_code = resp.status;
                    let content_type = resp.content_type.clone();
                    let headers = resp.headers.clone();
                    let body = resp.body.clone();

                    let url_for_extract = entry.url.clone();
                    let content_type_clone = content_type.clone();
                    let body_clone = body.clone();

                    let page_ext = tokio::task::spawn_blocking(move || {
                        blocking_extract_page(&url_for_extract, &content_type_clone, &body_clone)
                    })
                    .await
                    .map_err(|e| {
                        (
                            entry.clone(),
                            CrawlError::Other(format!("extraction task failed: {e}")),
                        )
                    })?;

                    Ok(FetchResult {
                        entry,
                        status_code,
                        content_type,
                        body,
                        headers,
                        extraction: page_ext.extraction,
                        is_binary: page_ext.is_binary,
                        is_pdf: page_ext.is_pdf,
                        detected_charset: page_ext.detected_charset,
                    })
                });
            }

            // 2. Collect one completed result (or break if nothing in-flight)
            if join_set.is_empty() {
                break;
            }

            let Some(result) = join_set.join_next().await else {
                break;
            };

            match result {
                Ok(Ok(fetch)) => {
                    let should_stop = self
                        .process_fetch_result(
                            fetch,
                            state,
                            working_set,
                            base_host,
                            base_host_suffix,
                            max_depth,
                            max_pages,
                            tx,
                            &mut join_set,
                        )
                        .await?;
                    if should_stop {
                        cancelled = true;
                    }
                }
                Ok(Err((entry, error))) => {
                    state.pages_failed += 1;
                    self.event_emitter
                        .on_error(&ErrorEvent {
                            url: entry.url.clone(),
                            error: error.to_string(),
                        })
                        .await;
                    let _ = self.store.store_error(&entry.url, &error).await;
                }
                Err(_join_error) => {
                    state.pages_failed += 1;
                }
            }

            // 3. Check stopping condition
            let stats = CrawlStats {
                pages_crawled: state.pages.len(),
                pages_failed: state.pages_failed,
                urls_discovered: state.urls_discovered,
                urls_filtered: state.urls_filtered,
                elapsed: start_time.elapsed(),
            };
            if !self.strategy.should_continue(&stats) {
                break;
            }
        }

        Ok(())
    }

    /// Check whether a URL should be fetched based on path filters and robots.txt.
    fn should_fetch_url(
        &self,
        entry: &FrontierEntry,
        exclude_regexes: &[Regex],
        include_regexes: &[Regex],
        robots_rules: &Option<RobotsRules>,
        urls_filtered: &mut usize,
    ) -> bool {
        let page_parsed = match Url::parse(&entry.url) {
            Ok(u) => u,
            Err(_) => return false,
        };
        let path = page_parsed.path();

        if !exclude_regexes.is_empty() && exclude_regexes.iter().any(|re| re.is_match(path)) {
            *urls_filtered += 1;
            return false;
        }
        // Depth-0 seed URL is always included regardless of include_paths filter
        if !include_regexes.is_empty()
            && entry.depth > 0
            && !include_regexes.iter().any(|re| re.is_match(path))
        {
            *urls_filtered += 1;
            return false;
        }
        if let Some(rules) = robots_rules
            && !is_path_allowed(path, rules)
        {
            *urls_filtered += 1;
            return false;
        }

        true
    }

    /// Process a completed fetch: extract data, discover links, build page result.
    ///
    /// Returns `true` if the crawl should stop (max_pages reached or receiver dropped).
    #[allow(clippy::too_many_arguments)]
    async fn process_fetch_result(
        &self,
        fetch: FetchResult,
        state: &mut CrawlState,
        working_set: &mut Vec<FrontierEntry>,
        base_host: &str,
        base_host_suffix: &str,
        max_depth: usize,
        max_pages: usize,
        tx: &Option<tokio::sync::mpsc::Sender<CrawlEvent>>,
        join_set: &mut JoinSet<Result<FetchResult, (FrontierEntry, CrawlError)>>,
    ) -> Result<bool, CrawlError> {
        let page_url = fetch.entry.url.clone();
        let depth = fetch.entry.depth;

        if self.config.cookies_enabled {
            state
                .all_cookies
                .extend(extract_cookies_from_hashmap(&fetch.headers));
        }

        let mut body = fetch.body;

        if let Some(max_size) = self.config.max_body_size
            && body.len() > max_size
        {
            body.truncate(max_size);
        }
        let body_size = body.len();

        let page_was_skipped = fetch.is_binary || fetch.is_pdf;
        if page_was_skipped {
            state.was_skipped = true;
        }

        let page_parsed = Url::parse(&page_url).unwrap_or_else(|_| FALLBACK_URL.clone());
        let domain = page_parsed.host_str().unwrap_or("");
        let norm_url = normalize_url(&page_url);
        let stayed_on_domain = domain == base_host;

        state.normalized_urls.push(norm_url.clone());

        // Link discovery
        if depth < max_depth && !page_was_skipped {
            self.discover_and_enqueue_links(
                &fetch.extraction.links,
                &page_url,
                depth,
                base_host,
                base_host_suffix,
                working_set,
                &mut state.urls_discovered,
            )
            .await?;
        }

        let markdown = crate::markdown::convert_to_markdown(&body).await;

        let page = CrawlPageResult {
            url: page_url.clone(),
            normalized_url: norm_url,
            status_code: fetch.status_code,
            content_type: fetch.content_type,
            html: body,
            body_size,
            metadata: fetch.extraction.metadata,
            links: fetch.extraction.links,
            images: fetch.extraction.images,
            feeds: fetch.extraction.feeds,
            json_ld: fetch.extraction.json_ld,
            depth,
            stayed_on_domain,
            was_skipped: page_was_skipped,
            is_pdf: fetch.is_pdf,
            detected_charset: fetch.detected_charset,
            markdown,
            extracted_data: None,
            extraction_meta: None,
        };

        // Apply content filter — filtered pages still contribute to link discovery above
        let page = match self.content_filter.filter(page).await? {
            Some(filtered_page) => filtered_page,
            None => {
                state.urls_filtered += 1;
                return Ok(false);
            }
        };

        self.strategy.on_page_processed(&page);
        let _ = self.store.store_crawl_page(&page.url, &page).await;

        self.event_emitter
            .on_page(&PageEvent {
                url: page.url.clone(),
                status_code: page.status_code,
                depth: page.depth,
            })
            .await;

        // Send page event through the channel if streaming
        if let Some(sender) = tx
            && sender
                .send(CrawlEvent::Page(Box::new(page.clone())))
                .await
                .is_err()
        {
            // Receiver dropped; signal cancellation
            return Ok(true);
        }

        state.pages.push(page);

        if state.pages.len() >= max_pages {
            join_set.abort_all();
            return Ok(true);
        }

        Ok(false)
    }

    /// Discover links from a page and add unseen ones to the working set.
    #[allow(clippy::too_many_arguments)]
    async fn discover_and_enqueue_links(
        &self,
        links: &[LinkInfo],
        _page_url: &str,
        depth: usize,
        base_host: &str,
        base_host_suffix: &str,
        working_set: &mut Vec<FrontierEntry>,
        urls_discovered: &mut usize,
    ) -> Result<(), CrawlError> {
        for link in links {
            if link.link_type != LinkType::Internal && link.link_type != LinkType::Document {
                continue;
            }

            let link_url = strip_fragment(&link.url);

            // Check stay_on_domain
            if self.config.stay_on_domain
                && let Ok(lu) = Url::parse(&link_url)
            {
                let link_host = lu.host_str().unwrap_or("");
                if link_host != base_host
                    && (!self.config.allow_subdomains || !link_host.ends_with(base_host_suffix))
                {
                    continue;
                }
            }

            let dedup_key = normalize_url_for_dedup(&link_url);
            if !self.frontier.is_seen(&dedup_key).await? {
                self.frontier.mark_seen(&dedup_key).await?;
                let priority = self.strategy.score_url(&link_url, depth + 1);
                working_set.push(FrontierEntry {
                    url: link_url.clone(),
                    depth: depth + 1,
                    priority,
                });
                *urls_discovered += 1;
                self.event_emitter.on_discovered(&link_url, depth + 1).await;
            }
        }

        Ok(())
    }
}
