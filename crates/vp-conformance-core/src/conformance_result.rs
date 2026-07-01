//! Per-scenario conformance record (placeholder until Milestone E).

/// Frozen comparison evidence for a single scenario run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceResult {
    passed: bool,
    scenario_id: String,
}

impl ConformanceResult {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self {
            passed: false,
            scenario_id: "placeholder".to_owned(),
        }
    }

    /// Whether the scenario comparison passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.passed
    }

    /// Scenario identifier for this record.
    #[must_use]
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }
}
