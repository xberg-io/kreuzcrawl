//! Request and response types flowing through the Tower service stack.

use std::collections::HashMap;

// CrawlRequest is only used by the native Tower service stack; on wasm32 the
// Tower stack is not compiled so the type would be dead code.
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

/// HTTP request flowing through the Tower service stack.
///
/// Not available on `wasm32` targets — the Tower stack is native-only.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct CrawlRequest {
    pub url: String,
    pub headers: HashMap<String, String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl CrawlRequest {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            headers: HashMap::new(),
        }
    }

    pub fn domain(&self) -> Option<String> {
        Url::parse(&self.url)
            .ok()
            .and_then(|u| u.host_str().map(|s| s.to_owned()))
    }
}

/// HTTP response from the Tower service stack.
#[derive(Debug, Clone)]
pub struct CrawlResponse {
    pub status: u16,
    pub content_type: String,
    pub body: String,
    pub body_bytes: Vec<u8>,
    pub headers: HashMap<String, Vec<String>>,
}
