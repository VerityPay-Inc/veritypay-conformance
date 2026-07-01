//! Shared execution result shape for adapter and oracle paths (placeholder until Milestone C).

/// Comparable result produced by implementation and reference execution paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparableResult {
    label: String,
}

impl ComparableResult {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self {
            label: "placeholder".to_owned(),
        }
    }

    /// Diagnostic label for bootstrap-only smoke tests.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }
}
