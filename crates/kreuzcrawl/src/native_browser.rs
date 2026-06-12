//! Native browser backend adapter — standalone module so it can be used both
//! when only `browser-native` is active and when the full `browser` feature is on.

use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::time::Duration;

use kreuzcrawl_browser::adapter::{NativeBrowserExecutor, NativeCookie as NBCookie};
use tracing::Instrument as _;

use crate::error::CrawlError;
use crate::http::{BrowserExtras, HttpResponse};
use crate::telemetry::attributes::{CRAWL_BROWSER_BACKEND, CRAWL_BROWSER_SESSION_ID, CRAWL_PAGES_RENDERED};
use crate::telemetry::metrics::registry;
use crate::types::{AuthConfig, BrowserWait, CookieInfo, CrawlConfig, ResponseMeta};

/// Process-wide monotonic session counter for `crawl.browser.session_id`.
static NATIVE_SESSION_COUNTER: AtomicU64 = AtomicU64::new(1);

pub(crate) async fn native_browser_fetch(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
    native_executor: &NativeBrowserExecutor,
) -> Result<HttpResponse, CrawlError> {
    let session_id = NATIVE_SESSION_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
    let session_id_str = session_id.to_string();

    let span = tracing::info_span!(
        "crawl.browser.session",
        { CRAWL_BROWSER_BACKEND } = "native",
        { CRAWL_BROWSER_SESSION_ID } = %session_id_str,
        { CRAWL_PAGES_RENDERED } = 1_i64,
    );

    registry().browser_sessions_active.add(1, &[]);
    // Guard: decrement the active-session counter when this scope exits.
    struct SessionGuard;
    impl Drop for SessionGuard {
        fn drop(&mut self) {
            registry().browser_sessions_active.add(-1, &[]);
        }
    }
    let _guard = SessionGuard;

    native_browser_fetch_inner(url, config, prior_cookies, native_executor)
        .instrument(span)
        .await
}

async fn native_browser_fetch_inner(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
    native_executor: &NativeBrowserExecutor,
) -> Result<HttpResponse, CrawlError> {
    if config.browser.endpoint.is_some() {
        return Err(CrawlError::InvalidConfig(
            "browser.endpoint is only supported by the chromiumoxide backend".into(),
        ));
    }

    let mut extra_headers = config.custom_headers.clone();
    match config.auth {
        Some(AuthConfig::Bearer { ref token }) => {
            extra_headers.insert("Authorization".to_owned(), format!("Bearer {token}"));
        }
        Some(AuthConfig::Header { ref name, ref value }) => {
            extra_headers.insert(name.clone(), value.clone());
        }
        _ => {}
    }

    let wait_until = match config.browser.wait {
        BrowserWait::NetworkIdle => kreuzcrawl_browser::adapter::NativeBrowserWait::NetworkIdle,
        BrowserWait::Selector => kreuzcrawl_browser::adapter::NativeBrowserWait::Selector,
        BrowserWait::Fixed => kreuzcrawl_browser::adapter::NativeBrowserWait::Load,
    };

    // Resolve proxy: browser.proxy overrides the top-level config.proxy.
    let resolved_proxy = config.browser.proxy.as_ref().or(config.proxy.as_ref()).map(|p| {
        if p.username.is_some() || p.password.is_some() {
            let user = p.username.as_deref().unwrap_or("");
            let pass = p.password.as_deref().unwrap_or("");
            // Insert credentials into the URL: scheme://user:pass@host:port
            if let Some(rest) = p.url.strip_prefix("http://") {
                format!("http://{user}:{pass}@{rest}")
            } else if let Some(rest) = p.url.strip_prefix("https://") {
                format!("https://{user}:{pass}@{rest}")
            } else {
                p.url.clone()
            }
        } else {
            p.url.clone()
        }
    });

    let prior_native: Vec<NBCookie> = prior_cookies
        .unwrap_or(&[])
        .iter()
        .map(|c| NBCookie {
            name: c.name.clone(),
            value: c.value.clone(),
            domain: c.domain.clone(),
            path: c.path.clone(),
            secure: false,
            http_only: false,
        })
        .collect();

    let native_config = kreuzcrawl_browser::adapter::NativeBrowserConfig {
        user_agent: config.user_agent.clone(),
        timeout: config.browser.timeout,
        wait_until,
        extra_headers,
        respect_robots_txt: config.respect_robots_txt,
        stealth: matches!(config.browser.mode, crate::types::BrowserMode::Stealth),
        proxy_url: resolved_proxy,
        prior_cookies: prior_native,
        block_url_patterns: config.browser.block_url_patterns.clone(),
        eval_script: config.browser.eval_script.clone(),
        wait_selector: config.browser.wait_selector.clone(),
        robots_user_agent: config.browser.robots_user_agent.clone(),
        capture_network_events: config.browser.capture_network_events,
    };

    let timeout = config.browser.timeout;
    let rendered = native_executor.render_url(url, &native_config).await.map_err(|e| {
        let message = e.to_string();
        if message.contains("timed out") {
            CrawlError::BrowserTimeout(format!("browser timed out after {timeout:?}"))
        } else {
            CrawlError::BrowserError(format!("native browser render failed: {message}"))
        }
    })?;

    if config.browser.wait == BrowserWait::Fixed {
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    if let Some(extra) = config.browser.extra_wait {
        tokio::time::sleep(extra).await;
    }

    let content_type = rendered
        .headers
        .get("content-type")
        .cloned()
        .unwrap_or_else(|| "text/html".to_owned());
    let body_bytes = rendered.html.as_bytes().to_vec();

    // Map NativeNetworkEvent → ResponseMeta (best available fields).
    let net_events: Vec<ResponseMeta> = rendered
        .network_events
        .into_iter()
        .map(|ev| ResponseMeta {
            server: ev.response_headers.get("server").cloned(),
            etag: ev.response_headers.get("etag").cloned(),
            last_modified: ev.response_headers.get("last-modified").cloned(),
            cache_control: ev.response_headers.get("cache-control").cloned(),
            x_powered_by: ev.response_headers.get("x-powered-by").cloned(),
            content_language: ev.response_headers.get("content-language").cloned(),
            content_encoding: ev.response_headers.get("content-encoding").cloned(),
        })
        .collect();

    // Map NativeCookie → CookieInfo.
    let cookies: Vec<CookieInfo> = rendered
        .cookies
        .into_iter()
        .map(|c| CookieInfo {
            name: c.name,
            value: c.value,
            domain: c.domain,
            path: c.path,
        })
        .collect();

    let extras = BrowserExtras {
        eval_result: rendered.eval_result,
        network_events: net_events,
        cookies,
    };

    Ok(HttpResponse {
        status: rendered.status.unwrap_or(200),
        content_type,
        body: rendered.html,
        body_bytes,
        headers: rendered.headers.into_iter().map(|(k, v)| (k, vec![v])).collect(),
        browser_extras: Some(extras),
        // Browser navigation resolves the URL internally; use the input URL as
        // the final URL. The native scrape path tracks final_url via
        // follow_redirects / RedirectOutcome — this field is only used by the
        // wasm path which does not go through browser backends.
        final_url: url.to_owned(),
    })
}
