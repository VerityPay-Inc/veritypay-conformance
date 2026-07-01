//! Reference oracle boundary (placeholder until Milestone D).

/// Produces expected outcomes via `veritypay-reference`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReferenceOracle;

impl ReferenceOracle {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }

    /// Oracle invocation is deferred until Milestone D.
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        true
    }
}
