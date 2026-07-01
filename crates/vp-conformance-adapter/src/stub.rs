//! Test-only stub adapter for the implementation boundary.

use vp_conformance_core::{
    build_implementation_result, AdapterError, AdapterId, AdapterRunOptions, ComparableResult,
    ImplementationAdapter, ScenarioContext,
};
use vp_reference_model::Outcome;

/// Returns a configured [`Outcome`] for any scenario — useful for harness tests only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StubAdapter {
    adapter_id: AdapterId,
    outcome: Outcome,
}

impl StubAdapter {
    #[must_use]
    pub fn new(adapter_id: impl Into<AdapterId>, outcome: Outcome) -> Self {
        Self {
            adapter_id: adapter_id.into(),
            outcome,
        }
    }

    /// Default stub retained for workspace bootstrap wiring.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new("stub", Outcome::Indeterminate)
    }

    #[must_use]
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }
}

impl ImplementationAdapter for StubAdapter {
    fn id(&self) -> &AdapterId {
        &self.adapter_id
    }

    fn run(&self, context: &ScenarioContext) -> Result<ComparableResult, AdapterError> {
        build_implementation_result(&self.adapter_id, context, self.outcome)
    }

    fn run_with_options(
        &self,
        context: &ScenarioContext,
        options: &AdapterRunOptions,
    ) -> Result<ComparableResult, AdapterError> {
        let _ = options;
        self.run(context)
    }
}
