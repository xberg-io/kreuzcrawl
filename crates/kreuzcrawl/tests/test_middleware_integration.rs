//! Integration tests verifying that middleware headers actually reach the HTTP server.

use kreuzcrawl::{
    CachingMiddleware, CrawlConfig, CrawlEngine, NoopRateLimiter, UaRotationMiddleware,
};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_ua_rotation_reaches_server() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Hello</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    let config = CrawlConfig::default();
    let engine = CrawlEngine::builder()
        .config(config)
        .middleware(UaRotationMiddleware::new(vec!["TestBot/1.0".into()]))
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.scrape(&mock.uri()).await;
    assert!(result.is_ok(), "should succeed: {:?}", result.err());

    // Verify the User-Agent header the server actually received.
    let received = mock.received_requests().await.unwrap();
    assert!(!received.is_empty());
    let ua_values: Vec<_> = received[0]
        .headers
        .get_all("user-agent")
        .iter()
        .map(|v| v.to_str().unwrap().to_owned())
        .collect();
    assert!(
        ua_values.iter().any(|v| v == "TestBot/1.0"),
        "server should have received TestBot/1.0 as user-agent, got: {:?}",
        ua_values
    );
}

#[tokio::test]
async fn test_ua_rotation_cycles_through_agents() {
    let mock = MockServer::start().await;

    for i in 0..3 {
        Mock::given(method("GET"))
            .and(path(format!("/page{i}")))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(format!("<html><body>Page {i}</body></html>"))
                    .append_header("content-type", "text/html"),
            )
            .mount(&mock)
            .await;
    }

    let agents = vec!["AgentA/1.0".to_string(), "AgentB/2.0".to_string()];
    let config = CrawlConfig::default();
    let engine = CrawlEngine::builder()
        .config(config)
        .middleware(UaRotationMiddleware::new(agents))
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    for i in 0..3 {
        let url = format!("{}/page{i}", mock.uri());
        engine.scrape(&url).await.unwrap();
    }

    let received = mock.received_requests().await.unwrap();
    assert_eq!(received.len(), 3);

    // Collect the UA from each request.
    let uas: Vec<String> = received
        .iter()
        .map(|r| {
            r.headers
                .get_all("user-agent")
                .iter()
                .map(|v| v.to_str().unwrap().to_owned())
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect();

    // The middleware should cycle: AgentA, AgentB, AgentA.
    assert!(
        uas[0].contains("AgentA/1.0"),
        "first request should use AgentA, got: {}",
        uas[0]
    );
    assert!(
        uas[1].contains("AgentB/2.0"),
        "second request should use AgentB, got: {}",
        uas[1]
    );
    assert!(
        uas[2].contains("AgentA/1.0"),
        "third request should cycle back to AgentA, got: {}",
        uas[2]
    );
}

#[tokio::test]
async fn test_caching_middleware_sends_conditional_headers() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>Cached</body></html>")
                .append_header("content-type", "text/html")
                .append_header("etag", "\"abc123\""),
        )
        .expect(1..=3)
        .mount(&mock)
        .await;

    let config = CrawlConfig::default();
    let engine = CrawlEngine::builder()
        .config(config)
        .middleware(CachingMiddleware::new(100))
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    // First scrape populates the middleware cache.
    let result1 = engine.scrape(&mock.uri()).await.unwrap();
    assert_eq!(result1.status_code, 200);
    assert!(result1.html.contains("Cached"));

    // Second scrape -- CachingMiddleware should inject If-None-Match.
    let result2 = engine.scrape(&mock.uri()).await.unwrap();
    assert_eq!(result2.status_code, 200);

    // Verify at least one request was received by the mock.
    let received = mock.received_requests().await.unwrap();
    assert!(
        received.len() >= 2,
        "server should have received at least 2 requests, got {}",
        received.len()
    );
    // The second request should carry the If-None-Match header.
    let second_req = &received[1];
    let inm = second_req
        .headers
        .get("if-none-match")
        .expect("second request should have If-None-Match header");
    assert_eq!(inm.to_str().unwrap(), "\"abc123\"");
}
