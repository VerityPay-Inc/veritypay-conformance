//! Path-free harness view of a loaded VP-CS scenario.

use vp_reference_model::{Claim, Evidence};

use crate::build_error::BuildError;
use crate::scenario_binding::ScenarioBinding;
use crate::scenario_id::ScenarioId;
use crate::scenario_metadata::ScenarioMetadata;

/// Immutable handoff from scenario loading to execution paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioContext {
    scenario_id: ScenarioId,
    specification_binding: ScenarioBinding,
    claim: Claim,
    evidence: Evidence,
    metadata: ScenarioMetadata,
}

impl ScenarioContext {
    #[must_use]
    pub fn scenario_id(&self) -> &ScenarioId {
        &self.scenario_id
    }

    #[must_use]
    pub fn specification_binding(&self) -> &ScenarioBinding {
        &self.specification_binding
    }

    #[must_use]
    pub fn claim(&self) -> &Claim {
        &self.claim
    }

    #[must_use]
    pub fn evidence(&self) -> &Evidence {
        &self.evidence
    }

    #[must_use]
    pub fn metadata(&self) -> &ScenarioMetadata {
        &self.metadata
    }

    #[must_use]
    pub fn builder() -> ScenarioContextBuilder {
        ScenarioContextBuilder::new()
    }
}

/// Constructs a [`ScenarioContext`] with validated required fields.
#[derive(Debug, Default)]
pub struct ScenarioContextBuilder {
    scenario_id: Option<ScenarioId>,
    specification_binding: Option<ScenarioBinding>,
    claim: Option<Claim>,
    evidence: Option<Evidence>,
    metadata: ScenarioMetadata,
}

impl ScenarioContextBuilder {
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
    pub fn claim(mut self, claim: Claim) -> Self {
        self.claim = Some(claim);
        self
    }

    #[must_use]
    pub fn evidence(mut self, evidence: Evidence) -> Self {
        self.evidence = Some(evidence);
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: ScenarioMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    #[must_use]
    pub fn metadata_entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut entries = self.metadata.entries().clone();
        entries.insert(key.into(), value.into());
        self.metadata = ScenarioMetadata::from_pairs(entries);
        self
    }

    pub fn build(self) -> Result<ScenarioContext, BuildError> {
        let scenario_id = self
            .scenario_id
            .ok_or_else(|| BuildError::missing("scenario_id"))?;
        let specification_binding = self
            .specification_binding
            .ok_or_else(|| BuildError::missing("specification_binding"))?;
        let claim = self.claim.ok_or_else(|| BuildError::missing("claim"))?;
        let evidence = self
            .evidence
            .ok_or_else(|| BuildError::missing("evidence"))?;

        Ok(ScenarioContext {
            scenario_id,
            specification_binding,
            claim,
            evidence,
            metadata: self.metadata,
        })
    }
}
