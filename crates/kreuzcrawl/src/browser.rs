//! Headless Chrome/CDP browser fallback for fetching JavaScript-rendered pages.
//!
//! This module is only compiled when the `browser` feature is enabled.

use std::time::Duration;

use chromiumoxide::Handler;
use chromiumoxide::browser::{Browser, BrowserConfig as ChromeBrowserConfig};
use chromiumoxide::cdp::browser_protocol::network::{
    Headers, SetCookieParams, SetExtraHttpHeadersParams,
};
use tokio_stream::StreamExt;

use crate::browser_pool::BrowserPool;
use crate::error::CrawlError;
use crate::http::HttpResponse;
use crate::types::{AuthConfig, BrowserWait, CookieInfo, CrawlConfig};

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
) -> Result<HttpResponse, CrawlError> {
    if let Some(pool) = pool {
        let pooled = pool.acquire_page().await?;
        let result = page_fetch(url, config, pooled.page(), prior_cookies).await;
        pooled.close().await;
        result
    } else {
        let (browser, mut handler) = launch_or_connect(config).await?;
        let handler_handle = tokio::spawn(async move { while handler.next().await.is_some() {} });

        let page = browser
            .new_page("about:blank")
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to create page: {e}")))?;

        let result = page_fetch(url, config, &page, prior_cookies).await;

        drop(browser);
        let _ = tokio::time::timeout(Duration::from_secs(5), handler_handle).await;

        result
    }
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
    // Set user agent if configured.
    if let Some(ref ua) = config.user_agent {
        page.set_user_agent(ua)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to set user agent: {e}")))?;
    }

    // Set cookies from prior HTTP response.
    if let Some(cookies) = prior_cookies {
        for cookie in cookies {
            let mut builder = SetCookieParams::builder()
                .name(&cookie.name)
                .value(&cookie.value);
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
        Some(AuthConfig::Header {
            ref name,
            ref value,
        }) => {
            extra_headers.insert(name.clone(), serde_json::Value::String(value.clone()));
        }
        _ => {}
    }
    if !extra_headers.is_empty() {
        let params =
            SetExtraHttpHeadersParams::new(Headers::new(serde_json::Value::Object(extra_headers)));
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
async fn launch_or_connect(config: &CrawlConfig) -> Result<(Browser, Handler), CrawlError> {
    if let Some(ref endpoint) = config.browser.endpoint {
        Browser::connect(endpoint)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to connect to {endpoint}: {e}")))
    } else {
        let browser_config = ChromeBrowserConfig::builder()
            .no_sandbox()
            .new_headless_mode()
            .build()
            .map_err(|e| CrawlError::BrowserError(format!("invalid browser config: {e}")))?;

        Browser::launch(browser_config)
            .await
            .map_err(|e| CrawlError::BrowserError(format!("failed to launch browser: {e}")))
    }
}
