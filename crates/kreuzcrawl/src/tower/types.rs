//! Request and response types flowing through the Tower service stack.

use std::collections::HashMap;
use url::Url;

/// HTTP request flowing through the Tower service stack.
#[derive(Debug, Clone)]
pub struct CrawlRequest {
    pub url: String,
    pub headers: HashMap<String, String>,
}

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
