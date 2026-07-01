//! Implementation adapter contract (placeholder until Milestone C).

use crate::ScenarioContext;

/// Bridge between the harness and an implementation under test.
pub trait ImplementationAdapter {
    /// Stable adapter identifier for reports and metadata.
    fn adapter_id(&self) -> &str;

    /// Whether this adapter is wired for the given scenario context.
    ///
    /// Execution is deferred until Milestone C.
    fn accepts(&self, context: &ScenarioContext) -> bool;
}
