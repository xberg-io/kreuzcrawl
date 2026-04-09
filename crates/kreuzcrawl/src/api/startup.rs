//! API server startup functions.

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use crate::engine::CrawlEngine;
use crate::error::CrawlError;
use crate::types::CrawlConfig;

use super::router::create_router;

/// Start the API server with a pre-built engine.
///
/// # Arguments
///
/// * `host` - IP address to bind to (e.g. `"127.0.0.1"` or `"0.0.0.0"`)
/// * `port` - Port number to bind to
/// * `engine` - A shared [`CrawlEngine`] that powers all operations
///
/// # Errors
///
/// Returns a [`CrawlError`] if the address is invalid or the server fails to start.
pub async fn serve(host: &str, port: u16, engine: Arc<CrawlEngine>) -> Result<(), CrawlError> {
    let ip: IpAddr = host
        .parse()
        .map_err(|e| CrawlError::InvalidConfig(format!("invalid host address: {e}")))?;

    let addr = SocketAddr::new(ip, port);
    let app = create_router(engine);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| CrawlError::Other(format!("failed to bind {addr}: {e}")))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| CrawlError::Other(format!("server error: {e}")))?;

    Ok(())
}

/// Start the API server with a [`CrawlConfig`], building the engine internally.
///
/// Convenience wrapper that constructs a default [`CrawlEngine`] from the given config.
///
/// # Arguments
///
/// * `host` - IP address to bind to
/// * `port` - Port number to bind to
/// * `config` - Crawl configuration for the engine
///
/// # Errors
///
/// Returns a [`CrawlError`] if the config is invalid, the address cannot be
/// parsed, or the server fails to start.
pub async fn serve_with_config(
    host: &str,
    port: u16,
    config: CrawlConfig,
) -> Result<(), CrawlError> {
    let engine = CrawlEngine::builder().config(config).build()?;
    serve(host, port, Arc::new(engine)).await
}
