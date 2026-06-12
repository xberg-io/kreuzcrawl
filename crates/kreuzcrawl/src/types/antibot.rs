//! Pluggable antibot strategy API.
//!
//! [`AntibotStrategy`] is a hook pair: [`AntibotStrategy::pre_request`] fires
//! before the tower-stack fetch (useful for header injection, token rotation,
//! CAPTCHA pre-warming), and [`AntibotStrategy::post_response`] fires after WAF
//! classification to produce a [`Decision`] that the dispatch loop acts on.
//!
//! # Wiring into the engine
//!
//! Attach a strategy via [`crate::types::DispatchProfile::antibot_strategy`].
//! When `None`, the engine's built-in WAF-signal → escalation logic applies
//! unchanged (identical to pre-Cluster-5 behaviour).
//!
//! # Example
//!
//! ```
//! use kreuzcrawl::{AntibotStrategy, AntibotError, Decision};
//! use kreuzcrawl::http::HttpResponse;
//! use kreuzcrawl::WafSignal;
//! use async_trait::async_trait;
//! use std::time::Duration;
//!
//! #[derive(Debug)]
//! struct VendorAwareStrategy;
//!
//! #[async_trait]
//! impl AntibotStrategy for VendorAwareStrategy {
//!     async fn pre_request(&self, _url: &str) -> Result<(), AntibotError> {
//!         Ok(())
//!     }
//!
//!     async fn post_response(
//!         &self,
//!         _response: &HttpResponse,
//!         waf_signal: Option<&WafSignal>,
//!     ) -> Decision {
//!         match waf_signal {
//!             Some(signal) if signal.vendor == "datadome" => {
//!                 Decision::Retry { backoff: Duration::from_secs(5) }
//!             }
//!             Some(_) => Decision::EscalateBrowser,
//!             None => Decision::Accept,
//!         }
//!     }
//! }
//! ```

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use crate::http::HttpResponse;
use crate::types::WafSignal;

/// Errors from [`AntibotStrategy`] hook invocations.
#[derive(Debug, thiserror::Error)]
pub enum AntibotError {
    /// The `pre_request` hook failed.
    #[error("antibot pre-request hook failed: {0}")]
    PreRequest(String),
    /// The `post_response` hook failed.
    #[error("antibot post-response hook failed: {0}")]
    PostResponse(String),
}

/// What the dispatch loop does after the antibot `post_response` hook runs.
///
/// Returned by [`AntibotStrategy::post_response`]. The engine translates each
/// variant into its internal `RetryDirective` before the retry-policy sees it.
#[derive(Debug, Clone)]
pub enum Decision {
    /// Continue normally — hand the response to the retry policy as usual.
    Accept,
    /// Retry the same tier after `backoff`.
    ///
    /// Translated to `RetryDirective::Retry { backoff_ms }` before the
    /// retry-policy runs, bypassing the policy for this attempt.
    Retry {
        /// How long to wait before the retry.
        backoff: Duration,
    },
    /// Rotate to a different proxy and retry the same tier.
    ///
    /// **Not yet implemented.** The engine logs a `tracing::warn!` and falls
    /// through to `Accept` behaviour. A proxy-pool config surface is tracked as
    /// a follow-up.
    RotateProxy,
    /// Force escalation to the Browser tier, bypassing the retry policy.
    ///
    /// Translated to `RetryDirective::Escalate { reason: EscalationReason::AntibotEscalate }`.
    EscalateBrowser,
}

/// Pluggable antibot hook pair.
///
/// Implement this trait to intercept every request/response pair in the
/// dispatch loop. Hook code should be fast — it runs on the same Tokio task as
/// the fetch.
///
/// # Object safety
///
/// `AntibotStrategy` is object-safe. Wrap in `Arc<dyn AntibotStrategy>` and
/// attach to [`crate::types::DispatchProfile::antibot_strategy`].
#[async_trait]
pub trait AntibotStrategy: Send + Sync + fmt::Debug {
    /// Called once per attempt, before the tower-stack fetch fires.
    ///
    /// The `url` is the URL that will be fetched on this attempt. The hook
    /// may perform side effects (warm a token, rotate a header value in a
    /// shared state cell) but cannot modify the outgoing request in this
    /// iteration (use middleware or tower layers for that).
    ///
    /// Return `Err(AntibotError::PreRequest(_))` to abort this attempt and let
    /// the retry policy decide what happens next (it sees a transient error).
    async fn pre_request(&self, url: &str) -> Result<(), AntibotError>;

    /// Called once per successful HTTP response, after WAF classification but
    /// before the retry policy runs.
    ///
    /// `waf_signal` is `Some` when the WAF classifier found a fingerprint match
    /// in the response, `None` for clean responses.
    ///
    /// The returned [`Decision`] overrides the retry policy for this attempt.
    async fn post_response(&self, response: &HttpResponse, waf_signal: Option<&WafSignal>) -> Decision;
}

/// Convenience alias.
pub type DynAntibotStrategy = Arc<dyn AntibotStrategy>;

/// Default [`AntibotStrategy`] that mirrors the pre-Cluster-5 engine behaviour.
///
/// - `pre_request` is a no-op.
/// - `post_response` returns [`Decision::EscalateBrowser`] when a WAF signal is
///   present, and [`Decision::Accept`] otherwise — identical to what the engine
///   previously did without a pluggable strategy.
///
/// Use this as a base for custom strategies that want to add vendor-specific
/// backoff rules without reimplementing the default escalation logic.
#[derive(Debug, Clone, Default)]
pub struct DefaultAntibotStrategy;

impl DefaultAntibotStrategy {
    /// Construct a new `DefaultAntibotStrategy`.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AntibotStrategy for DefaultAntibotStrategy {
    async fn pre_request(&self, _url: &str) -> Result<(), AntibotError> {
        Ok(())
    }

    async fn post_response(&self, _response: &HttpResponse, waf_signal: Option<&WafSignal>) -> Decision {
        if waf_signal.is_some() {
            Decision::EscalateBrowser
        } else {
            Decision::Accept
        }
    }
}
