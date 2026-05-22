//! Base HTTP fetch service (innermost in the Tower stack).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use tower::Service;

use super::types::{CrawlRequest, CrawlResponse};
use crate::error::{CrawlError, classify_reqwest_error};
use crate::types::CrawlConfig;

/// Innermost Tower service that performs the actual HTTP fetch.
#[derive(Clone)]
pub struct HttpFetchService {
    client: reqwest::Client,
    config: Arc<CrawlConfig>,
}

impl HttpFetchService {
    pub fn new(client: reqwest::Client, config: CrawlConfig) -> Self {
        Self {
            client,
            config: Arc::new(config),
        }
    }
}

/// Check whether a `CrawlError` is retryable (server errors, rate limits, bad gateways).
fn is_retryable(e: &CrawlError) -> bool {
    matches!(
        e,
        CrawlError::ServerError(_) | CrawlError::RateLimited(_) | CrawlError::BadGateway(_)
    )
}

/// Perform a single HTTP fetch (no retry).
async fn do_fetch(
    client: &reqwest::Client,
    config: &CrawlConfig,
    req: &CrawlRequest,
) -> Result<CrawlResponse, CrawlError> {
    // Build reqwest request
    let mut http_req = client.get(&req.url);

    // Set user-agent (skip if request-level headers already provide one,
    // e.g. from the UaRotationLayer)
    if !req.headers.contains_key("user-agent") {
        if let Some(ref ua) = config.user_agent {
            http_req = http_req.header(reqwest::header::USER_AGENT, ua.as_str());
        } else {
            http_req = http_req.header(
                reqwest::header::USER_AGENT,
                concat!("kreuzcrawl/", env!("CARGO_PKG_VERSION")),
            );
        }
    }

    // Auth
    if let Some(ref auth) = config.auth {
        match auth {
            crate::types::AuthConfig::Basic { username, password } => {
                http_req = http_req.basic_auth(username, Some(password));
            }
            crate::types::AuthConfig::Bearer { token } => {
                http_req = http_req.bearer_auth(token);
            }
            crate::types::AuthConfig::Header { name, value } => {
                http_req = http_req.header(name.as_str(), value.as_str());
            }
        }
    }

    // Config custom headers
    for (k, v) in &config.custom_headers {
        http_req = http_req.header(k.as_str(), v.as_str());
    }

    // Request-level headers (from middleware layers)
    for (k, v) in &req.headers {
        http_req = http_req.header(k.as_str(), v.as_str());
    }

    // Send
    let resp = http_req.send().await.map_err(|e| classify_reqwest_error(&e))?;

    let status = resp.status().as_u16();
    let content_type = resp
        .headers()
        .get_all(reqwest::header::CONTENT_TYPE)
        .iter()
        .next_back()
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_owned();

    // Extract headers into HashMap<String, Vec<String>>
    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    for (name, value) in resp.headers().iter() {
        if let Ok(v) = value.to_str() {
            headers
                .entry(name.as_str().to_lowercase())
                .or_default()
                .push(v.to_string());
        }
    }

    // Check error status codes
    match status {
        401 => return Err(CrawlError::Unauthorized("unauthorized".into())),
        403 => {
            let server = headers
                .get("server")
                .and_then(|v| v.first())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            let body = resp.text().await.unwrap_or_default();
            if crate::http::is_waf_blocked(&server, &body, &headers) {
                let vendor = crate::http::detect_waf_vendor(&server, &body.to_lowercase());
                return Err(CrawlError::WafBlocked(format!("waf/blocked detected: {vendor}")));
            }
            return Err(CrawlError::Forbidden("forbidden".into()));
        }
        404 => return Err(CrawlError::NotFound(format!("not_found: {}", req.url))),
        408 => return Err(CrawlError::Timeout("timeout".into())),
        410 => return Err(CrawlError::Gone("gone".into())),
        429 => return Err(CrawlError::RateLimited("rate_limited".into())),
        500 => return Err(CrawlError::ServerError("server_error".into())),
        502 => return Err(CrawlError::BadGateway("bad_gateway".into())),
        503 => {
            return Err(CrawlError::ServerError("service unavailable".into()));
        }
        _ => {}
    }

    let body_bytes = resp.bytes().await.map_err(|e| {
        // Walk the error source chain to detect body/data-loss errors that
        // reqwest wraps in generic errors.
        let chain = crate::error::error_chain_string(&e);
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

    let body_vec = body_bytes.to_vec();

    // Content-length validation
    if let Some(expected) = headers
        .get("content-length")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<usize>().ok())
        && body_vec.len() < expected
        && expected - body_vec.len() > 100
    {
        return Err(CrawlError::DataLoss(format!(
            "data_loss: expected {} bytes, got {}",
            expected,
            body_vec.len()
        )));
    }

    let body = String::from_utf8_lossy(&body_vec).into_owned();

    // WAF challenge detection on 200 responses.
    // Some WAFs (AWS WAF, Akamai) return 200 with a challenge page instead of 403.
    // Check short bodies for WAF patterns to catch these false positives.
    #[cfg(not(target_arch = "wasm32"))]
    {
        let server = headers
            .get("server")
            .and_then(|v| v.first())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        if status == 200 && body.len() < 5000 && crate::http::is_waf_blocked(&server, &body, &headers) {
            return Err(CrawlError::WafBlocked(format!(
                "waf/blocked: {status} with challenge content"
            )));
        }
    }

    Ok(CrawlResponse {
        status,
        content_type,
        body,
        body_bytes: body_vec,
        headers,
    })
}

impl Service<CrawlRequest> for HttpFetchService {
    type Response = CrawlResponse;
    type Error = CrawlError;
    type Future = Pin<Box<dyn Future<Output = Result<CrawlResponse, CrawlError>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: CrawlRequest) -> Self::Future {
        let client = self.client.clone();
        let config = self.config.clone();

        Box::pin(async move {
            let retry_count = config.retry_count;
            let retry_codes = &config.retry_codes;

            for attempt in 0..=retry_count {
                match do_fetch(&client, &config, &req).await {
                    Ok(resp) => {
                        if retry_codes.contains(&resp.status) && attempt < retry_count {
                            tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
                            continue;
                        }
                        return Ok(resp);
                    }
                    Err(e) if is_retryable(&e) && attempt < retry_count => {
                        tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }

            // Should not reach here, but just in case
            Err(CrawlError::Other("retry exhausted".into()))
        })
    }
}
