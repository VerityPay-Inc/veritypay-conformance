//! Minimal adapter for harness boundary tests (placeholder until Milestone C).

use vp_conformance_core::{ImplementationAdapter, ScenarioContext};

/// Stub adapter that demonstrates the harness boundary without execution.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StubAdapter;

impl StubAdapter {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }
}

impl ImplementationAdapter for StubAdapter {
    fn adapter_id(&self) -> &str {
        "stub"
    }

    fn accepts(&self, _context: &ScenarioContext) -> bool {
        false
    }
}
