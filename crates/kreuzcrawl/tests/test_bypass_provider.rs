//! Verifies that BypassProvider trait can be implemented and attached to CrawlConfig.
//!
//! Full integration tests with real HTTP responses are deferred until
//! kreuzberg-cloud implements the vendor adapters and can test the end-to-end
//! fetch + extraction flow.

#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use kreuzcrawl::{BypassProvider, CrawlConfig, CrawlError, DynBypassProvider, HttpResponse};

/// Test provider that can be instantiated (actual HTTP response construction
/// is out of scope — HttpResponse has pub(crate) fields).
#[derive(Debug)]
struct TestProvider;

#[async_trait]
impl BypassProvider for TestProvider {
    async fn fetch(&self, _url: &str) -> Result<HttpResponse, CrawlError> {
        Err(CrawlError::InvalidConfig(
            "test provider does not implement fetch".into(),
        ))
    }

    fn vendor_name(&self) -> &'static str {
        "test"
    }
}

#[test]
fn bypass_provider_can_be_attached_to_crawl_config() {
    // Verify that the BypassProvider trait can be implemented, wrapped in
    // Arc<dyn ...>, and attached to CrawlConfig without compilation errors.
    let provider: DynBypassProvider = Arc::new(TestProvider);

    let config = CrawlConfig {
        bypass: Some(provider),
        ..Default::default()
    };

    assert!(config.bypass.is_some(), "bypass provider should be attached");
    assert_eq!(
        config.bypass.as_ref().unwrap().vendor_name(),
        "test",
        "vendor_name should be accessible"
    );
}
