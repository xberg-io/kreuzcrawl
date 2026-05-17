//! Native browser backend for Kreuzcrawl.
//!
//! Portions of this crate are derived from Obscura.
//! Source: https://github.com/h4ckf0r0day/obscura
//! Source commit: e371e942fc894578283b00690e34943fe1e45c7e
//! License: Apache-2.0
//! Modifications: adapted into a Kreuzcrawl-owned native browser backend.

#[macro_use]
extern crate html5ever;

pub mod adapter;
pub mod context;
pub mod dom;
pub mod js;
pub mod lifecycle;
pub mod net;
pub mod page;

pub use context::BrowserContext;
pub use lifecycle::{LifecycleState, WaitUntil};
pub use page::{Page, PageError};
