//! Orchestrates scenario load and execution paths (placeholder until Milestones D–E).

use vp_conformance_scenarios::ScenarioLoader;

/// Coordinates loader, adapter, oracle, and comparison stages.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConformanceRunner {
    loader: ScenarioLoader,
}

impl ConformanceRunner {
    /// Creates a runner with default bootstrap dependencies.
    #[must_use]
    pub fn new() -> Self {
        Self {
            loader: ScenarioLoader::new(),
        }
    }

    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    /// Returns whether orchestration wiring is bootstrapped.
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        self.loader.is_bootstrapped()
    }
}
