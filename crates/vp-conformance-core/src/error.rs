//! Shared harness errors (placeholder).

/// Conformance harness error outside scenario fixture parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConformanceError {
    /// Placeholder variant for workspace bootstrap.
    Placeholder,
}

impl ConformanceError {
    /// Placeholder value for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::Placeholder
    }
}

impl std::fmt::Display for ConformanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Placeholder => f.write_str("placeholder error"),
        }
    }
}

impl std::error::Error for ConformanceError {}
