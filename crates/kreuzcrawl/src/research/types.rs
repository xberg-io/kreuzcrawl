//! Types for the deep research agent.

use serde::{Deserialize, Serialize};

/// Configuration for a research session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchConfig {
    /// The research query or topic.
    pub query: String,
    /// Maximum number of research steps (crawl + synthesize cycles).
    #[serde(default = "default_max_steps")]
    pub max_steps: usize,
    /// Maximum number of pages to crawl per step.
    #[serde(default = "default_max_pages_per_step")]
    pub max_pages_per_step: usize,
    /// Maximum link-hop depth per crawl step.
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    /// Optional seed URLs to start crawling from.
    #[serde(default)]
    pub seed_urls: Vec<String>,
}

fn default_max_steps() -> usize {
    10
}
fn default_max_pages_per_step() -> usize {
    5
}
fn default_max_depth() -> usize {
    3
}

impl Default for ResearchConfig {
    fn default() -> Self {
        Self {
            query: String::new(),
            max_steps: default_max_steps(),
            max_pages_per_step: default_max_pages_per_step(),
            max_depth: default_max_depth(),
            seed_urls: Vec::new(),
        }
    }
}

/// The final output of a research session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    /// The original research query.
    pub query: String,
    /// The synthesized report (Markdown).
    pub synthesis: String,
    /// Individual findings extracted from crawled pages.
    pub findings: Vec<Finding>,
    /// Sources visited during the research.
    pub sources: Vec<SourceInfo>,
    /// The sequence of steps the agent took.
    pub steps: Vec<ResearchStep>,
    /// Total number of pages crawled across all steps.
    pub pages_crawled: usize,
    /// Optional extraction metadata (cost, tokens, model).
    pub cost: Option<crate::types::ExtractionMeta>,
}

/// A single piece of extracted content from a crawled page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// The extracted text content.
    pub content: String,
    /// The URL the content was extracted from.
    pub source_url: String,
    /// A relevance score in `[0.0, 1.0]`.
    pub relevance_score: f64,
}

/// Information about a source visited during research.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// The page URL.
    pub url: String,
    /// The page title, if available.
    pub title: Option<String>,
    /// A short text snippet from the page, if available.
    pub snippet: Option<String>,
}

/// A single step in the research loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchStep {
    /// Zero-based step index.
    pub step_number: usize,
    /// The action taken during this step.
    pub action: StepAction,
    /// URLs visited during this step.
    pub urls_visited: Vec<String>,
    /// Number of findings extracted during this step.
    pub findings_count: usize,
    /// Error message if the step failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// The action performed in a research step.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StepAction {
    /// Crawl a URL to a given depth.
    Crawl {
        /// The target URL.
        url: String,
        /// Maximum link-hop depth for this crawl.
        depth: usize,
    },
    /// Synthesize collected findings into a report.
    Synthesize,
}
