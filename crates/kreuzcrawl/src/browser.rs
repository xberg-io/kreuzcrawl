//! Headless Chrome/CDP browser fallback for fetching JavaScript-rendered pages.
//!
//! This module is only compiled when the `browser` feature is enabled.

use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::time::Duration;

use chromiumoxide::Handler;
use chromiumoxide::browser::{Browser, BrowserConfig as ChromeBrowserConfig};
use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use chromiumoxide::cdp::browser_protocol::network::{Headers, SetCookieParams, SetExtraHttpHeadersParams};
use tokio_stream::StreamExt;
use tracing::Instrument as _;

use crate::browser_pool::BrowserPool;
use crate::error::CrawlError;
use crate::http::HttpResponse;
use crate::telemetry::attributes::{CRAWL_BROWSER_BACKEND, CRAWL_BROWSER_SESSION_ID, CRAWL_PAGES_RENDERED};
use crate::telemetry::metrics::registry;
use crate::types::{AuthConfig, BrowserBackend, BrowserWait, CookieInfo, CrawlConfig};

/// Process-wide monotonic session counter for `crawl.browser.session_id`.
static BROWSER_SESSION_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Fetch a URL using a headless Chrome browser via CDP.
///
/// When `pool` is `Some`, acquires a page from the pool, uses it, and returns
/// it on completion. When `pool` is `None`, launches a one-shot browser
/// instance and tears it down afterwards.
///
/// Returns an `HttpResponse` compatible with the existing scrape pipeline.
pub(crate) async fn browser_fetch(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
    pool: Option<&BrowserPool>,
    #[cfg(feature = "browser-native")] native_executor: Option<&kreuzcrawl_browser::adapter::NativeBrowserExecutor>,
) -> Result<HttpResponse, CrawlError> {
    match config.browser.backend {
        BrowserBackend::Chromiumoxide => chromiumoxide_fetch(url, config, prior_cookies, pool).await,
        BrowserBackend::Native => {
            #[cfg(feature = "browser-native")]
            {
                native_fetch(url, config, prior_cookies, native_executor).await
            }
            #[cfg(not(feature = "browser-native"))]
            {
                native_fetch(url, config, prior_cookies).await
            }
        }
    }
}

async fn chromiumoxide_fetch(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
    pool: Option<&BrowserPool>,
) -> Result<HttpResponse, CrawlError> {
    let session_id = BROWSER_SESSION_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
    let session_id_str = session_id.to_string();

    let span = tracing::info_span!(
        "crawl.browser.session",
        { CRAWL_BROWSER_BACKEND } = "chromiumoxide",
        { CRAWL_BROWSER_SESSION_ID } = %session_id_str,
        { CRAWL_PAGES_RENDERED } = 1_i64,
    );

    registry().browser_sessions_active.add(1, &[]);
    // Guard: decrement the active-session counter when this scope exits.
    // Fires even on early return or error path.
    struct SessionGuard;
    impl Drop for SessionGuard {
        fn drop(&mut self) {
            registry().browser_sessions_active.add(-1, &[]);
        }
    }
    let _guard = SessionGuard;

    chromiumoxide_fetch_inner(url, config, prior_cookies, pool)
        .instrument(span)
        .await
}

async fn chromiumoxide_fetch_inner(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
    pool: Option<&BrowserPool>,
) -> Result<HttpResponse, CrawlError> {
    if let Some(pool) = pool {
        // Attempt to reuse a session from the session pool if session affinity
        // is enabled. If no session exists or affinity is disabled, acquire a
        // fresh page from the browser pool.
        let page = if config.browser.session_affinity {
            let session_key = crate::browser_session_pool::SessionKey::from_url(
                url,
                config.browser.proxy.as_ref().map(|p| p.url.as_str()),
            )?;
            let session_pool = config.browser_session_pool.as_deref().ok_or_else(|| {
                CrawlError::BrowserError("session_affinity enabled but session pool is not configured".into())
            })?;

            // Try to acquire an existing session.
            if let Some(pooled_page) = session_pool.acquire(&session_key).await {
                pooled_page
            } else {
                // No session available; acquire a fresh page from the browser pool.
                let pooled = pool.acquire_page().await?;
                pooled.page().clone()
            }
        } else {
            // Affinity disabled; always get a fresh page.
            let pooled = pool.acquire_page().await?;
            pooled.page().clone()
        };

        let result = page_fetch(url, config, &page, prior_cookies).await;

        // If session affinity is enabled and the fetch succeeded, stash the page
        // in the session pool for reuse. Otherwise, close it.
        if config.browser.session_affinity
            && result.is_ok()
            && let Ok(session_key) = crate::browser_session_pool::SessionKey::from_url(
                url,
                config.browser.proxy.as_ref().map(|p| p.url.as_str()),
            )
            && let Some(session_pool) = config.browser_session_pool.as_deref()
        {
            session_pool.insert(session_key, page).await;
        } else {
            let _ = page.close().await;
        }

        result
    } else {
        let (mut browser, mut handler, data_dir) = launch_or_connect(config).await?;
        let handler_handle = tokio::spawn(async move { while handler.next().await.is_some() {} });

        let page = browser
            .new_page("about:blank")
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to create page: {e}")))?;

        let result = page_fetch(url, config, &page, prior_cookies).await;

        let _ = page.close().await;
        let _ = browser.close().await;
        let _ = browser.wait().await;
        drop(browser);
        let _ = tokio::time::timeout(Duration::from_secs(5), handler_handle).await;

        // Clean up the temporary user data directory.
        if let Some(dir) = data_dir {
            let _ = std::fs::remove_dir_all(&dir);
        }

        result
    }
}

#[cfg(feature = "browser-native")]
async fn native_fetch(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
    native_executor: Option<&kreuzcrawl_browser::adapter::NativeBrowserExecutor>,
) -> Result<HttpResponse, CrawlError> {
    let native_executor = native_executor.ok_or_else(|| {
        CrawlError::BrowserError("native browser executor is not available for BrowserBackend::Native".into())
    })?;
    crate::native_browser::native_browser_fetch(url, config, prior_cookies, native_executor).await
}

#[cfg(not(feature = "browser-native"))]
async fn native_fetch(
    _url: &str,
    _config: &CrawlConfig,
    _prior_cookies: Option<&[CookieInfo]>,
) -> Result<HttpResponse, CrawlError> {
    Err(CrawlError::InvalidConfig(
        "browser.backend = native requires the browser-native feature".into(),
    ))
}

/// Navigate a pre-existing CDP page to `url`, wait for rendering, and extract
/// the final HTML. The caller provides the page; this function does not
/// create or close it.
async fn page_fetch(
    url: &str,
    config: &CrawlConfig,
    page: &chromiumoxide::Page,
    prior_cookies: Option<&[CookieInfo]>,
) -> Result<HttpResponse, CrawlError> {
    let stealth = matches!(config.browser.mode, crate::types::BrowserMode::Stealth);

    // Inject stealth patches only when BrowserMode::Stealth is active.
    if stealth {
        crate::stealth::apply_stealth_patches(page).await;
    }

    // Resolve user agent: caller-supplied > stealth-enabled default > implicit browser default.
    let resolved_ua = if let Some(ref ua) = config.user_agent {
        ua.clone()
    } else if stealth {
        // Use a modern Chrome UA when BrowserMode::Stealth is active and no explicit UA is set.
        resolve_default_user_agent().to_string()
    } else {
        // Fall through to chromiumoxide's default behavior.
        "".to_string()
    };

    // Set user agent if resolved to non-empty.
    if !resolved_ua.is_empty() {
        page.set_user_agent(&resolved_ua)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to set user agent: {e}")))?;
    }

    // Set viewport when BrowserMode::Stealth is active (default 1920x1080).
    if stealth && let Err(e) = set_viewport(page, 1920, 1080).await {
        return Err(CrawlError::BrowserError(format!("failed to set viewport: {e}")));
    }

    // Set cookies from prior HTTP response.
    if let Some(cookies) = prior_cookies {
        for cookie in cookies {
            let mut builder = SetCookieParams::builder().name(&cookie.name).value(&cookie.value);
            if let Some(ref domain) = cookie.domain {
                builder = builder.domain(domain);
            }
            if let Some(ref path) = cookie.path {
                builder = builder.path(path);
            }
            if let Ok(params) = builder.build() {
                // Cookie setting is best-effort — some cookies may be rejected.
                let _ = page.execute(params).await;
            }
        }
    }

    // Set custom headers (including auth).
    let mut extra_headers = serde_json::Map::new();
    for (k, v) in &config.custom_headers {
        extra_headers.insert(k.clone(), serde_json::Value::String(v.clone()));
    }
    match config.auth {
        Some(AuthConfig::Bearer { ref token }) => {
            extra_headers.insert(
                "Authorization".to_owned(),
                serde_json::Value::String(format!("Bearer {token}")),
            );
        }
        Some(AuthConfig::Header { ref name, ref value }) => {
            extra_headers.insert(name.clone(), serde_json::Value::String(value.clone()));
        }
        _ => {}
    }
    if !extra_headers.is_empty() {
        let params = SetExtraHttpHeadersParams::new(Headers::new(serde_json::Value::Object(extra_headers)));
        page.execute(params)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to set headers: {e}")))?;
    }

    // Navigate and wait for rendering, all under a single timeout.
    let timeout = config.browser.timeout;
    tokio::time::timeout(timeout, async {
        page.goto(url)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("navigation failed: {e}")))?;

        wait_for_ready(page, config)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("wait failed: {e}")))?;

        Ok::<(), CrawlError>(())
    })
    .await
    .map_err(|_| CrawlError::BrowserTimeout(format!("browser timed out after {timeout:?}")))??;

    // Extra wait if configured.
    if let Some(extra) = config.browser.extra_wait {
        tokio::time::sleep(extra).await;
    }

    // Extract rendered HTML.
    let html = page
        .content()
        .await
        .map_err(|e| CrawlError::BrowserError(format!("failed to extract HTML: {e}")))?;

    // body_bytes duplicates body for consistency with the HTTP path
    // which needs raw bytes for binary/charset detection.
    let body_bytes = html.as_bytes().to_vec();

    // Note: CDP page.content() does not expose the HTTP status code.
    // We return 200 for all successfully-rendered pages. The actual
    // HTTP status is not available through this code path.
    Ok(HttpResponse {
        status: 200,
        content_type: "text/html".to_owned(),
        body: html,
        body_bytes,
        headers: std::collections::HashMap::new(),
        browser_extras: None,
        // CDP navigation resolves the URL internally; use the input URL as the
        // final URL. The native scrape path tracks final_url via
        // follow_redirects — this field is only consumed by the wasm scrape
        // path which does not use browser backends.
        final_url: url.to_owned(),
    })
}

/// Wait for the page to be ready based on the configured wait strategy.
async fn wait_for_ready(
    page: &chromiumoxide::Page,
    config: &CrawlConfig,
) -> Result<(), chromiumoxide::error::CdpError> {
    match config.browser.wait {
        BrowserWait::NetworkIdle => {
            // Note: true CDP network idle detection (zero in-flight requests)
            // is not implemented. This is a settle delay that gives client-side
            // JS time to execute after the initial page load.
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        BrowserWait::Selector => {
            if let Some(ref selector) = config.browser.wait_selector {
                page.find_element(selector).await?;
            } else {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
        BrowserWait::Fixed => {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
    Ok(())
}

/// Launch a new managed browser or connect to an external CDP endpoint.
///
/// Each launch creates a unique user data directory to avoid Chrome's
/// `SingletonLock` conflicts when multiple instances run concurrently
/// or a previous instance crashed without cleanup.
async fn launch_or_connect(config: &CrawlConfig) -> Result<(Browser, Handler, Option<std::path::PathBuf>), CrawlError> {
    if let Some(ref endpoint) = config.browser.endpoint {
        let (browser, handler) = Browser::connect(endpoint)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to connect to {endpoint}: {e}")))?;
        Ok((browser, handler, None))
    } else {
        // Use a unique temp directory per launch to avoid SingletonLock conflicts.
        use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
        static LAUNCH_COUNTER: AtomicU64 = AtomicU64::new(0);
        let user_data_dir = std::env::temp_dir().join(format!(
            "kreuzcrawl-browser-{}-{}",
            std::process::id(),
            LAUNCH_COUNTER.fetch_add(1, AtomicOrdering::Relaxed),
        ));

        let mut builder = ChromeBrowserConfig::builder()
            .no_sandbox()
            .new_headless_mode()
            .user_data_dir(&user_data_dir)
            .disable_default_args();
        // macOS 26 + Chrome 148+ trip Apple's fork-safety check on Chrome's
        // internal helper-process forks. See browser_pool::launch_browser for
        // the long-form rationale; same env-var pair applied here so both
        // launch paths (one-shot vs. pooled) behave consistently.
        builder = builder
            .env("OBJC_DISABLE_INITIALIZE_FORK_SAFETY", "YES")
            .env("OS_ACTIVITY_MODE", "disable");
        for arg in crate::browser_pool::safe_default_args() {
            builder = builder.arg(arg);
        }
        let browser_config = builder
            .build()
            .map_err(|e| CrawlError::BrowserError(format!("invalid browser config: {e}")))?;

        match Browser::launch(browser_config).await {
            Ok((browser, handler)) => Ok((browser, handler, Some(user_data_dir))),
            Err(e) => {
                // Clean up the temp dir on failure so it doesn't leak.
                let _ = std::fs::remove_dir_all(&user_data_dir);
                Err(CrawlError::BrowserError(format!("failed to launch browser: {e}")))
            }
        }
    }
}

/// Returns a modern Chrome user-agent string suitable for the runtime environment.
/// Used as the default UA when stealth mode is enabled.
fn resolve_default_user_agent() -> &'static str {
    // Chrome 145 user agents per platform. The chromiumoxide path does not
    // receive runtime platform info, so we default to Linux (worker standard).
    // B2 may add per-page UA rotation; for now return the Linux Chrome string.
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36"
}

/// Set the viewport (device metrics) via CDP Emulation.setDeviceMetricsOverride.
async fn set_viewport(page: &chromiumoxide::Page, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let params = SetDeviceMetricsOverrideParams::builder()
        .width(width)
        .height(height)
        .device_scale_factor(1.0)
        .build()?;

    page.execute(params).await?;
    Ok(())
}
