//! Configuration types for benchmark and profiling runs.

use std::path::PathBuf;
use std::time::Duration;

use crate::types::ExecutionMode;

/// Runtime configuration for a benchmark run.
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Directory where result JSON files are written.
    pub output_dir: PathBuf,
    /// Whether to fetch pages live or from the local cache.
    pub execution_mode: ExecutionMode,
    /// Maximum number of concurrent scrape workers.
    pub max_concurrent: usize,
    /// Minimum gap between requests to the same host, in milliseconds.
    pub rate_limit_ms: u64,
    /// Per-request timeout.
    pub timeout: Duration,
    /// Number of warmup iterations to run before recording measurements.
    pub warmup_iterations: usize,
    /// Number of timed iterations to include in statistics.
    pub benchmark_iterations: usize,
    /// Whether to compute quality metrics against fixture ground truth.
    pub measure_quality: bool,
    /// Whether to persist fetched HTML pages to the cache directory.
    pub save_cache: bool,
    /// Directory used for the HTML page cache.
    pub cache_dir: PathBuf,
    /// Optional sharding specification `(shard_index, total_shards)`.
    pub shard: Option<(usize, usize)>,
    /// Optional regex filter applied to fixture IDs.
    pub filter: Option<String>,
    /// Human-readable dataset name included in benchmark metadata.
    ///
    /// When `None`, [`crate::output`] falls back to the `cache_dir` file name.
    pub dataset_name: Option<String>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("results"),
            execution_mode: ExecutionMode::Cached,
            max_concurrent: 10,
            rate_limit_ms: 200,
            timeout: Duration::from_secs(30),
            warmup_iterations: 1,
            benchmark_iterations: 3,
            measure_quality: true,
            save_cache: false,
            cache_dir: PathBuf::from(".benchmark-cache"),
            shard: None,
            filter: None,
            dataset_name: None,
        }
    }
}

/// Configuration for a CPU or memory profiling session.
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
    /// Directory where flamegraphs and profile data are written.
    pub output_dir: PathBuf,
    /// Sampling frequency for CPU profiling, in Hz.
    pub sampling_frequency: i32,
    /// Number of URLs to scrape during a profiling session.
    pub sample_size: usize,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("profiles"),
            sampling_frequency: 1000,
            sample_size: 50,
        }
    }
}
