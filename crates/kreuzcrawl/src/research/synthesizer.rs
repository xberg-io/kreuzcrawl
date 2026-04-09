//! Research synthesizer that combines findings into a coherent report.

use super::types::{Finding, SourceInfo};
use crate::error::CrawlError;

/// Combines findings into a Markdown report.
///
/// Currently sorts findings by relevance and concatenates them.
/// Future iterations will use LLM to generate a proper narrative synthesis.
pub(crate) struct ResearchSynthesizer {}

impl ResearchSynthesizer {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Produce a Markdown report from the collected findings and sources.
    pub(crate) async fn synthesize(
        &self,
        query: &str,
        findings: &[Finding],
        sources: &[SourceInfo],
    ) -> Result<String, CrawlError> {
        let mut sorted: Vec<&Finding> = findings.iter().collect();
        sorted.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut report = format!("# Research Report: {query}\n\n");
        report.push_str("## Key Findings\n\n");
        for (i, finding) in sorted.iter().enumerate().take(20) {
            report.push_str(&format!(
                "{}. {} (source: {}, relevance: {:.2})\n\n",
                i + 1,
                finding.content,
                finding.source_url,
                finding.relevance_score,
            ));
        }

        if !sources.is_empty() {
            report.push_str("## Sources\n\n");
            for (i, source) in sources.iter().enumerate() {
                let title = source.title.as_deref().unwrap_or(&source.url);
                report.push_str(&format!("{}. [{}]({})\n", i + 1, title, source.url));
            }
        }

        Ok(report)
    }
}
