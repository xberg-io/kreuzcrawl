//! Page interaction module for action-based browser automation.
//!
//! Provides action type definitions and validation. Action execution
//! is handled by the browser module when the `browser` feature is enabled.

mod actions;
mod validation;

pub use actions::{
    MAX_ACTIONS, MAX_SCRIPT_LEN, MAX_SCROLL_AMOUNT, MAX_SELECTOR_LEN, MAX_SINGLE_WAIT_MS,
    MAX_TEXT_LEN, MAX_TOTAL_WAIT_SECS, PageAction, ScrollDirection,
};
pub use validation::validate_actions;
