//! Model Context Protocol (MCP) server implementation.
//!
//! Provides an MCP server that exposes kreuzcrawl's web crawling and scraping
//! capabilities as MCP tools for integration with AI assistants.
//!
//! # Implemented Tools
//!
//! - **scrape**: Scrape a single URL and extract content as markdown or JSON
//! - **crawl**: Crawl a website following links up to a configured depth
//! - **map**: Discover all pages on a website via links and sitemaps
//! - **batch_scrape**: Scrape multiple URLs concurrently
//! - **download**: Download a document from a URL
//! - **interact**: Execute browser actions on a page
//! - **get_version**: Get the current kreuzcrawl library version
//!
//! # Planned Tools (stubs registered, not yet implemented)
//!
//! - **screenshot**: Capture a screenshot of a URL (requires browser feature)
//! - **research**: AI-driven research across multiple pages (requires ai feature)
//! - **crawl_status**: Check the status of a crawl job (requires job registry)
//!
//! # Example
//!
//! ```rust,no_run
//! use kreuzcrawl::start_mcp_server;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     start_mcp_server().await?;
//!     Ok(())
//! }
//! ```

mod errors;
pub mod format;
mod params;
mod server;

pub use server::{start_mcp_server, start_mcp_server_with_config};
