//! HTTP fetching with redirect handling, retry logic, and cookie extraction.

use std::time::Duration;

use reqwest::header::{CONTENT_TYPE, HeaderMap, USER_AGENT};

use crate::error::{CrawlError, classify_reqwest_error, error_chain_string};
#[cfg(not(target_arch = "wasm32"))]
use crate::types::CookieInfo;
use crate::types::{AuthConfig, CrawlConfig, ResponseMeta};

/// Browser-specific extras attached to an `HttpResponse` produced by the native
/// browser backend. Populated when `browser_used` is true.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct BrowserExtras {
    pub eval_result: Option<serde_json::Value>,
    pub network_events: Vec<crate::types::ResponseMeta>,
    pub cookies: Vec<crate::types::CookieInfo>,
}

/// An HTTP response with status, headers, and body content.
pub struct HttpResponse {
    pub status: u16,
    pub content_type: String,
    pub body: String,
    pub body_bytes: Vec<u8>,
    #[allow(dead_code)]
    pub headers: std::collections::HashMap<String, Vec<String>>,
    #[allow(dead_code)]
    pub browser_extras: Option<BrowserExtras>,
    /// The URL of the final response after any transparent redirect following.
    ///
    /// On native targets reqwest uses `Policy::none()` so this always equals
    /// the request URL (redirects are handled manually by `follow_redirects`).
    /// On wasm targets the browser's `fetch` follows redirects transparently
    /// and `reqwest::Response::url()` returns the post-redirect URL — which is
    /// what the wasm scrape path needs to populate `ScrapeResult::final_url`.
    // `dead_code` fires on native because the wasm scrape path (the only
    // consumer) is gated on `#[cfg(target_arch = "wasm32")]`.
    #[allow(dead_code)]
    pub final_url: String,
}

/// Perform a single HTTP GET request with the given configuration.
///
/// Handles user-agent, authentication, custom headers, error status codes,
/// and content-length validation.
pub(crate) async fn http_fetch(
    url: &str,
    config: &CrawlConfig,
    extra_headers: &std::collections::HashMap<String, String>,
    client: &reqwest::Client,
) -> Result<HttpResponse, CrawlError> {
    let mut req = client.get(url);

    // Set user-agent
    if let Some(ref ua) = config.user_agent {
        req = req.header(USER_AGENT, ua.as_str());
    } else {
        req = req.header(USER_AGENT, concat!("kreuzcrawl/", env!("CARGO_PKG_VERSION")));
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
        Some(AuthConfig::Header { ref name, ref value }) => {
            req = req.header(name.as_str(), value.as_str());
        }
        None => {}
    }

    // Custom headers
    for (k, v) in &config.custom_headers {
        req = req.header(k.as_str(), v.as_str());
    }

    // Apply middleware-provided headers (override config headers)
    for (k, v) in extra_headers {
        req = req.header(k.as_str(), v.as_str());
    }

    let resp = req.send().await.map_err(|e| classify_reqwest_error(&e))?;

    let status = resp.status().as_u16();
    // Capture the final URL before consuming the response. On native targets
    // (redirect Policy::none) this equals the request URL. On wasm targets
    // the browser follows redirects transparently, so this is the real
    // post-redirect URL.
    let final_url = resp.url().to_string();

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
            let server_lower = headers
                .get("server")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_lowercase();
            let body = resp.text().await.unwrap_or_default();
            if is_waf_blocked_headermap(&server_lower, &body, &headers) {
                let body_lower = body.to_lowercase();
                return Err(CrawlError::WafBlocked(format!(
                    "waf/blocked detected: {}",
                    detect_waf_vendor(&server_lower, &body_lower)
                )));
            }
            return Err(CrawlError::Forbidden("forbidden".into()));
        }
        404 => return Err(CrawlError::NotFound(format!("not_found: {url}"))),
        408 => return Err(CrawlError::Timeout("timeout: request timed out".into())),
        410 => return Err(CrawlError::Gone("gone".into())),
        429 => return Err(CrawlError::RateLimited("rate_limited".into())),
        500 => return Err(CrawlError::ServerError("server_error".into())),
        502 => return Err(CrawlError::BadGateway("bad_gateway".into())),
        503 => {
            return Err(CrawlError::ServerError("server_error: service unavailable".into()));
        }
        _ => {}
    }

    // 2xx interstitial detection (header-only): modern Cloudflare /
    // DataDome / PerimeterX serve their JS challenge with 200 OK, not 403.
    // Without this check the challenge page body is fed downstream as if
    // it were real content. Header signals (`x-datadome`,
    // `x-amzn-waf-action`, `x-px-*`, `x-sucuri-id`) are unambiguous WAF
    // markers regardless of status code — when a WAF stamps its own header
    // there is no false-positive risk. Body-fingerprint detection on 2xx
    // happens later (after the body is read) with a body-size guard to
    // avoid matching legitimate pages that mention vendor names.
    if (200..300).contains(&status) && headers_only_waf_match(&headers) {
        let server_lower = headers
            .get("server")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        // We need the body to identify the vendor precisely; read it now
        // (the body-fingerprint check below would re-read anyway).
        let body = resp.text().await.unwrap_or_default();
        let body_lower = body.to_lowercase();
        return Err(CrawlError::WafBlocked(format!(
            "waf/blocked detected on 2xx (header): {}",
            detect_waf_vendor(&server_lower, &body_lower)
        )));
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
        let is_body_error = chain.contains("content-length")
            || chain.contains("truncat")
            || chain.contains("incomplete")
            || chain.contains("end of file")
            || chain.contains("body error")
            || chain.contains("body from connection")
            || chain.contains("decoding response body")
            || chain.contains("error decoding");
        #[cfg(not(target_arch = "wasm32"))]
        let is_body_error = is_body_error || e.is_body();
        if is_body_error {
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

    // 2xx interstitial detection (body-fingerprint): if a 2xx response has
    // a small body containing a high-confidence vendor JS fingerprint,
    // it's almost certainly a challenge page rather than real content.
    // Real content pages are overwhelmingly larger than CHALLENGE_BODY_LIMIT
    // and don't contain these specific markers in a script src or inline.
    if (200..300).contains(&status) && body_bytes_vec.len() <= CHALLENGE_BODY_LIMIT && is_challenge_interstitial(&body)
    {
        let server_lower = headers
            .get("server")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        let body_lower = body.to_lowercase();
        return Err(CrawlError::WafBlocked(format!(
            "waf/blocked detected on 2xx (body): {}",
            detect_waf_vendor(&server_lower, &body_lower)
        )));
    }

    // Extract headers into HashMap<String, Vec<String>>
    let mut headers_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(v) = value.to_str() {
            headers_map
                .entry(name.as_str().to_lowercase())
                .or_default()
                .push(v.to_string());
        }
    }

    Ok(HttpResponse {
        status,
        content_type,
        body,
        body_bytes: body_bytes_vec,
        headers: headers_map,
        browser_extras: None,
        final_url,
    })
}

/// Build a `reqwest::Client` with the given configuration (redirect policy, timeout, cookies, proxy).
#[cfg_attr(target_arch = "wasm32", allow(unused_variables, unused_mut))]
pub(crate) fn build_client(config: &CrawlConfig) -> Result<reqwest::Client, CrawlError> {
    let mut builder = reqwest::Client::builder();

    #[cfg(not(target_arch = "wasm32"))]
    {
        builder = builder
            .redirect(reqwest::redirect::Policy::none())
            .timeout(config.request_timeout);
    }

    #[cfg(not(target_arch = "wasm32"))]
    if config.cookies_enabled {
        builder = builder.cookie_store(true);
    }

    // Proxy support (not available on wasm)
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(ref proxy_config) = config.proxy {
        let mut proxy = reqwest::Proxy::all(&proxy_config.url)
            .map_err(|e| CrawlError::InvalidConfig(format!("invalid proxy URL: {e}")))?;
        if let (Some(user), Some(pass)) = (&proxy_config.username, &proxy_config.password) {
            proxy = proxy.basic_auth(user, pass);
        }
        builder = builder.proxy(proxy);
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
    extra_headers: &std::collections::HashMap<String, String>,
    client: &reqwest::Client,
) -> Result<HttpResponse, CrawlError> {
    let retries = config.retry_count;
    let retry_codes = config.retry_codes.clone();

    let mut last_err = None;
    for attempt in 0..=retries {
        match http_fetch(url, config, extra_headers, client).await {
            Ok(resp) => return Ok(resp),
            Err(e) => {
                // Check if we should retry this error
                let should_retry = match &e {
                    CrawlError::ServerError(_) => retry_codes.contains(&503) || retry_codes.contains(&500),
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

/// Extract cookies from a `HashMap<String, Vec<String>>` of response headers.
///
/// Looks for the `"set-cookie"` key and parses each value as an individual
/// Set-Cookie header, preserving all cookies from the response.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn extract_cookies_from_hashmap(
    headers: &std::collections::HashMap<String, Vec<String>>,
) -> Vec<CookieInfo> {
    let mut cookies = Vec::new();
    if let Some(values) = headers.get("set-cookie") {
        for raw in values {
            let parts: Vec<&str> = raw.split(';').collect();
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

/// Extract response metadata from a `HashMap<String, Vec<String>>` of headers.
pub(crate) fn extract_response_meta_from_hashmap(
    headers: &std::collections::HashMap<String, Vec<String>>,
) -> ResponseMeta {
    ResponseMeta {
        etag: headers.get("etag").and_then(|v| v.first().cloned()),
        last_modified: headers.get("last-modified").and_then(|v| v.first().cloned()),
        cache_control: headers.get("cache-control").and_then(|v| v.first().cloned()),
        server: headers.get("server").and_then(|v| v.first().cloned()),
        x_powered_by: headers.get("x-powered-by").and_then(|v| v.first().cloned()),
        content_language: headers.get("content-language").and_then(|v| v.first().cloned()),
        content_encoding: headers.get("content-encoding").and_then(|v| v.first().cloned()),
    }
}

/// Maximum body size (bytes) that a 2xx response can have and still be
/// classified as a WAF interstitial. Real content pages overwhelmingly
/// exceed this; challenge interstitials are tiny by design.
const CHALLENGE_BODY_LIMIT: usize = 100 * 1024;

/// Header-only WAF signature. When any WAF stamps its own header on the
/// response, the verdict is unambiguous regardless of status code.
fn headers_only_waf_match(headers: &HeaderMap) -> bool {
    headers.contains_key("x-sucuri-id")
        || headers.contains_key("x-datadome")
        || headers.contains_key("x-amzn-waf-action")
        || headers.keys().any(|k| k.as_str().starts_with("x-px-"))
}

/// High-confidence interstitial body fingerprints. These markers are
/// vendor-specific JS hooks / script sources that legitimate content
/// pages do not embed. Generic phrases like "verifying your connection"
/// are deliberately excluded — they appear in legitimate support docs.
fn is_challenge_interstitial(body: &str) -> bool {
    let lower = body.to_lowercase();
    // Cloudflare interstitial signatures
    lower.contains("cf-chl-")
        || lower.contains("__cf_chl_")
        || lower.contains("_cf_chl_opt")
        || lower.contains("cf-browser-verification")
        || lower.contains("challenge-platform")
        || lower.contains("/cdn-cgi/challenge-platform/")
        // DataDome interstitial signatures
        || lower.contains("https://js.datadome.co/")
        || lower.contains("captcha-delivery.com")
        || lower.contains("dd.js")
        || lower.contains("ddjskey")
        // PerimeterX interstitial signatures
        || lower.contains("/_px/captcha")
        || lower.contains("px-captcha")
        // Imperva / Incapsula
        || lower.contains("_incap_ses_")
        // AWS WAF
        || lower.contains("awswaf.com/token")
        // Generic Cloudflare turnstile widget
        || lower.contains("challenges.cloudflare.com/turnstile/")
}

fn waf_pattern_match(server_lower: &str, body_lower: &str) -> bool {
    server_lower.contains("cloudflare")
        || body_lower.contains("cf-browser-verification")
        || body_lower.contains("challenge-form")
        || body_lower.contains("cf-chl-")
        || server_lower.contains("akamaighost")
        || body_lower.contains("awselb")
        || body_lower.contains("x-amzn-waf")
        || body_lower.contains("request blocked")
        || server_lower.contains("incapsula")
        || body_lower.contains("incapsula")
        || body_lower.contains("_incap_ses_")
        || body_lower.contains("datadome")
        || body_lower.contains("dd.js")
        || body_lower.contains("captcha-delivery.com")
        || body_lower.contains("ddjskey")
        || body_lower.contains("perimeterx")
        || body_lower.contains("px-captcha")
        || body_lower.contains("sucuri")
        || body_lower.contains("x-sucuri-id")
        || server_lower.contains("big-ip")
        || body_lower.contains("bigipserver")
        // AWS CloudFront + WAF
        || server_lower.contains("cloudfront")
        || body_lower.contains("awswaf.com")
        || body_lower.contains("verifying your connection")
        || body_lower.contains("challenge.js")
        // Additional common patterns
        || body_lower.contains("please verify you are human")
        || body_lower.contains("checking your browser")
        || body_lower.contains("just a moment") // Cloudflare "Just a moment..."
}

fn is_waf_blocked_headermap(server_lower: &str, body: &str, headers: &HeaderMap) -> bool {
    let body_lower = body.to_lowercase();
    if waf_pattern_match(server_lower, &body_lower) {
        return true;
    }
    headers.contains_key("x-sucuri-id")
        || headers.contains_key("x-datadome")
        || headers.contains_key("x-amzn-waf-action")
        || headers.keys().any(|k| k.as_str().starts_with("x-px-"))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn is_waf_blocked(
    server: &str,
    body: &str,
    headers: &std::collections::HashMap<String, Vec<String>>,
) -> bool {
    let body_lower = body.to_lowercase();
    let server_lower = server.to_lowercase();
    if waf_pattern_match(&server_lower, &body_lower) {
        return true;
    }
    headers.contains_key("x-sucuri-id")
        || headers.contains_key("x-datadome")
        || headers.contains_key("x-amzn-waf-action")
        || headers.keys().any(|k| k.starts_with("x-px-"))
}

/// Identify the WAF vendor from server and body content.
pub(crate) fn detect_waf_vendor(server: &str, body: &str) -> &'static str {
    if server.contains("cloudflare")
        || body.contains("cf-browser-verification")
        || body.contains("challenge-form")
        || body.contains("cf-chl-")
    {
        return "cloudflare";
    }
    if server.contains("akamaighost") {
        return "akamai";
    }
    if body.contains("incapsula") || body.contains("_incap_ses_") {
        return "imperva";
    }
    if body.contains("datadome") || body.contains("captcha-delivery.com") || body.contains("ddjskey") {
        return "datadome";
    }
    if body.contains("perimeterx") || body.contains("px-captcha") {
        return "perimeterx";
    }
    if body.contains("sucuri") {
        return "sucuri";
    }
    if server.contains("big-ip") {
        return "f5";
    }
    if body.contains("awselb") || body.contains("x-amzn-waf") {
        return "aws-waf";
    }
    "unknown"
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// `http_fetch` must populate `final_url` from the reqwest response URL.
    ///
    /// On native targets (Policy::none) this equals the request URL because
    /// redirects are not followed transparently.  The test verifies the field
    /// is set to a non-empty value matching the requested URL — confirming
    /// the plumbing that the wasm path relies on to capture the post-redirect
    /// URL is in place.
    ///
    /// Note: the wasm-specific transparent-redirect behaviour (browser `fetch`
    /// following 3xx and returning the final URL via `response.url()`) cannot
    /// be exercised under `cargo test` because it requires a wasm32 target and
    /// a real browser runtime.  The build-time check (`cargo build --target
    /// wasm32-unknown-unknown`) verifies the changed code path compiles
    /// correctly for wasm.
    #[tokio::test]
    async fn http_fetch_populates_final_url() {
        let mock = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/page"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<html><body>Hello</body></html>")
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;

        let url = format!("{}/page", mock.uri());
        let config = CrawlConfig::default();
        let client = build_client(&config).expect("client must build");
        let resp = http_fetch(&url, &config, &std::collections::HashMap::new(), &client)
            .await
            .expect("http_fetch must succeed");

        assert!(
            !resp.final_url.is_empty(),
            "final_url must not be empty after a successful fetch"
        );
        assert!(
            resp.final_url.contains("/page"),
            "final_url must contain the requested path, got: {}",
            resp.final_url
        );
    }
}
