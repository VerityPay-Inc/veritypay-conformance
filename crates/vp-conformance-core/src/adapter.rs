//! Implementation adapter contract.

use vp_reference_model::Outcome;

use crate::adapter_error::AdapterError;
use crate::adapter_id::AdapterId;
use crate::adapter_run_options::AdapterRunOptions;
use crate::comparable_result::{ComparableResult, ExecutionPath};
use crate::scenario_context::ScenarioContext;

/// Bridge between the harness and an implementation under test.
pub trait ImplementationAdapter {
    /// Stable adapter identifier for reports and metadata.
    fn id(&self) -> &AdapterId;

    /// Executes the implementation path for a loaded scenario.
    ///
    /// Implementations must return a [`ComparableResult`] tagged with
    /// [`ExecutionPath::ImplementationAdapter`].
    fn run(&self, context: &ScenarioContext) -> Result<ComparableResult, AdapterError>;

    /// Executes with optional per-run knobs. Defaults to [`Self::run`].
    fn run_with_options(
        &self,
        context: &ScenarioContext,
        options: &AdapterRunOptions,
    ) -> Result<ComparableResult, AdapterError> {
        let _ = options;
        self.run(context)
    }
}

/// Builds an implementation-path [`ComparableResult`] from scenario inputs.
pub fn build_implementation_result(
    adapter_id: &AdapterId,
    context: &ScenarioContext,
    outcome: Outcome,
) -> Result<ComparableResult, AdapterError> {
    ComparableResult::builder()
        .execution_path(ExecutionPath::implementation_adapter(adapter_id.as_str()))
        .evaluated_claim_id(context.claim().id.clone())
        .outcome(outcome)
        .specification_binding(context.specification_binding().clone())
        .metadata(context.metadata().clone())
        .build()
        .map_err(AdapterError::InvalidResult)
}
