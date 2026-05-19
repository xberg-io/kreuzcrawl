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

const DEFAULT_SCROLL_AMOUNT: i64 = 800;
const DEFAULT_SELECTOR_WAIT_MS: i64 = 30_000;
const SELECTOR_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Scroll direction for native page interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeScrollDirection {
    /// Scroll upward.
    Up,
    /// Scroll downward.
    Down,
}

/// A backend-neutral page action translated for the native browser adapter.
#[derive(Debug, Clone)]
pub enum NativePageAction {
    /// Click an element matched by a CSS selector.
    Click { selector: String },
    /// Type text into an element matched by a CSS selector.
    TypeText { selector: String, text: String },
    /// Dispatch a key press to the active element.
    Press { key: String },
    /// Scroll the page or a scrollable element.
    Scroll {
        direction: NativeScrollDirection,
        selector: Option<String>,
        amount: Option<i64>,
    },
    /// Wait for a duration or selector.
    Wait {
        milliseconds: Option<i64>,
        selector: Option<String>,
    },
    /// Request a screenshot.
    Screenshot { full_page: Option<bool> },
    /// Execute JavaScript in the page context.
    ExecuteJs { script: String },
    /// Capture the current HTML.
    Scrape,
}

/// Result from a single native page action.
#[derive(Debug, Clone)]
pub struct NativeActionResult {
    /// Zero-based action index in the submitted sequence.
    pub action_index: usize,
    /// Stable action type string.
    pub action_type: String,
    /// Whether the action completed successfully.
    pub success: bool,
    /// Action-specific return data.
    pub data: Option<serde_json::Value>,
    /// Error message for failed actions.
    pub error: Option<String>,
}

/// Result of native interaction execution.
#[derive(Debug, Clone)]
pub struct NativeInteractionResult {
    /// Per-action execution results.
    pub action_results: Vec<NativeActionResult>,
    /// Final page HTML after all actions.
    pub final_html: String,
    /// Final page URL after all actions.
    pub final_url: String,
    /// Screenshot bytes when supported and requested.
    pub screenshot: Option<Vec<u8>>,
}

pub async fn render_url(url: &str, config: &NativeBrowserConfig) -> Result<RenderedPage, PageError> {
    let context = create_context(config).await;
    render_with_context(url, config, context).await
}

/// Navigate to a URL and execute page actions using the native browser backend.
pub async fn interact_url(
    url: &str,
    config: &NativeBrowserConfig,
    actions: &[NativePageAction],
    post_navigation_wait: Option<Duration>,
) -> Result<NativeInteractionResult, PageError> {
    let context = create_context(config).await;
    let mut page = Page::new("page-1".to_string(), context);
    configure_page_interception(&mut page, config);
    navigate_configured(&mut page, url, config).await?;

    if let Some(wait) = post_navigation_wait {
        tokio::time::sleep(wait).await;
    }
    if let Some(ref script) = config.eval_script {
        page.evaluate_result(script)
            .map_err(|e| PageError::ParseError(format!("post-navigation eval_script failed: {e}")))?;
    }

    let mut action_results = Vec::with_capacity(actions.len());
    let mut screenshot = None;
    for (index, action) in actions.iter().enumerate() {
        match execute_action(&mut page, action).await {
            Ok(data) => {
                if let Some(bytes) = data.screenshot {
                    screenshot = Some(bytes);
                }
                action_results.push(NativeActionResult {
                    action_index: index,
                    action_type: action_type(action).to_owned(),
                    success: true,
                    data: data.data,
                    error: None,
                });
            }
            Err(error) => {
                action_results.push(NativeActionResult {
                    action_index: index,
                    action_type: action_type(action).to_owned(),
                    success: false,
                    data: None,
                    error: Some(error),
                });
            }
        }
    }

    let final_url = page.url_string();
    let final_html = rendered_html(&page)
        .ok_or_else(|| PageError::ParseError(format!("no rendered DOM available for {final_url}")))?;

    Ok(NativeInteractionResult {
        action_results,
        final_html,
        final_url,
        screenshot,
    })
}

async fn create_context(config: &NativeBrowserConfig) -> Arc<BrowserContext> {
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

    context
}

async fn render_with_context(
    url: &str,
    config: &NativeBrowserConfig,
    context: Arc<BrowserContext>,
) -> Result<RenderedPage, PageError> {
    let mut page = Page::new("page-1".to_string(), context.clone());
    configure_page_interception(&mut page, config);
    navigate_configured(&mut page, url, config).await?;

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

fn configure_page_interception(page: &mut Page, config: &NativeBrowserConfig) {
    if !config.block_url_patterns.is_empty() {
        page.intercept_enabled = true;
        page.intercept_block_patterns = config.block_url_patterns.clone();
    }
}

async fn navigate_configured(page: &mut Page, url: &str, config: &NativeBrowserConfig) -> Result<(), PageError> {
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
            let found = selector_exists(page, selector)
                .map_err(|e| PageError::ParseError(format!("invalid wait selector {selector:?}: {e}")))?;
            if found {
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

    Ok(())
}

struct NativeActionData {
    data: Option<serde_json::Value>,
    screenshot: Option<Vec<u8>>,
}

impl NativeActionData {
    fn empty() -> Self {
        Self {
            data: None,
            screenshot: None,
        }
    }

    fn data(data: serde_json::Value) -> Self {
        Self {
            data: Some(data),
            screenshot: None,
        }
    }
}

async fn execute_action(page: &mut Page, action: &NativePageAction) -> Result<NativeActionData, String> {
    match action {
        NativePageAction::Click { selector } => {
            click(page, selector).await?;
            Ok(NativeActionData::empty())
        }
        NativePageAction::TypeText { selector, text } => {
            type_text(page, selector, text)?;
            Ok(NativeActionData::empty())
        }
        NativePageAction::Press { key } => {
            press(page, key).await?;
            Ok(NativeActionData::empty())
        }
        NativePageAction::Scroll {
            direction,
            selector,
            amount,
        } => {
            scroll(page, *direction, selector.as_deref(), *amount)?;
            Ok(NativeActionData::empty())
        }
        NativePageAction::Wait { milliseconds, selector } => {
            wait_for_action(page, *milliseconds, selector.as_deref()).await?;
            Ok(NativeActionData::empty())
        }
        NativePageAction::Screenshot { full_page } => {
            let scope = if full_page.unwrap_or(false) {
                "full-page"
            } else {
                "viewport"
            };
            Err(format!(
                "Native backend does not support {scope} screenshot actions because it has no visual layout renderer; use BrowserBackend::Chromiumoxide for screenshots"
            ))
        }
        NativePageAction::ExecuteJs { script } => page.evaluate_result(script).map(NativeActionData::data),
        NativePageAction::Scrape => {
            let final_url = page.url_string();
            let html = rendered_html(page).ok_or_else(|| format!("no rendered DOM available for {final_url}"))?;
            Ok(NativeActionData::data(serde_json::json!({ "html": html })))
        }
    }
}

async fn click(page: &mut Page, selector: &str) -> Result<(), String> {
    validate_selector_syntax(page, selector)?;
    let selector_json = json_string(selector, "selector")?;
    let script = format!(
        r#"
        (() => {{
            const selector = {selector_json};
            const target = document.querySelector(selector);
            if (!target) {{
                return {{ ok: false, error: `click target not found: ${{selector}}` }};
            }}
            target.click();
            return {{ ok: true }};
        }})()
        "#
    );
    let result = page
        .evaluate_result(&script)
        .map_err(|e| format!("click selector evaluation failed: {e}"))?;
    expect_ok(result, "click")?;
    page.process_pending_navigation()
        .await
        .map_err(|e| format!("failed to process click navigation: {e}"))?;
    Ok(())
}

fn type_text(page: &mut Page, selector: &str, text: &str) -> Result<(), String> {
    validate_selector_syntax(page, selector)?;
    let selector_json = json_string(selector, "selector")?;
    let text_json = json_string(text, "text")?;
    let script = format!(
        r#"
        (() => {{
            const selector = {selector_json};
            const text = {text_json};
            const target = document.querySelector(selector);
            if (!target) {{
                return {{ ok: false, error: `type target not found: ${{selector}}` }};
            }}
            target.focus && target.focus();
            const current = target.value == null ? "" : String(target.value);
            target.value = current + text;
            target.dispatchEvent(new Event("input", {{ bubbles: true }}));
            target.dispatchEvent(new Event("change", {{ bubbles: true }}));
            return {{ ok: true }};
        }})()
        "#
    );
    let result = page
        .evaluate_result(&script)
        .map_err(|e| format!("type selector evaluation failed: {e}"))?;
    expect_ok(result, "type")
}

async fn press(page: &mut Page, key: &str) -> Result<(), String> {
    let key_json = json_string(key, "key")?;
    let script = format!(
        r#"
        (() => {{
            const key = {key_json};
            const target = document.activeElement || document.body || document;
            for (const eventType of ["keydown", "keypress", "keyup"]) {{
                target.dispatchEvent(new KeyboardEvent(eventType, {{ key, bubbles: true, cancelable: true }}));
            }}
            return {{ ok: true }};
        }})()
        "#
    );
    expect_ok(page.evaluate(&script), "press")?;
    page.process_pending_navigation()
        .await
        .map_err(|e| format!("failed to process key navigation: {e}"))?;
    Ok(())
}

fn scroll(
    page: &mut Page,
    direction: NativeScrollDirection,
    selector: Option<&str>,
    amount: Option<i64>,
) -> Result<(), String> {
    let amount = amount.unwrap_or(DEFAULT_SCROLL_AMOUNT).saturating_abs();
    let signed_amount = match direction {
        NativeScrollDirection::Up => -amount,
        NativeScrollDirection::Down => amount,
    };
    if let Some(selector) = selector {
        validate_selector_syntax(page, selector)?;
    }
    let selector_json = json_option_string(selector, "selector")?;
    let script = format!(
        r#"
        (() => {{
            const selector = {selector_json};
            if (selector) {{
                const target = document.querySelector(selector);
                if (!target) {{
                    return {{ ok: false, error: `scroll target not found: ${{selector}}` }};
                }}
                target.scrollTop = (target.scrollTop || 0) + {signed_amount};
                return {{ ok: true }};
            }}
            if (typeof window.scrollBy === "function") {{
                window.scrollBy(0, {signed_amount});
            }}
            globalThis.__kreuzcrawlScrollY = (globalThis.__kreuzcrawlScrollY || 0) + {signed_amount};
            return {{ ok: true }};
        }})()
        "#
    );
    let result = page
        .evaluate_result(&script)
        .map_err(|e| format!("scroll selector evaluation failed: {e}"))?;
    expect_ok(result, "scroll")
}

async fn wait_for_action(page: &mut Page, milliseconds: Option<i64>, selector: Option<&str>) -> Result<(), String> {
    if let Some(milliseconds) = milliseconds
        && milliseconds < 0
    {
        return Err(format!("wait time {milliseconds}ms must not be negative"));
    }

    if let Some(selector) = selector {
        let wait_ms = milliseconds.unwrap_or(DEFAULT_SELECTOR_WAIT_MS) as u64;
        let deadline = tokio::time::Instant::now() + Duration::from_millis(wait_ms);
        loop {
            if selector_exists(page, selector)? {
                return Ok(());
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(format!("timed out waiting for selector {selector:?}"));
            }
            tokio::time::sleep(SELECTOR_POLL_INTERVAL).await;
        }
    }

    if let Some(milliseconds) = milliseconds {
        tokio::time::sleep(Duration::from_millis(milliseconds as u64)).await;
    }
    Ok(())
}

fn selector_exists(page: &mut Page, selector: &str) -> Result<bool, String> {
    if let Some(result) = page.with_dom(|dom| dom.query_selector(selector)) {
        return result
            .map(|node| node.is_some())
            .map_err(|e| format!("selector syntax error: {e}"));
    }

    let selector_json = json_string(selector, "selector")?;
    let script = format!("!!document.querySelector({selector_json})");
    let found = page
        .evaluate_result(&script)
        .map_err(|e| format!("wait selector evaluation failed: {e}"))?;
    Ok(found.as_bool().unwrap_or(false))
}

fn validate_selector_syntax(page: &Page, selector: &str) -> Result<(), String> {
    if let Some(result) = page.with_dom(|dom| dom.query_selector(selector)) {
        result.map(|_| ()).map_err(|e| format!("selector syntax error: {e}"))?;
    }
    Ok(())
}

fn expect_ok(value: serde_json::Value, operation: &str) -> Result<(), String> {
    if value.get("ok").and_then(serde_json::Value::as_bool) == Some(true) {
        return Ok(());
    }
    if let Some(error) = value.get("error").and_then(serde_json::Value::as_str) {
        return Err(error.to_owned());
    }
    Err(format!("native {operation} script returned {value}"))
}

fn json_string(value: &str, field: &str) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("failed to encode {field}: {e}"))
}

fn json_option_string(value: Option<&str>, field: &str) -> Result<String, String> {
    serde_json::to_string(&value).map_err(|e| format!("failed to encode {field}: {e}"))
}

fn action_type(action: &NativePageAction) -> &'static str {
    match action {
        NativePageAction::Click { .. } => "click",
        NativePageAction::TypeText { .. } => "type",
        NativePageAction::Press { .. } => "press",
        NativePageAction::Scroll { .. } => "scroll",
        NativePageAction::Wait { .. } => "wait",
        NativePageAction::Screenshot { .. } => "screenshot",
        NativePageAction::ExecuteJs { .. } => "executeJs",
        NativePageAction::Scrape => "scrape",
    }
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
