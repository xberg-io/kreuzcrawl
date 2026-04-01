// Hand-written tests for sitemap scenarios requiring binary file loading.
// The E2E generator's helpers use String-based body loading which doesn't support
// binary files (gzip). Corresponding fixtures have `skip` directives.

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_sitemap_compressed_gzip() {
    // Parses a gzip-compressed sitemap file
    let server = MockServer::start().await;

    let gz_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("fixtures/responses/xml/sitemap_gzip.xml.gz");
    let gz_bytes = std::fs::read(&gz_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", gz_path.display()));

    let response = ResponseTemplate::new(200)
        .set_body_bytes(gz_bytes)
        .append_header("content-type", "application/x-gzip");

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&server)
        .await;

    let config = kreuzcrawl::CrawlConfig {
        respect_robots_txt: false,
        ..Default::default()
    };

    let engine = kreuzcrawl::CrawlEngine::builder()
        .config(config.clone())
        .build()
        .unwrap();
    let result = engine.map(&server.uri()).await;
    let result = result.expect("map should succeed");
    // The map function handles gzip decompression internally via fetch_sitemap_tree
    assert_eq!(result.urls.len(), 3);
}
