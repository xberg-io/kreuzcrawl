//! Deep research agent module.
//!
//! Provides an autonomous research agent that plans, crawls, and synthesizes
//! findings using an LLM-driven loop. Feature-gated behind `ai`.

mod agent;
mod planner;
mod synthesizer;
mod types;

pub use agent::ResearchAgent;
pub use types::{Finding, ResearchConfig, ResearchResult, ResearchStep, SourceInfo, StepAction};
