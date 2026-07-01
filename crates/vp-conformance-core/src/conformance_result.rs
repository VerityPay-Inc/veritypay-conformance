//! Frozen per-scenario conformance comparison artifact.

use crate::build_error::BuildError;
use crate::comparable_result::ComparableResult;
use crate::conformance_verdict::ConformanceVerdict;
use crate::scenario_binding::ScenarioBinding;
use crate::scenario_id::ScenarioId;
use crate::scenario_metadata::ScenarioMetadata;

/// Immutable record of a conformance check for one scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceResult {
    scenario_id: ScenarioId,
    specification_binding: ScenarioBinding,
    verdict: ConformanceVerdict,
    oracle_result: ComparableResult,
    implementation_result: ComparableResult,
    comparison_notes: ScenarioMetadata,
}

impl ConformanceResult {
    #[must_use]
    pub fn scenario_id(&self) -> &ScenarioId {
        &self.scenario_id
    }

    #[must_use]
    pub fn specification_binding(&self) -> &ScenarioBinding {
        &self.specification_binding
    }

    #[must_use]
    pub fn verdict(&self) -> ConformanceVerdict {
        self.verdict
    }

    #[must_use]
    pub fn oracle_result(&self) -> &ComparableResult {
        &self.oracle_result
    }

    #[must_use]
    pub fn implementation_result(&self) -> &ComparableResult {
        &self.implementation_result
    }

    #[must_use]
    pub fn comparison_notes(&self) -> &ScenarioMetadata {
        &self.comparison_notes
    }

    #[must_use]
    pub fn builder() -> ConformanceResultBuilder {
        ConformanceResultBuilder::new()
    }
}

/// Constructs a [`ConformanceResult`] with validated required fields.
#[derive(Debug, Default)]
pub struct ConformanceResultBuilder {
    scenario_id: Option<ScenarioId>,
    specification_binding: Option<ScenarioBinding>,
    verdict: Option<ConformanceVerdict>,
    oracle_result: Option<ComparableResult>,
    implementation_result: Option<ComparableResult>,
    comparison_notes: ScenarioMetadata,
}

impl ConformanceResultBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn scenario_id(mut self, scenario_id: impl Into<ScenarioId>) -> Self {
        self.scenario_id = Some(scenario_id.into());
        self
    }

    #[must_use]
    pub fn specification_binding(mut self, specification_binding: ScenarioBinding) -> Self {
        self.specification_binding = Some(specification_binding);
        self
    }

    #[must_use]
    pub fn verdict(mut self, verdict: ConformanceVerdict) -> Self {
        self.verdict = Some(verdict);
        self
    }

    #[must_use]
    pub fn oracle_result(mut self, oracle_result: ComparableResult) -> Self {
        self.oracle_result = Some(oracle_result);
        self
    }

    #[must_use]
    pub fn implementation_result(mut self, implementation_result: ComparableResult) -> Self {
        self.implementation_result = Some(implementation_result);
        self
    }

    #[must_use]
    pub fn comparison_notes(mut self, comparison_notes: ScenarioMetadata) -> Self {
        self.comparison_notes = comparison_notes;
        self
    }

    #[must_use]
    pub fn comparison_note(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut entries = self.comparison_notes.entries().clone();
        entries.insert(key.into(), value.into());
        self.comparison_notes = ScenarioMetadata::from_pairs(entries);
        self
    }

    pub fn build(self) -> Result<ConformanceResult, BuildError> {
        let scenario_id = self
            .scenario_id
            .ok_or_else(|| BuildError::missing("scenario_id"))?;
        let specification_binding = self
            .specification_binding
            .ok_or_else(|| BuildError::missing("specification_binding"))?;
        let verdict = self.verdict.ok_or_else(|| BuildError::missing("verdict"))?;
        let oracle_result = self
            .oracle_result
            .ok_or_else(|| BuildError::missing("oracle_result"))?;
        let implementation_result = self
            .implementation_result
            .ok_or_else(|| BuildError::missing("implementation_result"))?;

        if !oracle_result.execution_path().is_reference_oracle() {
            return Err(BuildError::invalid(
                "oracle_result",
                "execution_path must be ReferenceOracle",
            ));
        }

        if !implementation_result
            .execution_path()
            .is_implementation_adapter()
        {
            return Err(BuildError::invalid(
                "implementation_result",
                "execution_path must be ImplementationAdapter",
            ));
        }

        Ok(ConformanceResult {
            scenario_id,
            specification_binding,
            verdict,
            oracle_result,
            implementation_result,
            comparison_notes: self.comparison_notes,
        })
    }
}
