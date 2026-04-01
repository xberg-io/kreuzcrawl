//! CrawlEngine composes trait implementations into a crawl pipeline.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use regex::Regex;
use scraper::Html;
use tokio_stream::wrappers::ReceiverStream;
use url::Url;

use crate::defaults;
use crate::error::CrawlError;
use crate::helpers::{compile_regexes, fetch_robots_rules, find_ascii_case_insensitive};
use crate::html::{
    detect_charset, detect_meta_refresh, extract_page_data, is_binary_content_type, is_binary_url,
    is_html_content, is_pdf_content, is_pdf_url,
};
use crate::http::{build_client, extract_cookies, http_fetch};
use crate::normalize::{normalize_url, normalize_url_for_dedup, resolve_redirect, strip_fragment};
use crate::robots::{RobotsRules, is_path_allowed};
use crate::traits::*;
use crate::types::*;

/// The main crawl engine, composed of pluggable trait implementations.
#[derive(Clone)]
pub struct CrawlEngine {
    pub(crate) config: CrawlConfig,
    pub(crate) frontier: Arc<dyn Frontier>,
    pub(crate) rate_limiter: Arc<dyn RateLimiter>,
    pub(crate) store: Arc<dyn CrawlStore>,
    pub(crate) middlewares: Vec<Arc<dyn CrawlMiddleware>>,
    pub(crate) event_emitter: Arc<dyn EventEmitter>,
    #[allow(dead_code)] // wired in builder, reqwest integration pending
    pub(crate) dns_resolver: Arc<dyn DnsResolver>,
    pub(crate) strategy: Arc<dyn CrawlStrategy>,
    pub(crate) content_filter: Arc<dyn ContentFilter>,
}

/// Builder for [`CrawlEngine`].
///
/// Any field left unset will be filled with a default implementation
/// from [`crate::defaults`].
pub struct CrawlEngineBuilder {
    config: Option<CrawlConfig>,
    frontier: Option<Arc<dyn Frontier>>,
    rate_limiter: Option<Arc<dyn RateLimiter>>,
    store: Option<Arc<dyn CrawlStore>>,
    middlewares: Vec<Arc<dyn CrawlMiddleware>>,
    event_emitter: Option<Arc<dyn EventEmitter>>,
    dns_resolver: Option<Arc<dyn DnsResolver>>,
    strategy: Option<Arc<dyn CrawlStrategy>>,
    content_filter: Option<Arc<dyn ContentFilter>>,
}

impl CrawlEngineBuilder {
    /// Create a new builder with no fields set.
    pub fn new() -> Self {
        Self {
            config: None,
            frontier: None,
            rate_limiter: None,
            store: None,
            middlewares: Vec::new(),
            event_emitter: None,
            dns_resolver: None,
            strategy: None,
            content_filter: None,
        }
    }

    /// Set the crawl configuration.
    pub fn config(mut self, config: CrawlConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the frontier implementation.
    pub fn frontier(mut self, frontier: impl Frontier + 'static) -> Self {
        self.frontier = Some(Arc::new(frontier));
        self
    }

    /// Set the rate limiter implementation.
    pub fn rate_limiter(mut self, rate_limiter: impl RateLimiter + 'static) -> Self {
        self.rate_limiter = Some(Arc::new(rate_limiter));
        self
    }

    /// Set the store implementation.
    pub fn store(mut self, store: impl CrawlStore + 'static) -> Self {
        self.store = Some(Arc::new(store));
        self
    }

    /// Add a middleware to the pipeline.
    pub fn middleware(mut self, middleware: impl CrawlMiddleware + 'static) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    /// Set the event emitter implementation.
    pub fn event_emitter(mut self, event_emitter: impl EventEmitter + 'static) -> Self {
        self.event_emitter = Some(Arc::new(event_emitter));
        self
    }

    /// Set the DNS resolver implementation.
    pub fn dns_resolver(mut self, dns_resolver: impl DnsResolver + 'static) -> Self {
        self.dns_resolver = Some(Arc::new(dns_resolver));
        self
    }

    /// Set the crawl strategy implementation.
    pub fn strategy(mut self, strategy: impl CrawlStrategy + 'static) -> Self {
        self.strategy = Some(Arc::new(strategy));
        self
    }

    /// Set the content filter implementation.
    pub fn content_filter(mut self, content_filter: impl ContentFilter + 'static) -> Self {
        self.content_filter = Some(Arc::new(content_filter));
        self
    }

    /// Build the [`CrawlEngine`], filling in defaults for any unset fields.
    pub fn build(self) -> CrawlEngine {
        CrawlEngine {
            config: self.config.unwrap_or_default(),
            frontier: self
                .frontier
                .unwrap_or_else(|| Arc::new(defaults::InMemoryFrontier::new())),
            rate_limiter: self
                .rate_limiter
                .unwrap_or_else(|| Arc::new(defaults::NoopRateLimiter)),
            store: self.store.unwrap_or_else(|| Arc::new(defaults::NoopStore)),
            middlewares: self.middlewares,
            event_emitter: self
                .event_emitter
                .unwrap_or_else(|| Arc::new(defaults::NoopEmitter)),
            dns_resolver: self
                .dns_resolver
                .unwrap_or_else(|| Arc::new(defaults::SystemResolver)),
            strategy: self
                .strategy
                .unwrap_or_else(|| Arc::new(defaults::BfsStrategy)),
            content_filter: self
                .content_filter
                .unwrap_or_else(|| Arc::new(defaults::NoopFilter)),
        }
    }
}

impl Default for CrawlEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CrawlEngine {
    /// Create a new [`CrawlEngineBuilder`].
    pub fn builder() -> CrawlEngineBuilder {
        CrawlEngineBuilder::new()
    }

    /// Scrape a single URL, returning the extracted data.
    ///
    /// Runs the middleware chain before and after the scrape call.
    pub async fn scrape(&self, url: &str) -> Result<ScrapeResult, CrawlError> {
        // Run before_request middleware
        let mut req_ctx = RequestContext {
            url: url.to_owned(),
            headers: std::collections::HashMap::new(),
        };
        for mw in &self.middlewares {
            mw.before_request(&mut req_ctx).await?;
        }

        // Perform the scrape
        let result = crate::scrape::scrape(&req_ctx.url, &self.config).await?;

        // Run after_response middleware
        let mut resp_ctx = ResponseContext {
            url: req_ctx.url,
            status: result.status_code,
            content_type: result.content_type.clone(),
            body: result.html.clone(),
            headers: std::collections::HashMap::new(),
        };
        for mw in &self.middlewares {
            mw.after_response(&mut resp_ctx).await?;
        }

        Ok(result)
    }

    /// Crawl a website starting from `url`, following links up to the configured depth.
    ///
    /// Uses the engine's [`CrawlStrategy`] and [`Frontier`] traits to control URL
    /// selection order and deduplication.
    pub async fn crawl(&self, url: &str) -> Result<CrawlResult, CrawlError> {
        self.crawl_with_sender(url, None).await
    }

    /// Crawl a website and return a stream of events as pages are processed.
    ///
    /// Uses the engine's trait implementations (strategy, frontier, etc.) for the crawl.
    pub fn crawl_stream(&self, url: &str) -> ReceiverStream<CrawlEvent> {
        let url = url.to_owned();
        let engine = self.clone();

        let channel_size = self.config.max_concurrent.unwrap_or(4) * 16;
        let (tx, rx) = tokio::sync::mpsc::channel(channel_size);

        tokio::spawn(async move {
            match engine.crawl_with_sender(&url, Some(tx.clone())).await {
                Ok(result) => {
                    let _ = tx
                        .send(CrawlEvent::Complete {
                            pages_crawled: result.pages.len(),
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx
                        .send(CrawlEvent::Error {
                            url: url.clone(),
                            error: e.to_string(),
                        })
                        .await;
                    let _ = tx.send(CrawlEvent::Complete { pages_crawled: 0 }).await;
                }
            }
        });

        ReceiverStream::new(rx)
    }

    /// Scrape multiple URLs concurrently.
    pub async fn batch_scrape(
        &self,
        urls: &[&str],
    ) -> Vec<(String, Result<ScrapeResult, CrawlError>)> {
        crate::batch::batch_scrape(urls, &self.config).await
    }

    /// Discover all pages on a website by following links and sitemaps.
    pub async fn map(&self, url: &str) -> Result<MapResult, CrawlError> {
        crate::map::map(url, &self.config).await
    }

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
        let mut pages: Vec<CrawlPageResult> = Vec::with_capacity(capacity);
        let mut normalized_urls: Vec<String> = Vec::with_capacity(capacity);
        let mut redirect_count: usize = 0;
        let mut error: Option<String> = None;
        let mut was_skipped = false;
        let mut all_cookies: Vec<CookieInfo> = Vec::new();

        let mut pages_failed: usize = 0;
        let mut urls_discovered: usize = 0;
        let mut urls_filtered: usize = 0;

        let start_time = Instant::now();

        // ── Phase 1: resolve initial redirects ──────────────────────────
        let mut current_url = url.to_owned();
        let mut seen_redirects: HashSet<String> = HashSet::with_capacity(max_redirects + 1);
        seen_redirects.insert(current_url.clone());

        loop {
            let resp = match http_fetch(&current_url, &self.config, &client).await {
                Ok(r) => r,
                Err(e) => {
                    error = Some(format!("{e}"));
                    break;
                }
            };

            if self.config.cookies_enabled {
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

            // Check for meta refresh
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

        // ── Phase 2: prepare filters and robots rules ───────────────────
        let exclude_regexes: Vec<Regex> = compile_regexes(&self.config.exclude_paths)?;
        let include_regexes: Vec<Regex> = compile_regexes(&self.config.include_paths)?;

        let robots_rules: Option<RobotsRules> = if self.config.respect_robots_txt {
            fetch_robots_rules(&final_url, &self.config, &client).await
        } else {
            None
        };

        // Pass robots.txt crawl-delay to RateLimiter
        if let Some(ref rules) = robots_rules
            && let Some(delay) = rules.crawl_delay
            && let Ok(parsed) = Url::parse(&final_url)
            && let Some(domain) = parsed.host_str()
        {
            self.rate_limiter
                .set_crawl_delay(domain, Duration::from_secs(delay))
                .await?;
        }

        // ── Phase 3: seed the working set and mark as seen via Frontier ─
        let mut working_set: Vec<FrontierEntry> = Vec::new();

        let dedup_key = normalize_url_for_dedup(&final_url);
        self.frontier.mark_seen(&dedup_key).await?;
        working_set.push(FrontierEntry {
            url: final_url.clone(),
            depth: 0,
            priority: 1.0,
        });

        // ── Phase 4: main crawl loop using CrawlStrategy ───────────────
        while !working_set.is_empty() {
            if pages.len() >= max_pages {
                break;
            }

            // Check strategy stopping condition
            let stats = CrawlStats {
                pages_crawled: pages.len(),
                pages_failed,
                urls_discovered,
                urls_filtered,
                elapsed: start_time.elapsed(),
            };
            if !self.strategy.should_continue(&stats) {
                break;
            }

            // Let the strategy pick the next entry
            let Some(idx) = self.strategy.select_next(&working_set) else {
                break;
            };
            let entry = working_set.swap_remove(idx);
            let page_url = entry.url.clone();
            let depth = entry.depth;

            // Parse URL once for the entire iteration
            let page_parsed = match Url::parse(&page_url) {
                Ok(u) => u,
                Err(_) => continue,
            };
            let domain = page_parsed.host_str().unwrap_or("");
            let path = page_parsed.path();

            // Check include/exclude paths
            if !exclude_regexes.is_empty() && exclude_regexes.iter().any(|re| re.is_match(path)) {
                urls_filtered += 1;
                continue;
            }
            if !include_regexes.is_empty()
                && depth > 0
                && !include_regexes.iter().any(|re| re.is_match(path))
            {
                urls_filtered += 1;
                continue;
            }

            // Check robots.txt rules
            if let Some(ref rules) = robots_rules
                && !is_path_allowed(path, rules)
            {
                urls_filtered += 1;
                continue;
            }

            // Rate limiting
            if !domain.is_empty() {
                self.rate_limiter.acquire(domain).await?;
            }

            // Run before_request middleware
            let mut req_ctx = RequestContext {
                url: page_url.clone(),
                headers: std::collections::HashMap::new(),
            };
            for mw in &self.middlewares {
                mw.before_request(&mut req_ctx).await?;
            }

            let resp = match http_fetch(&req_ctx.url, &self.config, &client).await {
                Ok(r) => r,
                Err(e) => {
                    pages_failed += 1;
                    self.event_emitter
                        .on_error(&ErrorEvent {
                            url: entry.url.clone(),
                            error: e.to_string(),
                        })
                        .await;
                    let _ = self.store.store_error(&entry.url, &e).await;
                    continue;
                }
            };

            if self.config.cookies_enabled {
                all_cookies.extend(extract_cookies(&resp.headers));
            }

            let status_code = resp.status;
            let content_type_for_mw = resp.content_type.clone();
            let body_for_mw = resp.body.clone();

            // Populate ResponseContext.headers from HTTP response
            let mut resp_headers = std::collections::HashMap::new();
            for (name, value) in resp.headers.iter() {
                if let Ok(v) = value.to_str() {
                    resp_headers.insert(name.to_string(), v.to_string());
                }
            }

            // Run after_response middleware
            let mut resp_ctx = ResponseContext {
                url: req_ctx.url,
                status: status_code,
                content_type: content_type_for_mw,
                body: body_for_mw,
                headers: resp_headers,
            };
            for mw in &self.middlewares {
                mw.after_response(&mut resp_ctx).await?;
            }

            // Apply middleware mutations back
            let mut body = resp_ctx.body;

            // Record response for adaptive back-off
            if !domain.is_empty() {
                self.rate_limiter
                    .record_response(domain, status_code)
                    .await?;
            }
            let content_type = resp.content_type;
            if let Some(max_size) = self.config.max_body_size
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

            // Scope the non-Send `Html` document so it is dropped before any `.await`
            let extraction = {
                let doc = Html::parse_document(&body);
                extract_page_data(
                    &doc,
                    &body,
                    &page_parsed,
                    is_html && !page_was_skipped,
                    false,
                )
            };

            let norm_url = normalize_url(&page_url);
            let stayed_on_domain = domain == base_host;

            normalized_urls.push(norm_url.clone());

            // ── Link discovery: add children to working set via Frontier dedup ──
            if depth < max_depth && !page_was_skipped {
                for link in &extraction.links {
                    if link.link_type == LinkType::Internal || link.link_type == LinkType::Document
                    {
                        let link_url = strip_fragment(&link.url);

                        // Check stay_on_domain
                        if self.config.stay_on_domain
                            && let Ok(lu) = Url::parse(&link_url)
                        {
                            let link_host = lu.host_str().unwrap_or("");
                            if link_host != base_host
                                && (!self.config.allow_subdomains
                                    || !link_host.ends_with(&base_host_suffix))
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
                            urls_discovered += 1;
                            self.event_emitter.on_discovered(&link_url, depth + 1).await;
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
                metadata: extraction.metadata,
                links: extraction.links,
                images: extraction.images,
                feeds: extraction.feeds,
                json_ld: extraction.json_ld,
                depth,
                stayed_on_domain,
                was_skipped: page_was_skipped,
                is_pdf,
                detected_charset,
            };

            // Apply content filter -- links are already discovered above,
            // so filtered-out pages still contribute to link discovery.
            let page = match self.content_filter.filter(page).await? {
                Some(filtered_page) => filtered_page,
                None => {
                    urls_filtered += 1;
                    continue;
                }
            };

            // Store the crawl page result
            let _ = self.store.store_crawl_page(&page.url, &page).await;

            // Emit page event
            self.event_emitter
                .on_page(&PageEvent {
                    url: page.url.clone(),
                    status_code: page.status_code,
                    depth: page.depth,
                })
                .await;

            // Send page event through the channel if streaming
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
        }

        // Build final stats and notify store/emitter
        let stats = CrawlStats {
            pages_crawled: pages.len(),
            pages_failed,
            urls_discovered,
            urls_filtered,
            elapsed: start_time.elapsed(),
        };
        let _ = self.store.on_complete(&stats).await;
        self.event_emitter
            .on_complete(&CompleteEvent {
                pages_crawled: pages.len(),
            })
            .await;

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
}
