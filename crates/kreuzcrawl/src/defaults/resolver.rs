//! A DNS resolver that uses the system's default name resolution.

use std::net::IpAddr;

use async_trait::async_trait;

use crate::error::CrawlError;
use crate::traits::DnsResolver;

/// A DNS resolver that delegates to `tokio::net::lookup_host`.
#[derive(Debug, Clone, Default)]
pub struct SystemResolver;

#[async_trait]
impl DnsResolver for SystemResolver {
    async fn resolve(&self, host: &str) -> Result<Vec<IpAddr>, CrawlError> {
        let addrs = tokio::net::lookup_host(format!("{host}:0"))
            .await
            .map_err(|e| CrawlError::Dns(format!("failed to resolve {host}: {e}")))?;

        Ok(addrs.map(|addr| addr.ip()).collect())
    }
}
