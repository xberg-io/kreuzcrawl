//! Integration tests for WAF/bot-protection detection via actual HTTP responses.

use kreuzcrawl::{CrawlConfig, CrawlEngine, CrawlError, NoopRateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn assert_waf_blocked(body: &str, headers: Vec<(&str, &str)>) {
    let mock = MockServer::start().await;

    let mut resp = ResponseTemplate::new(403);
    for (k, v) in headers {
        resp = resp.append_header(k, v);
    }
    resp = resp.set_body_string(body);

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(resp)
        .mount(&mock)
        .await;

    let engine = CrawlEngine::builder()
        .config(CrawlConfig::default())
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.scrape(&mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::WafBlocked(_))),
        "expected WafBlocked, got: {:?}",
        result
    );
}

#[tokio::test]
async fn test_cloudflare_detection() {
    assert_waf_blocked(
        "<html>cf-browser-verification challenge-form</html>",
        vec![("content-type", "text/html"), ("server", "cloudflare")],
    )
    .await;
}

#[tokio::test]
async fn test_imperva_detection() {
    assert_waf_blocked(
        "<html>Powered by Incapsula _incap_ses_</html>",
        vec![("content-type", "text/html")],
    )
    .await;
}

#[tokio::test]
async fn test_datadome_detection() {
    assert_waf_blocked(
        "<html><script src=\"dd.js\"></script>datadome</html>",
        vec![("content-type", "text/html")],
    )
    .await;
}

#[tokio::test]
async fn test_akamai_detection() {
    assert_waf_blocked(
        "<html>Access Denied</html>",
        vec![("content-type", "text/html"), ("server", "AkamaiGHost")],
    )
    .await;
}

#[tokio::test]
async fn test_plain_403_is_not_waf() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_string("<html>Forbidden</html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let engine = CrawlEngine::builder()
        .config(CrawlConfig::default())
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.scrape(&mock.uri()).await;
    assert!(
        matches!(result, Err(CrawlError::Forbidden(_))),
        "plain 403 should be Forbidden, not WafBlocked: {:?}",
        result
    );
}
