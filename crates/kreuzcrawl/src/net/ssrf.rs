//! SSRF policy and validation for outbound HTTP requests.
//!
//! Provides a deny-by-default policy on private IP space, DNS rebinding mitigation,
//! and allowlist matching for URLs in crawl, sitemap, and robots.txt operations.

use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::LazyLock;

/// Private / metadata / loopback CIDRs that are denied by default.
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

/// Hostname/IP allowlist matcher for SSRF policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HostMatcher {
    /// Exact hostname match (case-insensitive).
    Exact(String),
    /// Suffix match: ".kreuzberg.dev" matches "api.kreuzberg.dev" and "kreuzberg.dev".
    Suffix(String),
    /// CIDR match: "10.0.0.0/8" matches IP addresses in that range. Stored as string, parsed on use.
    Cidr(String),
}

impl HostMatcher {
    /// Test if this matcher matches the given hostname.
    pub fn matches_host(&self, host: &str) -> bool {
        match self {
            HostMatcher::Exact(h) => h.eq_ignore_ascii_case(host),
            HostMatcher::Suffix(s) => {
                let suffix_clean = s.trim_start_matches('.').to_ascii_lowercase();
                let host_lower = host.to_ascii_lowercase();
                // Match if host is exactly the suffix, or if it ends with .{suffix}
                host_lower == suffix_clean || host_lower.ends_with(&format!(".{suffix_clean}"))
            }
            HostMatcher::Cidr(_) => false, // Cidr only matches IPs, not hostnames
        }
    }

    /// Test if this matcher matches the given IP address.
    pub fn matches_ip(&self, ip: &IpAddr) -> bool {
        match self {
            HostMatcher::Exact(_) | HostMatcher::Suffix(_) => false,
            HostMatcher::Cidr(cidr_str) => cidr_str.parse::<IpNet>().map(|net| net.contains(ip)).unwrap_or(false),
        }
    }
}

/// SSRF validation error.
#[derive(Debug, thiserror::Error)]
pub enum SsrfError {
    /// URL denied by SSRF policy: private IP, metadata IP, etc.
    #[error("denied by SSRF policy: {reason}")]
    DeniedByPolicy {
        /// Stable category of the denial: `"loopback"`, `"private_network"`,
        /// `"link_local"`, `"unique_local"`, `"multicast"`, or `"unspecified"`.
        reason: &'static str,
    },

    /// Host not on allowlist when an allowlist is configured.
    #[error("host not on allowlist")]
    NotOnAllowlist,

    /// DNS resolution failed for hostname.
    #[error("DNS resolution failed: {0}")]
    DnsResolutionFailed(String),

    /// Invalid URL format.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// URL scheme not in allowlist (e.g., `ftp://` when only `http`/`https` allowed).
    #[error("disallowed scheme: {0}")]
    DisallowedScheme(String),

    /// Too many HTTP redirects encountered during validation.
    #[error("too many redirects")]
    TooManyRedirects,
}

/// SSRF policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsrfPolicy {
    /// If true, reject URLs that resolve to private/metadata IP ranges.
    pub deny_private: bool,

    /// Allowed hostnames and IP ranges. Empty means deny all unless `deny_private` is false.
    pub allowlist: Vec<HostMatcher>,

    /// Maximum number of HTTP redirects to follow during validation.
    pub max_redirects: u8,

    /// Allowed URI schemes. Default: `["http", "https"]`. Not serialized (set at runtime).
    #[serde(skip)]
    pub scheme_allowlist: HashSet<&'static str>,
}

impl Default for SsrfPolicy {
    fn default() -> Self {
        let mut scheme_allowlist = HashSet::new();
        scheme_allowlist.insert("http");
        scheme_allowlist.insert("https");

        Self {
            deny_private: true,
            allowlist: Vec::new(),
            max_redirects: 5,
            scheme_allowlist,
        }
    }
}

impl SsrfPolicy {
    /// Create a policy from environment variables.
    ///
    /// Reads `KREUZCRAWL_ALLOW_PRIVATE_NETWORK` — if set to "1" or "true" (case-insensitive),
    /// sets `deny_private = false`. Otherwise, defaults to `deny_private = true`.
    pub fn from_env() -> Self {
        let allow_private = std::env::var("KREUZCRAWL_ALLOW_PRIVATE_NETWORK")
            .map(|v| v.to_lowercase())
            .ok()
            .and_then(|v| {
                if v == "1" || v == "true" {
                    Some(true)
                } else if v == "0" || v == "false" {
                    Some(false)
                } else {
                    None
                }
            })
            .unwrap_or(false);

        let mut scheme_allowlist = std::collections::HashSet::new();
        scheme_allowlist.insert("http");
        scheme_allowlist.insert("https");

        Self {
            deny_private: !allow_private,
            allowlist: Vec::new(),
            max_redirects: 5,
            scheme_allowlist,
        }
    }
}

/// Validate a URL against the SSRF policy.
///
/// 1. Parses the URL and validates the scheme.
/// 2. If the host is a literal IP, validates that IP directly.
/// 3. Otherwise, resolves the hostname via DNS and validates all resolved IPs.
/// 4. If an allowlist is configured, ensures the host or at least one resolved IP matches.
/// 5. If `deny_private` is true, rejects any IP in the default deny list unless allowlisted.
///
/// DNS rebinding mitigation: all resolved IPs are validated; if ANY resolved IP violates
/// the policy, the URL is rejected.
pub async fn validate_url(url: &url::Url, policy: &SsrfPolicy) -> Result<(), SsrfError> {
    // Validate scheme
    let scheme = url.scheme();
    if !policy.scheme_allowlist.contains(scheme) {
        return Err(SsrfError::DisallowedScheme(scheme.to_string()));
    }

    // Extract hostname
    let host = url
        .host()
        .ok_or_else(|| SsrfError::InvalidUrl(format!("missing hostname: {url}")))?;

    let host_str = match host {
        url::Host::Domain(d) => d,
        url::Host::Ipv4(ip) => {
            let ip_addr: IpAddr = ip.into();
            if is_ip_permitted(ip_addr, policy) {
                return Ok(());
            } else {
                let reason = classify_private_ip(ip_addr);
                return Err(SsrfError::DeniedByPolicy { reason });
            }
        }
        url::Host::Ipv6(ip) => {
            let ip_addr: IpAddr = ip.into();
            if is_ip_permitted(ip_addr, policy) {
                return Ok(());
            } else {
                let reason = classify_private_ip(ip_addr);
                return Err(SsrfError::DeniedByPolicy { reason });
            }
        }
    };

    // Resolve hostname
    let port = url.port().unwrap_or(match scheme {
        "https" => 443,
        _ => 80,
    });

    let lookup_addr = format!("{}:{}", host_str, port);
    let addresses: Vec<IpAddr> = tokio::net::lookup_host(&lookup_addr)
        .await
        .map_err(|e| SsrfError::DnsResolutionFailed(format!("{host_str}: {e}")))?
        .map(|addr| addr.ip())
        .collect();

    if addresses.is_empty() {
        return Err(SsrfError::DnsResolutionFailed(format!(
            "no addresses resolved for {host_str}"
        )));
    }

    // Check if hostname is on allowlist (short-circuit)
    for matcher in &policy.allowlist {
        if matcher.matches_host(host_str) {
            return Ok(());
        }
    }

    // Validate each resolved IP — DNS rebinding mitigation: ALL IPs must pass
    for ip in &addresses {
        if !is_ip_permitted(*ip, policy) {
            let reason = classify_private_ip(*ip);
            return Err(SsrfError::DeniedByPolicy { reason });
        }
    }

    Ok(())
}

/// Test if an IP address is permitted by the SSRF policy.
///
/// Returns true if the IP is allowed, false if it should be rejected.
fn is_ip_permitted(ip: IpAddr, policy: &SsrfPolicy) -> bool {
    // If deny_private is false, allow any IP
    if !policy.deny_private {
        return true;
    }

    // Check if IP matches any Cidr allowlist entry
    if policy.allowlist.iter().any(|m| m.matches_ip(&ip)) {
        return true;
    }

    // Check if IP is in the default deny list
    !DEFAULT_DENY_NETS.iter().any(|net| net.contains(&ip))
}

/// Classify a private IP into a category for error messaging.
fn classify_private_ip(ip: IpAddr) -> &'static str {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            match octets[0] {
                127 => "loopback",
                10 => "private_network",
                172 if octets[1] >= 16 && octets[1] <= 31 => "private_network",
                192 if octets[1] == 168 => "private_network",
                169 if octets[1] == 254 => "link_local",
                0 => "unspecified",
                224..=239 => "multicast",
                _ => "private_network",
            }
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            match segments[0] {
                0x0000
                    if segments[1] == 0
                        && segments[2] == 0
                        && segments[3] == 0
                        && segments[4] == 0
                        && segments[5] == 0
                        && segments[6] == 0
                        && segments[7] == 1 =>
                {
                    "loopback"
                }
                0xfe80 => "link_local",
                0xfc00 | 0xfd00 => "unique_local",
                0xff00..=0xffff => "multicast",
                _ => "private_network",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let policy = SsrfPolicy::default();
        assert!(policy.deny_private);
        assert!(policy.allowlist.is_empty());
        assert_eq!(policy.max_redirects, 5);
        assert!(policy.scheme_allowlist.contains("http"));
        assert!(policy.scheme_allowlist.contains("https"));
        assert!(!policy.scheme_allowlist.contains("ftp"));
    }

    #[test]
    fn test_host_matcher_exact() {
        let matcher = HostMatcher::Exact("example.com".to_string());
        assert!(matcher.matches_host("example.com"));
        assert!(matcher.matches_host("EXAMPLE.COM"));
        assert!(!matcher.matches_host("api.example.com"));
    }

    #[test]
    fn test_host_matcher_suffix() {
        let matcher = HostMatcher::Suffix(".example.com".to_string());
        assert!(matcher.matches_host("api.example.com"));
        assert!(matcher.matches_host("example.com"));
        assert!(matcher.matches_host("API.EXAMPLE.COM"));
        assert!(!matcher.matches_host("notexample.com"));

        let matcher_no_dot = HostMatcher::Suffix("example.com".to_string());
        assert!(matcher_no_dot.matches_host("example.com"));
        assert!(matcher_no_dot.matches_host("api.example.com"));
        assert!(!matcher_no_dot.matches_host("notexample.com"));
    }

    #[test]
    fn test_default_policy_deny_private_true() {
        // Default policy has deny_private = true
        let policy = SsrfPolicy::default();
        assert!(policy.deny_private);
        assert_eq!(policy.max_redirects, 5);
    }

    #[test]
    fn test_default_policy_scheme_allowlist() {
        // Default scheme allowlist includes http and https only
        let policy = SsrfPolicy::default();
        assert!(policy.scheme_allowlist.contains("http"));
        assert!(policy.scheme_allowlist.contains("https"));
        assert!(!policy.scheme_allowlist.contains("ftp"));
    }

    #[test]
    fn test_classify_ipv4_loopback() {
        assert_eq!(
            classify_private_ip(IpAddr::V4("127.0.0.1".parse().unwrap())),
            "loopback"
        );
    }

    #[test]
    fn test_classify_ipv4_private_10() {
        assert_eq!(
            classify_private_ip(IpAddr::V4("10.0.0.1".parse().unwrap())),
            "private_network"
        );
    }

    #[test]
    fn test_classify_ipv4_private_172() {
        assert_eq!(
            classify_private_ip(IpAddr::V4("172.16.0.1".parse().unwrap())),
            "private_network"
        );
    }

    #[test]
    fn test_classify_ipv4_private_192() {
        assert_eq!(
            classify_private_ip(IpAddr::V4("192.168.0.1".parse().unwrap())),
            "private_network"
        );
    }

    #[test]
    fn test_classify_ipv4_link_local() {
        assert_eq!(
            classify_private_ip(IpAddr::V4("169.254.1.1".parse().unwrap())),
            "link_local"
        );
    }

    #[test]
    fn test_classify_ipv6_loopback() {
        assert_eq!(classify_private_ip(IpAddr::V6("::1".parse().unwrap())), "loopback");
    }

    #[test]
    fn test_classify_ipv6_link_local() {
        assert_eq!(
            classify_private_ip(IpAddr::V6("fe80::1".parse().unwrap())),
            "link_local"
        );
    }

    #[test]
    fn test_classify_ipv6_unique_local() {
        assert_eq!(
            classify_private_ip(IpAddr::V6("fc00::1".parse().unwrap())),
            "unique_local"
        );
    }

    #[test]
    fn test_classify_ipv6_multicast() {
        assert_eq!(classify_private_ip(IpAddr::V6("ff00::1".parse().unwrap())), "multicast");
    }

    // ── validate_url: literal-IP fast-path (no DNS required) ────────────────

    #[tokio::test]
    async fn validate_url_rejects_loopback_v4() {
        let policy = SsrfPolicy::default();
        let url = "http://127.0.0.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "loopback" }),
            "expected DeniedByPolicy loopback, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_private_10() {
        let policy = SsrfPolicy::default();
        let url = "http://10.0.0.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(
                err,
                SsrfError::DeniedByPolicy {
                    reason: "private_network"
                }
            ),
            "expected DeniedByPolicy private_network, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_private_172() {
        let policy = SsrfPolicy::default();
        let url = "http://172.16.0.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(
                err,
                SsrfError::DeniedByPolicy {
                    reason: "private_network"
                }
            ),
            "expected DeniedByPolicy private_network, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_private_192() {
        let policy = SsrfPolicy::default();
        let url = "http://192.168.1.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(
                err,
                SsrfError::DeniedByPolicy {
                    reason: "private_network"
                }
            ),
            "expected DeniedByPolicy private_network, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_metadata_ip() {
        let policy = SsrfPolicy::default();
        let url = "http://169.254.169.254/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "link_local" }),
            "expected DeniedByPolicy link_local, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_unspecified() {
        let policy = SsrfPolicy::default();
        let url = "http://0.0.0.0/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "unspecified" }),
            "expected DeniedByPolicy unspecified, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_multicast() {
        let policy = SsrfPolicy::default();
        let url = "http://224.0.0.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "multicast" }),
            "expected DeniedByPolicy multicast, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_ipv6_loopback() {
        let policy = SsrfPolicy::default();
        let url = "http://[::1]/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "loopback" }),
            "expected DeniedByPolicy loopback, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_ipv6_link_local() {
        let policy = SsrfPolicy::default();
        let url = "http://[fe80::1]/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "link_local" }),
            "expected DeniedByPolicy link_local, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_ipv6_ula() {
        let policy = SsrfPolicy::default();
        let url = "http://[fc00::1]/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "unique_local" }),
            "expected DeniedByPolicy unique_local, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_ipv6_multicast() {
        let policy = SsrfPolicy::default();
        let url = "http://[ff00::1]/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(err, SsrfError::DeniedByPolicy { reason: "multicast" }),
            "expected DeniedByPolicy multicast, got {err:?}"
        );
    }

    #[tokio::test]
    async fn validate_url_rejects_disallowed_scheme_ftp() {
        let policy = SsrfPolicy::default();
        let url = "ftp://example.com/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        match err {
            SsrfError::DisallowedScheme(s) => assert_eq!(s, "ftp"),
            other => panic!("expected DisallowedScheme(\"ftp\"), got {other:?}"),
        }
    }

    #[tokio::test]
    async fn validate_url_rejects_disallowed_scheme_file() {
        let policy = SsrfPolicy::default();
        let url = "file:///etc/passwd".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        match err {
            SsrfError::DisallowedScheme(s) => assert_eq!(s, "file"),
            other => panic!("expected DisallowedScheme(\"file\"), got {other:?}"),
        }
    }

    #[tokio::test]
    async fn validate_url_permits_public_ipv4() {
        let policy = SsrfPolicy::default();
        let url = "http://1.1.1.1/".parse::<url::Url>().unwrap();
        validate_url(&url, &policy)
            .await
            .expect("public IPv4 should be permitted");
    }

    #[tokio::test]
    async fn validate_url_permits_public_ipv6() {
        let policy = SsrfPolicy::default();
        let url = "http://[2606:4700:4700::1111]/".parse::<url::Url>().unwrap();
        validate_url(&url, &policy)
            .await
            .expect("public IPv6 should be permitted");
    }

    #[tokio::test]
    async fn validate_url_cidr_allowlist_permits_private() {
        let mut policy = SsrfPolicy::default();
        policy.allowlist.push(HostMatcher::Cidr("10.0.0.0/8".to_string()));
        let url = "http://10.5.6.7/".parse::<url::Url>().unwrap();
        validate_url(&url, &policy)
            .await
            .expect("10.0.0.0/8 in allowlist should permit 10.5.6.7");
    }

    #[tokio::test]
    async fn validate_url_exact_allowlist_does_not_match_literal_ip() {
        // Exact matchers match hostnames only; they cannot allowlist literal IPs.
        // Cidr is the correct way to allowlist IP addresses.
        let mut policy = SsrfPolicy::default();
        policy.allowlist.push(HostMatcher::Exact("10.0.0.1".to_string()));
        let url = "http://10.0.0.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(
                err,
                SsrfError::DeniedByPolicy {
                    reason: "private_network"
                }
            ),
            "Exact matcher must not bypass CIDR-based IP denial; got {err:?}"
        );
    }

    #[test]
    fn validate_url_suffix_no_leading_dot_does_not_match_substring() {
        // Suffix("example.com") must not match "notexample.com" as a substring.
        // This is a unit test on HostMatcher directly — no network call needed.
        let matcher = HostMatcher::Suffix("example.com".to_string());
        assert!(
            !matcher.matches_host("notexample.com"),
            "Suffix(\"example.com\") must not match \"notexample.com\""
        );
    }

    #[tokio::test]
    async fn deny_private_false_permits_everything() {
        let policy = SsrfPolicy {
            deny_private: false,
            ..SsrfPolicy::default()
        };
        let url = "http://127.0.0.1/".parse::<url::Url>().unwrap();
        validate_url(&url, &policy)
            .await
            .expect("deny_private=false must permit loopback");
    }

    #[allow(unsafe_code)]
    #[tokio::test]
    #[serial_test::serial]
    async fn from_env_honors_kreuzcrawl_allow_private_network() {
        // SAFETY: #[serial] serialises all tests carrying this attribute, so no
        // other thread reads the environment variable concurrently.
        unsafe { std::env::set_var("KREUZCRAWL_ALLOW_PRIVATE_NETWORK", "true") };
        let policy = SsrfPolicy::from_env();
        unsafe { std::env::remove_var("KREUZCRAWL_ALLOW_PRIVATE_NETWORK") };
        let url = "http://10.0.0.1/".parse::<url::Url>().unwrap();
        validate_url(&url, &policy)
            .await
            .expect("KREUZCRAWL_ALLOW_PRIVATE_NETWORK=true must permit private IPs");
    }

    #[allow(unsafe_code)]
    #[tokio::test]
    #[serial_test::serial]
    async fn from_env_default_denies() {
        // SAFETY: #[serial] serialises all tests carrying this attribute.
        unsafe { std::env::remove_var("KREUZCRAWL_ALLOW_PRIVATE_NETWORK") };
        let policy = SsrfPolicy::from_env();
        let url = "http://10.0.0.1/".parse::<url::Url>().unwrap();
        let err = validate_url(&url, &policy).await.unwrap_err();
        assert!(
            matches!(
                err,
                SsrfError::DeniedByPolicy {
                    reason: "private_network"
                }
            ),
            "default from_env policy must deny private IPs; got {err:?}"
        );
    }
}
