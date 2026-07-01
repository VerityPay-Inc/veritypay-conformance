//! Loads VP-CS fixtures into immutable scenario records (placeholder until Milestone B).

use vp_conformance_core::ScenarioContext;

/// Ingests VP-CS fixtures from a validated specification checkout.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScenarioLoader;

impl ScenarioLoader {
    /// Creates a scenario loader instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Placeholder value for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    /// Fixture loading is deferred until Milestone B.
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        let _ = ScenarioContext::placeholder();
        true
    }
}
