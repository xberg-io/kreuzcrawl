pub mod blocklist;
pub mod client;
pub mod cookies;
pub mod interceptor;
pub mod robots;
#[cfg(feature = "stealth")]
pub mod wreq_client;

pub use blocklist::is_blocked as is_tracker_blocked;
pub use client::{ObscuraHttpClient, ObscuraNetError, RequestInfo, ResourceType, Response};
pub use cookies::{CookieInfo, CookieJar};
pub use robots::RobotsCache;
#[cfg(feature = "stealth")]
pub use wreq_client::{StealthHttpClient, STEALTH_USER_AGENT};
