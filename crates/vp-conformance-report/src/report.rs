//! Formats conformance results for humans and CI (placeholder until Milestone F).

use vp_conformance_core::ConformanceResult;

/// Presents frozen `ConformanceResult` records.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Report;

impl Report {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }

    /// Report rendering is deferred until Milestone F.
    #[must_use]
    pub fn is_bootstrapped(&self, result: &ConformanceResult) -> bool {
        !result.scenario_id().as_str().is_empty()
    }
}
