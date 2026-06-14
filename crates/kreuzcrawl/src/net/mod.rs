//! Network utilities: SSRF policy, validation, and security.

pub mod ssrf;

pub use ssrf::{HostMatcher, SsrfError, SsrfPolicy, validate_url};
