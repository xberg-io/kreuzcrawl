//! OpenAPI 3.1 schema generation for the crawlberg REST API.
//!
//! This module generates OpenAPI documentation from Rust types using utoipa.
//! The schema is available at the `/openapi.json` endpoint.

use utoipa::OpenApi;

/// OpenAPI documentation structure.
///
/// Defines all endpoints, request/response schemas, and examples
/// for the crawlberg web crawling and scraping API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Crawlberg API",
        version = env!("CARGO_PKG_VERSION"),
        description = "High-performance web crawling and scraping API. Firecrawl v1-compatible endpoints for scraping, crawling, URL discovery, batch operations, and document downloads.",
        contact(
            name = "Kreuzberg",
            url = "https://xberg.io"
        ),
        license(
            name = "MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
    ),
    paths(
        crate::api::handlers::health_handler,
        crate::api::handlers::version_handler,
        crate::api::handlers::scrape_handler,
        crate::api::handlers::crawl_handler,
        crate::api::handlers::crawl_status_handler,
        crate::api::handlers::crawl_cancel_handler,
        crate::api::handlers::map_handler,
        crate::api::handlers::batch_scrape_handler,
        crate::api::handlers::batch_status_handler,
        crate::api::handlers::download_handler,
    ),
    components(
        schemas(
            crate::api::types::ScrapeRequest,
            crate::api::types::CrawlRequest,
            crate::api::types::MapRequest,
            crate::api::types::BatchScrapeRequest,
            crate::api::types::DownloadRequest,
            crate::api::types::JobCreatedResponse,
            crate::api::types::JobStatusResponse,
            crate::api::types::HealthResponse,
            crate::api::types::VersionResponse,
            crate::api::types::ErrorBody,
        )
    ),
    tags(
        (name = "crawl", description = "Web crawling and URL mapping endpoints"),
        (name = "scrape", description = "Single and batch URL scraping endpoints"),
        (name = "system", description = "Health check and version endpoints"),
        (name = "download", description = "Document download endpoints"),
    )
)]
pub struct ApiDoc;

/// Generate OpenAPI JSON schema.
///
/// Returns the complete OpenAPI 3.1 specification as a JSON string.
#[allow(dead_code)]
pub fn openapi_json() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_schema_generation() {
        let schema = openapi_json();
        assert!(!schema.is_empty());
        assert!(schema.contains("Crawlberg API"));
        assert!(schema.contains("/health"));
        assert!(schema.contains("/v1/scrape"));
    }

    #[test]
    fn test_openapi_schema_valid_json() {
        let schema = openapi_json();
        let parsed: serde_json::Value = serde_json::from_str(&schema).expect("Invalid JSON");
        assert!(parsed.is_object());
        assert!(parsed["openapi"].is_string());
    }

    #[test]
    fn test_openapi_includes_all_endpoints() {
        let schema = openapi_json();
        assert!(schema.contains("/health"));
        assert!(schema.contains("/version"));
        assert!(schema.contains("/v1/scrape"));
        assert!(schema.contains("/v1/crawl"));
        assert!(schema.contains("/v1/crawl/{id}"));
        assert!(schema.contains("/v1/map"));
        assert!(schema.contains("/v1/batch/scrape"));
        assert!(schema.contains("/v1/batch/scrape/{id}"));
        assert!(schema.contains("/v1/download"));
    }

    #[test]
    fn test_openapi_includes_schemas() {
        let schema = openapi_json();
        assert!(schema.contains("ScrapeRequest"));
        assert!(schema.contains("CrawlRequest"));
        assert!(schema.contains("MapRequest"));
        assert!(schema.contains("BatchScrapeRequest"));
        assert!(schema.contains("DownloadRequest"));
        assert!(schema.contains("JobCreatedResponse"));
        assert!(schema.contains("JobStatusResponse"));
        assert!(schema.contains("HealthResponse"));
        assert!(schema.contains("VersionResponse"));
        assert!(schema.contains("ErrorBody"));
    }

    #[test]
    fn test_openapi_includes_tags() {
        let schema = openapi_json();
        assert!(schema.contains("\"crawl\""));
        assert!(schema.contains("\"scrape\""));
        assert!(schema.contains("\"system\""));
        assert!(schema.contains("\"download\""));
    }
}
