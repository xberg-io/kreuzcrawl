//! Research planner that decides the next step in the research loop.

use super::types::{Finding, StepAction};
use crate::error::CrawlError;

/// Plans the next research action based on current state.
///
/// Currently uses a simple round-robin over seed URLs, then synthesizes.
/// Future iterations will use LLM to plan intelligently based on findings.
pub(crate) struct ResearchPlanner {}

impl ResearchPlanner {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Decide the next action given the current research state.
    ///
    /// Returns [`StepAction::Crawl`] while there are unvisited seed URLs and
    /// we have not reached `max_steps - 1`. Returns [`StepAction::Synthesize`]
    /// otherwise.
    pub(crate) async fn plan_next_step(
        &self,
        _query: &str,
        seed_urls: &[String],
        _findings: &[Finding],
        step_number: usize,
        max_steps: usize,
    ) -> Result<StepAction, CrawlError> {
        if step_number < seed_urls.len() && step_number < max_steps {
            Ok(StepAction::Crawl {
                url: seed_urls[step_number].clone(),
                depth: 1,
            })
        } else {
            Ok(StepAction::Synthesize)
        }
    }
}
