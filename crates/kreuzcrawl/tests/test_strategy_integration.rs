//! Integration tests for crawl strategies: BFS vs DFS produce different visit orders.

use kreuzcrawl::{BfsStrategy, CrawlConfig, CrawlEngine, DfsStrategy, NoopRateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_tree_site() -> MockServer {
    let mock = MockServer::start().await;

    // Root links to /a and /b
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(
                    "<html><body><a href=\"/a\">A</a><a href=\"/b\">B</a></body></html>",
                )
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    // /a links to /a/1
    Mock::given(method("GET"))
        .and(path("/a"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/a/1\">A1</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    // /b links to /b/1
    Mock::given(method("GET"))
        .and(path("/b"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><a href=\"/b/1\">B1</a></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/a/1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>A1 leaf</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/b/1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>B1 leaf</body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&mock)
        .await;

    mock
}

#[tokio::test]
async fn test_bfs_crawls_all_pages() {
    let mock = setup_tree_site().await;

    let config = CrawlConfig {
        max_depth: Some(2),
        max_concurrent: Some(1),
        ..Default::default()
    };

    let engine = CrawlEngine::builder()
        .config(config)
        .strategy(BfsStrategy)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.crawl(&mock.uri()).await.unwrap();
    assert_eq!(
        result.pages.len(),
        5,
        "BFS should crawl all 5 pages, got: {:?}",
        result.pages.iter().map(|p| &p.url).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_dfs_crawls_all_pages() {
    let mock = setup_tree_site().await;

    let config = CrawlConfig {
        max_depth: Some(2),
        max_concurrent: Some(1),
        ..Default::default()
    };

    let engine = CrawlEngine::builder()
        .config(config)
        .strategy(DfsStrategy)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let result = engine.crawl(&mock.uri()).await.unwrap();
    assert_eq!(
        result.pages.len(),
        5,
        "DFS should crawl all 5 pages, got: {:?}",
        result.pages.iter().map(|p| &p.url).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_bfs_and_dfs_produce_different_orders() {
    let mock = setup_tree_site().await;

    let config = CrawlConfig {
        max_depth: Some(2),
        max_concurrent: Some(1),
        ..Default::default()
    };

    let bfs_engine = CrawlEngine::builder()
        .config(config.clone())
        .strategy(BfsStrategy)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let dfs_engine = CrawlEngine::builder()
        .config(config)
        .strategy(DfsStrategy)
        .rate_limiter(NoopRateLimiter)
        .build()
        .unwrap();

    let bfs = bfs_engine.crawl(&mock.uri()).await.unwrap();
    let dfs = dfs_engine.crawl(&mock.uri()).await.unwrap();

    let bfs_paths: Vec<String> = bfs
        .pages
        .iter()
        .map(|p| url::Url::parse(&p.url).unwrap().path().to_owned())
        .collect();
    let dfs_paths: Vec<String> = dfs
        .pages
        .iter()
        .map(|p| url::Url::parse(&p.url).unwrap().path().to_owned())
        .collect();

    assert_eq!(bfs.pages.len(), 5, "BFS should crawl all 5 pages");
    assert_eq!(dfs.pages.len(), 5, "DFS should crawl all 5 pages");

    // Both visit the same pages but in different order.
    let mut bfs_sorted = bfs_paths.clone();
    bfs_sorted.sort();
    let mut dfs_sorted = dfs_paths.clone();
    dfs_sorted.sort();
    assert_eq!(
        bfs_sorted, dfs_sorted,
        "Both should visit the same set of pages"
    );

    assert_ne!(
        bfs_paths, dfs_paths,
        "BFS and DFS should produce different orderings. BFS: {:?}, DFS: {:?}",
        bfs_paths, dfs_paths
    );
}
