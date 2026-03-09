//! Error types for the kreuzcrawl crate.

use thiserror::Error;

/// Errors that can occur during crawling, scraping, or mapping operations.
#[derive(Debug, Error)]
pub enum CrawlError {
    /// The requested page was not found (HTTP 404).
    #[error("not_found: {0}")]
    NotFound(String),
    /// The request was unauthorized (HTTP 401).
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// The request was forbidden (HTTP 403).
    #[error("forbidden: {0}")]
    Forbidden(String),
    /// The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators).
    #[error("forbidden: waf/blocked: {0}")]
    WafBlocked(String),
    /// The request timed out.
    #[error("timeout: {0}")]
    Timeout(String),
    /// The request was rate-limited (HTTP 429).
    #[error("rate_limited: {0}")]
    RateLimited(String),
    /// A server error occurred (HTTP 5xx).
    #[error("server_error: {0}")]
    ServerError(String),
    /// A bad gateway error occurred (HTTP 502).
    #[error("bad_gateway: {0}")]
    BadGateway(String),
    /// The resource is permanently gone (HTTP 410).
    #[error("gone: {0}")]
    Gone(String),
    /// A connection error occurred.
    #[error("connection: {0}")]
    Connection(String),
    /// A DNS resolution error occurred.
    #[error("dns: {0}")]
    Dns(String),
    /// An SSL/TLS error occurred.
    #[error("ssl: {0}")]
    Ssl(String),
    /// Data was lost or truncated during transfer.
    #[error("data_loss: {0}")]
    DataLoss(String),
    /// A redirect loop was detected.
    #[error("redirect_loop: {0}")]
    RedirectLoop(String),
    /// Too many redirects were followed.
    #[error("too_many_redirects: {0}")]
    TooManyRedirects(String),
    /// An unclassified error occurred.
    #[error("other: {0}")]
    Other(String),
}

/// Collect the full error source chain into a single lowercase string for keyword matching.
fn error_chain_string(e: &reqwest::Error) -> String {
    let mut parts = vec![e.to_string()];
    let mut current: &dyn std::error::Error = e;
    while let Some(src) = current.source() {
        parts.push(src.to_string());
        current = src;
    }
    parts.join(" | ").to_lowercase()
}

/// Classify a `reqwest::Error` into the appropriate `CrawlError` variant.
///
/// Walks the full error source chain to detect DNS, SSL/TLS, and other
/// network-level errors that reqwest wraps in generic connect errors.
pub(crate) fn classify_reqwest_error(e: &reqwest::Error) -> CrawlError {
    let chain = error_chain_string(e);
    if e.is_timeout() || chain.contains("timed out") || chain.contains("timeout") {
        CrawlError::Timeout(format!("timeout: {e}"))
    } else if chain.contains("dns") || chain.contains("resolve") || chain.contains("lookup") {
        CrawlError::Dns(format!("dns: {e}"))
    } else if chain.contains("ssl")
        || chain.contains("tls")
        || chain.contains("certificate")
        || chain.contains("record overflow")
        || chain.contains("handshake")
        || chain.contains("corrupt message")
        || chain.contains("alertdescription")
        || chain.contains("invalidcontenttype")
    {
        CrawlError::Ssl(format!("ssl: {e}"))
    } else if e.is_connect() || chain.contains("connection") || chain.contains("connect") {
        CrawlError::Connection(format!("connection: {e}"))
    } else if e.is_body()
        || chain.contains("content-length")
        || chain.contains("truncat")
        || chain.contains("incomplete")
    {
        CrawlError::DataLoss(format!("data_loss: {e}"))
    } else {
        CrawlError::Other(format!("other: {e}"))
    }
}
