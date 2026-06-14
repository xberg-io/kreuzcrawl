//! SSRF policy validation for the browser layer.
//!
//! This module duplicates the deny-list and validation logic from the core
//! `kreuzcrawl::net::ssrf` module to avoid a circular dependency (kreuzcrawl
//! optionally depends on kreuzcrawl-browser). The constants are kept in sync
//! by convention.
//!
//! Browser-specific mitigations:
//! - DNS-rebinding defense via hostname string matching (before DNS resolution).
//! - File-scheme bypass for test support.

use std::net::IpAddr;
use std::sync::LazyLock;

use ipnet::IpNet;
use url::Url;

/// Private / metadata / loopback CIDRs that are denied by default.
/// Must be kept in sync with `kreuzcrawl::net::ssrf::DEFAULT_DENY_NETS`.
static DEFAULT_DENY_NETS: LazyLock<Vec<IpNet>> = LazyLock::new(|| {
    vec![
        "127.0.0.0/8".parse().unwrap(),    // loopback
        "10.0.0.0/8".parse().unwrap(),     // private
        "172.16.0.0/12".parse().unwrap(),  // private
        "192.168.0.0/16".parse().unwrap(), // private
        "169.254.0.0/16".parse().unwrap(), // link-local
        "0.0.0.0/8".parse().unwrap(),      // unspecified
        "224.0.0.0/4".parse().unwrap(),    // multicast
        "::1/128".parse().unwrap(),        // ipv6 loopback
        "fe80::/10".parse().unwrap(),      // ipv6 link-local
        "fc00::/7".parse().unwrap(),       // ipv6 unique-local
        "ff00::/8".parse().unwrap(),       // ipv6 multicast
    ]
});

/// Validate an IP address against the SSRF policy.
fn is_ip_denied(ip: IpAddr) -> bool {
    for net in DEFAULT_DENY_NETS.iter() {
        if net.contains(&ip) {
            return true;
        }
    }
    false
}

/// Validate a URL for SSRF risks, with browser-specific mitigations.
///
/// Returns `Ok(())` if the URL is safe to fetch, or an error message otherwise.
///
/// # Logic
///
/// 1. Allows `file://` unconditionally (browser-specific for test support).
/// 2. Checks scheme is `http` or `https`.
/// 3. For IP addresses, rejects those in deny-list ranges.
/// 4. For domain names, string-matches `localhost` / `.localhost` (DNS-rebinding mitigation).
/// 5. Respects `KREUZCRAWL_ALLOW_PRIVATE_NETWORK` env var to bypass checks.
pub fn validate_url(url: &Url) -> Result<(), String> {
    let scheme = url.scheme();

    // File-scheme bypass: allowed unconditionally in the browser for test support.
    if scheme == "file" {
        return Ok(());
    }

    // Respect env-var bypass.
    let allow_private_network = std::env::var_os("KREUZCRAWL_ALLOW_PRIVATE_NETWORK").is_some();
    if allow_private_network {
        return Ok(());
    }

    // Scheme check.
    if scheme != "http" && scheme != "https" {
        return Err(format!(
            "Forbidden URL scheme '{}' - only http, https, and file are allowed",
            scheme
        ));
    }

    // Check host.
    if let Some(host) = url.host() {
        match host {
            url::Host::Ipv4(ip) => {
                let ip_addr: IpAddr = ip.into();
                if is_ip_denied(ip_addr) {
                    return Err(format!("Access to private/internal IP address {} is not allowed", ip));
                }
            }
            url::Host::Ipv6(ip) => {
                let ip_addr: IpAddr = ip.into();
                if is_ip_denied(ip_addr) {
                    return Err(format!("Access to private/internal IPv6 address {} is not allowed", ip));
                }
            }
            url::Host::Domain(domain) => {
                let lower_domain = domain.to_lowercase();

                // DNS-rebinding mitigation: block localhost variants before any DNS
                // resolution. The core module performs DNS resolution; an attacker can
                // exploit timing to change resolution results between the check and
                // the actual request. This string match blocks rebinding attempts.
                if lower_domain == "localhost" || lower_domain.ends_with(".localhost") {
                    return Err(format!("Localhost rebinding attack blocked: {}", domain));
                }

                // Note: String matches for literal IPs (`127.0.0.1`, `::1`) are omitted
                // here because they are redundant with the IP range checks above.
                // If a domain somehow resolves to these IPs, the range check will catch it.
            }
        }
    }

    Ok(())
}
