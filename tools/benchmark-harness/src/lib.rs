//! Benchmark harness library for kreuzcrawl.
//!
//! Provides fixtures, adapters, a benchmark runner, quality metrics, and
//! statistical helpers for evaluating scraping correctness and performance.

pub mod adapter;
pub mod adapters;
pub mod cache;
pub mod config;
pub mod dataset;
pub mod error;
pub mod fixture;
pub mod monitoring;
pub mod output;
pub mod profiling;
pub mod quality;
pub mod runner;
pub mod stats;
pub mod types;

pub use config::{BenchmarkConfig, ProfilingConfig};
pub use error::{Error, Result};
pub use types::*;
