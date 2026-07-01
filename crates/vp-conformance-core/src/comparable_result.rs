//! Shared execution result shape for a single adapter or oracle path.

use vp_reference_model::{ClaimId, Outcome};

use crate::build_error::BuildError;
use crate::scenario_binding::ScenarioBinding;
use crate::scenario_metadata::ScenarioMetadata;

/// Identifies which execution path produced a [`ComparableResult`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionPath {
    /// Result from the reference oracle path.
    ReferenceOracle,
    /// Result from an implementation adapter.
    ImplementationAdapter { adapter_id: String },
}

impl ExecutionPath {
    #[must_use]
    pub fn reference_oracle() -> Self {
        Self::ReferenceOracle
    }

    #[must_use]
    pub fn implementation_adapter(adapter_id: impl Into<String>) -> Self {
        Self::ImplementationAdapter {
            adapter_id: adapter_id.into(),
        }
    }

    #[must_use]
    pub fn is_reference_oracle(&self) -> bool {
        matches!(self, Self::ReferenceOracle)
    }

    #[must_use]
    pub fn is_implementation_adapter(&self) -> bool {
        matches!(self, Self::ImplementationAdapter { .. })
    }
}

/// Result from exactly one execution path — reference oracle or implementation adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparableResult {
    execution_path: ExecutionPath,
    evaluated_claim_id: ClaimId,
    outcome: Outcome,
    specification_binding: ScenarioBinding,
    metadata: ScenarioMetadata,
}

impl ComparableResult {
    #[must_use]
    pub fn execution_path(&self) -> &ExecutionPath {
        &self.execution_path
    }

    #[must_use]
    pub fn evaluated_claim_id(&self) -> &ClaimId {
        &self.evaluated_claim_id
    }

    #[must_use]
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    #[must_use]
    pub fn specification_binding(&self) -> &ScenarioBinding {
        &self.specification_binding
    }

    #[must_use]
    pub fn metadata(&self) -> &ScenarioMetadata {
        &self.metadata
    }

    #[must_use]
    pub fn builder() -> ComparableResultBuilder {
        ComparableResultBuilder::new()
    }
}

/// Constructs a [`ComparableResult`] with validated required fields.
#[derive(Debug, Default)]
pub struct ComparableResultBuilder {
    execution_path: Option<ExecutionPath>,
    evaluated_claim_id: Option<ClaimId>,
    outcome: Option<Outcome>,
    specification_binding: Option<ScenarioBinding>,
    metadata: ScenarioMetadata,
}

impl ComparableResultBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn execution_path(mut self, execution_path: ExecutionPath) -> Self {
        self.execution_path = Some(execution_path);
        self
    }

    #[must_use]
    pub fn evaluated_claim_id(mut self, evaluated_claim_id: impl Into<ClaimId>) -> Self {
        self.evaluated_claim_id = Some(evaluated_claim_id.into());
        self
    }

    #[must_use]
    pub fn outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    #[must_use]
    pub fn specification_binding(mut self, specification_binding: ScenarioBinding) -> Self {
        self.specification_binding = Some(specification_binding);
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: ScenarioMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn build(self) -> Result<ComparableResult, BuildError> {
        let execution_path = self
            .execution_path
            .ok_or_else(|| BuildError::missing("execution_path"))?;
        let evaluated_claim_id = self
            .evaluated_claim_id
            .ok_or_else(|| BuildError::missing("evaluated_claim_id"))?;
        let outcome = self.outcome.ok_or_else(|| BuildError::missing("outcome"))?;
        let specification_binding = self
            .specification_binding
            .ok_or_else(|| BuildError::missing("specification_binding"))?;

        Ok(ComparableResult {
            execution_path,
            evaluated_claim_id,
            outcome,
            specification_binding,
            metadata: self.metadata,
        })
    }
}
