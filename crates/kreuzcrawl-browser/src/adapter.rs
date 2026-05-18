//! Kreuzcrawl-facing adapter for the native browser backend.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub use crate::page::PageError;

use crate::context::BrowserContext;
use crate::lifecycle::WaitUntil;
use crate::page::Page;

/// A cookie passed into or captured from the native browser.
#[derive(Debug, Clone)]
pub struct NativeCookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
}

/// A single network event recorded during page navigation.
#[derive(Debug, Clone)]
pub struct NativeNetworkEvent {
    pub url: String,
    pub method: String,
    pub resource_type: String,
    pub status: u16,
    pub request_headers: HashMap<String, String>,
    pub response_headers: HashMap<String, String>,
    pub body_size: usize,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct NativeBrowserConfig {
    pub user_agent: Option<String>,
    pub timeout: Duration,
    pub wait_until: NativeBrowserWait,
    pub extra_headers: HashMap<String, String>,
    pub respect_robots_txt: bool,
    /// Use Chrome 145 TLS fingerprint via wreq stealth client.
    pub stealth: bool,
    /// Proxy URL (http/https only). No SOCKS5 — use chromiumoxide for that.
    pub proxy_url: Option<String>,
    /// Cookies pre-populated into the jar before navigation.
    pub prior_cookies: Vec<NativeCookie>,
    /// URL patterns to block (supports `*` wildcards).
    pub block_url_patterns: Vec<String>,
    /// JavaScript snippet evaluated after navigation.
    pub eval_script: Option<String>,
    /// CSS selector to wait for (used when `wait_until == Selector`).
    pub wait_selector: Option<String>,
    /// User-agent for robots.txt fetches. Defaults to `user_agent`.
    pub robots_user_agent: Option<String>,
    /// Capture the full network event stream into the result.
    pub capture_network_events: bool,
}

impl Default for NativeBrowserConfig {
    fn default() -> Self {
        Self {
            user_agent: None,
            timeout: Duration::from_secs(30),
            wait_until: NativeBrowserWait::NetworkIdle,
            extra_headers: HashMap::new(),
            respect_robots_txt: false,
            stealth: false,
            proxy_url: None,
            prior_cookies: Vec::new(),
            block_url_patterns: Vec::new(),
            eval_script: None,
            wait_selector: None,
            robots_user_agent: None,
            capture_network_events: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeBrowserWait {
    Load,
    NetworkIdle,
    /// Poll `document.querySelector(selector)` every 100 ms until found.
    Selector,
}

#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub final_url: String,
    pub status: Option<u16>,
    pub html: String,
    pub headers: HashMap<String, String>,
    /// Return value of `eval_script`, when provided.
    pub eval_result: Option<serde_json::Value>,
    /// Network events recorded during navigation (populated when `capture_network_events`).
    pub network_events: Vec<NativeNetworkEvent>,
    /// All non-expired cookies from the jar after navigation.
    pub cookies: Vec<NativeCookie>,
}

pub async fn render_url(url: &str, config: &NativeBrowserConfig) -> Result<RenderedPage, PageError> {
    let mut context = BrowserContext::with_full_options(
        "kreuzcrawl".to_string(),
        config.proxy_url.clone(),
        config.stealth,
        config.user_agent.clone(),
    );
    context.obey_robots = config.respect_robots_txt;
    if let Some(ref robots_ua) = config.robots_user_agent {
        context.user_agent = robots_ua.clone();
    }
    let context = Arc::new(context);
    context
        .http_client
        .set_extra_headers(config.extra_headers.clone())
        .await;

    // Pre-populate the cookie jar from prior_cookies.
    for cookie in &config.prior_cookies {
        context.cookie_jar.set_parsed_cookie(
            &cookie.name,
            &cookie.value,
            cookie.domain.as_deref(),
            cookie.path.as_deref(),
            cookie.secure,
            cookie.http_only,
        );
    }

    render_with_context(url, config, context).await
}

async fn render_with_context(
    url: &str,
    config: &NativeBrowserConfig,
    context: Arc<BrowserContext>,
) -> Result<RenderedPage, PageError> {
    let mut page = Page::new("page-1".to_string(), context.clone());

    // Wire URL-pattern blocking before navigation so even subresource
    // fetches during navigate_single are filtered.
    if !config.block_url_patterns.is_empty() {
        page.intercept_enabled = true;
        page.intercept_block_patterns = config.block_url_patterns.clone();
    }

    let wait_until = match config.wait_until {
        NativeBrowserWait::Load => WaitUntil::Load,
        NativeBrowserWait::NetworkIdle | NativeBrowserWait::Selector => WaitUntil::NetworkIdle0,
    };

    tokio::time::timeout(config.timeout, page.navigate_with_wait(url, wait_until))
        .await
        .map_err(|_| PageError::NetworkError(format!("browser timed out after {:?}", config.timeout)))??;

    // Selector wait: poll document.querySelector every 100 ms within the
    // remaining timeout budget. We use the already-elapsed time to avoid
    // re-starting the full timeout.
    if config.wait_until == NativeBrowserWait::Selector
        && let Some(ref selector) = config.wait_selector
    {
        let deadline = tokio::time::Instant::now() + config.timeout;
        loop {
            let expr = format!("!!document.querySelector({selector:?})");
            let found = page.evaluate(&expr);
            if found.as_bool() == Some(true) {
                break;
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(PageError::NetworkError(format!(
                    "browser timed out waiting for selector '{selector}' after {:?}",
                    config.timeout
                )));
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    let final_url = page.url_string();
    let status = page
        .network_events
        .iter()
        .rev()
        .find(|event| event.resource_type == "Document")
        .map(|event| event.status);
    let headers = page
        .network_events
        .iter()
        .rev()
        .find(|event| event.resource_type == "Document")
        .map(|event| (*event.response_headers).clone())
        .unwrap_or_default();

    // Optional eval_script.
    let eval_result = if let Some(ref script) = config.eval_script {
        let val = page.evaluate(script);
        if val.is_null() { None } else { Some(val) }
    } else {
        None
    };

    // Network events snapshot.
    let network_events = if config.capture_network_events {
        page.network_events
            .iter()
            .map(|ev| NativeNetworkEvent {
                url: ev.url.clone(),
                method: ev.method.clone(),
                resource_type: ev.resource_type.clone(),
                status: ev.status,
                request_headers: ev.headers.clone(),
                response_headers: (*ev.response_headers).clone(),
                body_size: ev.body_size,
                timestamp_ms: (ev.timestamp * 1000.0) as u64,
            })
            .collect()
    } else {
        Vec::new()
    };

    // Cookie snapshot.
    let cookies = context
        .cookie_jar
        .snapshot()
        .into_iter()
        .map(|(name, value, domain, path, secure, http_only)| NativeCookie {
            name,
            value,
            domain: Some(domain),
            path: Some(path),
            secure,
            http_only,
        })
        .collect();

    let html = rendered_html(&page)
        .ok_or_else(|| PageError::ParseError(format!("no rendered DOM available for {final_url}")))?;

    Ok(RenderedPage {
        final_url,
        status,
        html,
        headers,
        eval_result,
        network_events,
        cookies,
    })
}

fn rendered_html(page: &Page) -> Option<String> {
    page.with_dom(|dom| {
        if let Some(root) = dom.query_selector("html").ok().flatten() {
            dom.outer_html(root)
        } else {
            dom.outer_html(dom.document())
        }
    })
}
