//! HTTP fetching with redirect handling, retry logic, and cookie extraction.

use std::time::Duration;

use reqwest::header::{CONTENT_TYPE, HeaderMap, USER_AGENT};

use crate::error::{CrawlError, classify_reqwest_error, error_chain_string};
use crate::types::{AuthConfig, CookieInfo, CrawlConfig, ResponseMeta};

/// An HTTP response with status, headers, and body content.
pub(crate) struct HttpResponse {
    pub(crate) status: u16,
    pub(crate) content_type: String,
    pub(crate) body: String,
    pub(crate) body_bytes: Vec<u8>,
    pub(crate) headers: HeaderMap,
}

/// Perform a single HTTP GET request with the given configuration.
///
/// Handles user-agent, authentication, custom headers, error status codes,
/// and content-length validation.
pub(crate) async fn http_fetch(
    url: &str,
    config: &CrawlConfig,
    client: &reqwest::Client,
) -> Result<HttpResponse, CrawlError> {
    let mut req = client.get(url);

    // Set user-agent
    if let Some(ref ua) = config.user_agent {
        req = req.header(USER_AGENT, ua.as_str());
    } else {
        req = req.header(
            USER_AGENT,
            concat!("kreuzcrawl/", env!("CARGO_PKG_VERSION")),
        );
    }

    // Auth
    match config.auth {
        Some(AuthConfig::Basic {
            ref username,
            ref password,
        }) => {
            req = req.basic_auth(username, Some(password));
        }
        Some(AuthConfig::Bearer { ref token }) => {
            req = req.bearer_auth(token);
        }
        Some(AuthConfig::Header {
            ref name,
            ref value,
        }) => {
            req = req.header(name.as_str(), value.as_str());
        }
        None => {}
    }

    // Custom headers
    for (k, v) in &config.custom_headers {
        req = req.header(k.as_str(), v.as_str());
    }

    let resp = req.send().await.map_err(|e| classify_reqwest_error(&e))?;

    let status = resp.status().as_u16();

    // Get content type from the last value (wiremock appends headers)
    let content_type = resp
        .headers()
        .get_all(CONTENT_TYPE)
        .iter()
        .next_back()
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_owned();

    let headers = resp.headers().clone();

    // Check for error status codes
    match status {
        401 => return Err(CrawlError::Unauthorized("unauthorized".into())),
        403 => {
            // Check for WAF
            let server = headers
                .get("server")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            let body = resp.text().await.unwrap_or_default();
            if server.to_lowercase().contains("cloudflare")
                || body.contains("cf-browser-verification")
                || body.contains("challenge-form")
            {
                return Err(CrawlError::WafBlocked("waf/blocked detected".into()));
            }
            return Err(CrawlError::Forbidden("forbidden".into()));
        }
        404 => return Err(CrawlError::NotFound("not_found".into())),
        408 => return Err(CrawlError::Timeout("timeout: request timed out".into())),
        410 => return Err(CrawlError::Gone("gone".into())),
        429 => return Err(CrawlError::RateLimited("rate_limited".into())),
        500 => return Err(CrawlError::ServerError("server_error".into())),
        502 => return Err(CrawlError::BadGateway("bad_gateway".into())),
        503 => {
            return Err(CrawlError::ServerError(
                "server_error: service unavailable".into(),
            ));
        }
        _ => {}
    }

    // Check content-length mismatch (data_loss)
    let expected_len = headers
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<usize>().ok());

    let body_bytes = resp.bytes().await.map_err(|e| {
        // Walk the error source chain to detect body/data-loss errors that
        // reqwest wraps in generic errors.
        let chain = error_chain_string(&e);
        if chain.contains("content-length")
            || chain.contains("truncat")
            || chain.contains("incomplete")
            || chain.contains("end of file")
            || chain.contains("body error")
            || chain.contains("body from connection")
            || e.is_body()
        {
            CrawlError::DataLoss(format!("data_loss: {e}"))
        } else {
            classify_reqwest_error(&e)
        }
    })?;

    if let Some(expected) = expected_len
        && body_bytes.len() < expected
        && expected - body_bytes.len() > 100
    {
        return Err(CrawlError::DataLoss(format!(
            "data_loss: expected {expected} bytes, got {}",
            body_bytes.len()
        )));
    }

    let body_bytes_vec = body_bytes.to_vec();
    let body = String::from_utf8_lossy(&body_bytes_vec).into_owned();

    Ok(HttpResponse {
        status,
        content_type,
        body,
        body_bytes: body_bytes_vec,
        headers,
    })
}

/// Build a `reqwest::Client` with the given configuration (redirect policy, timeout, cookies).
pub(crate) fn build_client(config: &CrawlConfig) -> Result<reqwest::Client, CrawlError> {
    let mut builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(config.request_timeout);

    if config.cookies_enabled {
        builder = builder.cookie_store(true);
    }

    builder
        .build()
        .map_err(|e| CrawlError::Other(format!("Failed to build HTTP client: {e}")))
}

/// Fetch a URL with retry logic based on configuration.
///
/// Retries on server errors and rate limiting if the corresponding status codes
/// are included in `config.retry_codes`. Uses exponential backoff between retries.
pub(crate) async fn fetch_with_retry(
    url: &str,
    config: &CrawlConfig,
    client: &reqwest::Client,
) -> Result<HttpResponse, CrawlError> {
    let retries = config.retry_count;
    let retry_codes = config.retry_codes.clone();

    let mut last_err = None;
    for attempt in 0..=retries {
        match http_fetch(url, config, client).await {
            Ok(resp) => return Ok(resp),
            Err(e) => {
                // Check if we should retry this error
                let should_retry = match &e {
                    CrawlError::ServerError(_) => {
                        retry_codes.contains(&503) || retry_codes.contains(&500)
                    }
                    CrawlError::RateLimited(_) => retry_codes.contains(&429),
                    _ => false,
                };
                if should_retry && attempt < retries {
                    // Exponential backoff
                    let delay = Duration::from_millis(100 * (1 << attempt));
                    tokio::time::sleep(delay).await;
                    last_err = Some(e);
                    continue;
                }
                return Err(e);
            }
        }
    }
    Err(last_err.unwrap_or_else(|| CrawlError::Other("retry exhausted".into())))
}

/// Extract cookies from `Set-Cookie` response headers.
pub(crate) fn extract_cookies(headers: &HeaderMap) -> Vec<CookieInfo> {
    let mut cookies = Vec::new();
    for val in headers.get_all("set-cookie") {
        if let Ok(s) = val.to_str() {
            let parts: Vec<&str> = s.split(';').collect();
            if let Some(nv) = parts.first()
                && let Some((name, value)) = nv.split_once('=')
            {
                let mut cookie = CookieInfo {
                    name: name.trim().to_owned(),
                    value: value.trim().to_owned(),
                    domain: None,
                    path: None,
                };
                for attr in &parts[1..] {
                    let attr = attr.trim().to_lowercase();
                    if let Some(d) = attr.strip_prefix("domain=") {
                        cookie.domain = Some(d.to_owned());
                    } else if let Some(p) = attr.strip_prefix("path=") {
                        cookie.path = Some(p.to_owned());
                    }
                }
                cookies.push(cookie);
            }
        }
    }
    cookies
}

/// Extract response metadata from HTTP headers.
pub(crate) fn extract_response_meta(headers: &HeaderMap) -> ResponseMeta {
    let get = |name: &str| -> Option<String> {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(String::from)
    };
    ResponseMeta {
        etag: get("etag"),
        last_modified: get("last-modified"),
        cache_control: get("cache-control"),
        server: get("server"),
        x_powered_by: get("x-powered-by"),
        content_language: get("content-language"),
        content_encoding: get("content-encoding"),
    }
}
