//! Kreuzcrawl MCP server implementation.
//!
//! This module provides the main MCP server struct and startup functions.

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_router,
    transport::stdio,
};

use crate::engine::CrawlEngineBuilder;
use crate::types::CrawlConfig;

/// Validate that a URL is non-empty and uses http(s) scheme.
fn validate_url(url: &str) -> Result<(), rmcp::ErrorData> {
    if url.is_empty() {
        return Err(rmcp::ErrorData::invalid_params("url is required", None));
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(rmcp::ErrorData::invalid_params(
            "url must start with http:// or https://",
            None,
        ));
    }
    Ok(())
}

/// Parse the optional format parameter, defaulting to "markdown".
fn parse_format(format: &Option<String>) -> &str {
    match format.as_deref() {
        Some(f) if f.eq_ignore_ascii_case("json") => "json",
        _ => "markdown",
    }
}

/// Kreuzcrawl MCP server.
///
/// Provides web scraping, crawling, and mapping capabilities via MCP tools.
///
/// The server holds a default `CrawlConfig` that is used as the base for
/// all tool calls. Per-request parameters override specific fields.
pub struct KreuzcrawlMcp {
    tool_router: ToolRouter<KreuzcrawlMcp>,
    /// Default crawl configuration
    config: CrawlConfig,
}

impl Clone for KreuzcrawlMcp {
    fn clone(&self) -> Self {
        Self {
            tool_router: self.tool_router.clone(),
            config: self.config.clone(),
        }
    }
}

#[tool_router]
impl KreuzcrawlMcp {
    /// Create a new Kreuzcrawl MCP server instance with default config.
    pub fn new() -> Self {
        Self::with_config(CrawlConfig::default())
    }

    /// Create a new Kreuzcrawl MCP server instance with explicit config.
    ///
    /// # Arguments
    ///
    /// * `config` - Default crawl configuration for all tool calls
    pub fn with_config(config: CrawlConfig) -> Self {
        Self {
            tool_router: Self::tool_router(),
            config,
        }
    }

    // Engine is built per-request because each tool call may have different config
    // overrides (max_depth, max_pages, etc.). For shared state across calls,
    // consider caching engines by config hash in future iterations.

    /// Build a CrawlEngine from the stored config with parameter overrides applied.
    fn build_engine(
        &self,
        config: CrawlConfig,
    ) -> Result<crate::engine::CrawlEngine, rmcp::ErrorData> {
        CrawlEngineBuilder::new()
            .config(config)
            .build()
            .map_err(|e| {
                rmcp::ErrorData::internal_error(format!("Failed to build engine: {e}"), None)
            })
    }

    /// Scrape a single URL and extract content as markdown or JSON.
    ///
    /// Returns the page content, metadata, links, images, and feeds.
    /// Use `format: "json"` for structured output or `format: "markdown"` (default)
    /// for human-readable content.
    #[tool(
        description = "Scrape a single URL and extract content as markdown or JSON. Returns page content, metadata, links, and images.",
        annotations(title = "Scrape URL", read_only_hint = true, idempotent_hint = true)
    )]
    async fn scrape(
        &self,
        Parameters(params): Parameters<super::params::ScrapeParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_crawl_error;
        use super::format::{format_as_json, format_as_markdown};

        validate_url(&params.url)?;

        let config = self.config.clone();

        #[cfg(feature = "browser")]
        let config = {
            let mut config = config;
            if let Some(true) = params.use_browser {
                config.browser.mode = crate::types::BrowserMode::Always;
            }
            config
        };

        let engine = self.build_engine(config)?;
        let result = engine.scrape(&params.url).await.map_err(map_crawl_error)?;

        let response = if parse_format(&params.format) == "json" {
            format_as_json(&result)
        } else {
            format_as_markdown(&result)
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Crawl a website following links up to a configured depth.
    ///
    /// Starts from the given URL and follows links, returning content for all
    /// discovered pages. Use `stay_on_domain: true` to restrict to the same domain.
    #[tool(
        description = "Crawl a website following links. Returns content for all discovered pages up to max_depth/max_pages.",
        annotations(title = "Crawl Website", read_only_hint = true)
    )]
    async fn crawl(
        &self,
        Parameters(params): Parameters<super::params::CrawlParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_crawl_error;
        use super::format::{format_crawl_as_json, format_crawl_as_markdown};

        validate_url(&params.url)?;

        if let Some(depth) = params.max_depth
            && depth > 100
        {
            return Err(rmcp::ErrorData::invalid_params(
                "max_depth must be <= 100",
                None,
            ));
        }
        if let Some(pages) = params.max_pages
            && (pages == 0 || pages > 100_000)
        {
            return Err(rmcp::ErrorData::invalid_params(
                "max_pages must be 1..=100000",
                None,
            ));
        }

        let mut config = self.config.clone();
        if let Some(depth) = params.max_depth {
            config.max_depth = Some(depth);
        }
        if let Some(pages) = params.max_pages {
            config.max_pages = Some(pages);
        }
        if let Some(stay) = params.stay_on_domain {
            config.stay_on_domain = stay;
        }

        let engine = self.build_engine(config)?;
        let result = engine.crawl(&params.url).await.map_err(map_crawl_error)?;

        let response = if parse_format(&params.format) == "json" {
            format_crawl_as_json(&result)
        } else {
            format_crawl_as_markdown(&result)
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Discover all pages on a website via links and sitemaps.
    ///
    /// Returns a list of all discovered URLs with their metadata (last modified,
    /// change frequency, priority). Use `search` to filter URLs by substring.
    #[tool(
        description = "Discover all pages on a website via links and sitemaps. Returns a list of discovered URLs.",
        annotations(title = "Map Website", read_only_hint = true, idempotent_hint = true)
    )]
    async fn map(
        &self,
        Parameters(params): Parameters<super::params::MapParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_crawl_error;
        use super::format::format_map_result;

        validate_url(&params.url)?;

        let mut config = self.config.clone();
        if let Some(limit) = params.limit {
            config.map_limit = Some(limit);
        }
        if let Some(ref search) = params.search {
            config.map_search = Some(search.clone());
        }
        if let Some(robots) = params.respect_robots_txt {
            config.respect_robots_txt = robots;
        }

        let engine = self.build_engine(config)?;
        let result = engine.map(&params.url).await.map_err(map_crawl_error)?;

        let response = format_map_result(&result);
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Scrape multiple URLs concurrently.
    ///
    /// Efficiently processes multiple URLs in parallel and returns combined results.
    #[tool(
        description = "Scrape multiple URLs concurrently. Returns results for all URLs.",
        annotations(title = "Batch Scrape", read_only_hint = true)
    )]
    async fn batch_scrape(
        &self,
        Parameters(params): Parameters<super::params::BatchScrapeParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::format::{format_as_json, format_as_markdown};

        if params.urls.is_empty() {
            return Err(rmcp::ErrorData::invalid_params(
                "urls array must not be empty",
                None,
            ));
        }

        for url in &params.urls {
            validate_url(url)?;
        }

        let mut config = self.config.clone();
        if let Some(concurrency) = params.concurrency {
            config.max_concurrent = Some(concurrency);
        }

        let engine = self.build_engine(config)?;
        let url_refs: Vec<&str> = params.urls.iter().map(|s| s.as_str()).collect();
        let results = engine.batch_scrape(&url_refs).await;

        let is_json = parse_format(&params.format) == "json";

        let mut response = String::new();
        for (url, result) in &results {
            match result {
                Ok(scrape_result) => {
                    if is_json {
                        response.push_str(&format_as_json(scrape_result));
                    } else {
                        response.push_str(&format!("## {url}\n\n"));
                        response.push_str(&format_as_markdown(scrape_result));
                    }
                    response.push_str("\n\n---\n\n");
                }
                Err(e) => {
                    response.push_str(&format!("## {url}\n\n**Error:** {e}\n\n---\n\n"));
                }
            }
        }

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Download a document from a URL.
    ///
    /// Downloads the raw document bytes and returns metadata about the downloaded file
    /// including its MIME type, size, and content hash.
    #[tool(
        description = "Download a document from a URL. Returns metadata about the downloaded file.",
        annotations(title = "Download Document", read_only_hint = true)
    )]
    async fn download(
        &self,
        Parameters(params): Parameters<super::params::DownloadParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use super::errors::map_crawl_error;

        validate_url(&params.url)?;

        let mut config = self.config.clone();
        config.download_documents = true;
        if let Some(max_size) = params.max_size {
            config.document_max_size = Some(max_size);
        }

        let engine = self.build_engine(config)?;
        let result = engine.scrape(&params.url).await.map_err(map_crawl_error)?;

        let response = if let Some(ref doc) = result.downloaded_document {
            serde_json::json!({
                "url": doc.url,
                "mime_type": doc.mime_type,
                "size": doc.size,
                "filename": doc.filename,
                "content_hash": doc.content_hash,
            })
            .to_string()
        } else {
            // Fell back to HTML scrape — return the page metadata instead
            serde_json::json!({
                "url": params.url,
                "content_type": result.content_type,
                "status_code": result.status_code,
                "body_size": result.body_size,
                "note": "URL returned HTML content, not a downloadable document"
            })
            .to_string()
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Capture a screenshot of a URL (requires browser feature).
    #[tool(
        description = "Capture a screenshot of a URL (requires browser feature).",
        annotations(title = "Screenshot", read_only_hint = true)
    )]
    async fn screenshot(
        &self,
        Parameters(params): Parameters<super::params::ScreenshotParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        validate_url(&params.url)?;

        Ok(CallToolResult::success(vec![Content::text(
            "Screenshot tool is not yet implemented. It requires the 'browser' feature.",
        )]))
    }

    /// Execute browser actions on a page (requires interact feature).
    #[tool(
        description = "Execute browser actions on a page (requires interact feature).",
        annotations(title = "Interact", read_only_hint = true)
    )]
    async fn interact(
        &self,
        Parameters(params): Parameters<super::params::InteractParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        validate_url(&params.url)?;

        Ok(CallToolResult::success(vec![Content::text(
            "Interact tool is not yet implemented. It requires the 'interact' feature.",
        )]))
    }

    /// AI-driven research across multiple pages (requires ai feature).
    #[tool(
        description = "AI-driven research across multiple pages (requires ai feature).",
        annotations(title = "Research", read_only_hint = true)
    )]
    async fn research(
        &self,
        Parameters(_params): Parameters<super::params::ResearchParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        Ok(CallToolResult::success(vec![Content::text(
            "Research tool is not yet implemented. It requires the 'ai' feature.",
        )]))
    }

    /// Check the status of a crawl job.
    #[tool(
        description = "Check the status of a crawl job.",
        annotations(title = "Crawl Status", read_only_hint = true)
    )]
    async fn crawl_status(
        &self,
        Parameters(_params): Parameters<super::params::CrawlStatusParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        Ok(CallToolResult::success(vec![Content::text(
            "Crawl status tool is not yet implemented. No job registry exists in the current MCP context.",
        )]))
    }

    /// Get the current kreuzcrawl version.
    #[tool(
        description = "Get the current kreuzcrawl library version.",
        annotations(title = "Get Version", read_only_hint = true, idempotent_hint = true)
    )]
    fn get_version(
        &self,
        Parameters(_): Parameters<super::params::EmptyParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let response = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize version: {e}\"}}")),
        )]))
    }
}

impl ServerHandler for KreuzcrawlMcp {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability::default());

        let server_info = Implementation::new("kreuzcrawl-mcp", env!("CARGO_PKG_VERSION"))
            .with_title("Kreuzcrawl Web Crawling MCP Server")
            .with_description(
                "Web crawling and scraping library for extracting content from websites. \
                 Supports single-page scraping, multi-page crawling, site mapping, and batch operations.",
            )
            .with_website_url("https://github.com/kreuzberg-dev/kreuzcrawl");

        InitializeResult::new(capabilities)
            .with_server_info(server_info)
            .with_instructions(
                "Scrape, crawl, and map websites. Use 'scrape' for single pages, 'crawl' for \
                 following links across a site, 'map' for discovering all URLs, and 'batch_scrape' \
                 for processing multiple URLs concurrently. Use format: 'json' for structured output \
                 or 'markdown' (default) for human-readable content.",
            )
    }
}

impl Default for KreuzcrawlMcp {
    fn default() -> Self {
        Self::new()
    }
}

/// Start the Kreuzcrawl MCP server with default configuration.
///
/// This function initializes and runs the MCP server using stdio transport.
/// It will block until the server is shut down.
///
/// # Errors
///
/// Returns an error if the server fails to start or encounters a fatal error.
///
/// # Example
///
/// ```rust,no_run
/// use kreuzcrawl::mcp::start_mcp_server;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     start_mcp_server().await?;
///     Ok(())
/// }
/// ```
pub async fn start_mcp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_mcp_server_with_config(CrawlConfig::default()).await
}

/// Start MCP server with custom crawl configuration.
///
/// This variant allows specifying a custom crawl configuration
/// instead of using defaults.
pub async fn start_mcp_server_with_config(
    config: CrawlConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = KreuzcrawlMcp::with_config(config).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
