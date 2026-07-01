//! Harness-stable view of a loaded VP-CS scenario (placeholder until Milestone B).

/// Path-free scenario handoff for adapter and oracle execution paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioContext {
    scenario_id: String,
}

impl ScenarioContext {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self {
            scenario_id: "placeholder".to_owned(),
        }
    }

    /// Stable scenario identifier.
    #[must_use]
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }
}
