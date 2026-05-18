//! Native browser backend adapter — standalone module so it can be used both
//! when only `browser-native` is active and when the full `browser` feature is on.

use std::time::Duration;

use kreuzcrawl_browser::adapter::NativeCookie as NBCookie;

use crate::error::CrawlError;
use crate::http::{BrowserExtras, HttpResponse};
use crate::types::{AuthConfig, BrowserWait, CookieInfo, CrawlConfig, ResponseMeta};

pub(crate) async fn native_browser_fetch(
    url: &str,
    config: &CrawlConfig,
    prior_cookies: Option<&[CookieInfo]>,
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
        stealth: config.browser.stealth,
        proxy_url: resolved_proxy,
        prior_cookies: prior_native,
        block_url_patterns: config.browser.block_url_patterns.clone(),
        eval_script: config.browser.eval_script.clone(),
        wait_selector: config.browser.wait_selector.clone(),
        robots_user_agent: config.browser.robots_user_agent.clone(),
        capture_network_events: config.browser.capture_network_events,
    };

    let url = url.to_owned();
    let timeout = config.browser.timeout;
    let rendered = tokio::task::spawn_blocking(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("failed to create native browser runtime: {e}"))?;
        runtime
            .block_on(kreuzcrawl_browser::adapter::render_url(&url, &native_config))
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| CrawlError::BrowserError(format!("native browser render task failed: {e}")))?
    .map_err(|e| {
        if e.contains("timed out") {
            CrawlError::BrowserTimeout(format!("browser timed out after {timeout:?}"))
        } else {
            CrawlError::BrowserError(format!("native browser render failed: {e}"))
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
    })
}
