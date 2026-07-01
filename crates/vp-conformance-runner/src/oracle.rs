//! Reference oracle — invokes `veritypay-reference` per ADR-0007.

use vp_conformance_core::{
    ComparableResult, ExecutionPath, ScenarioBinding, ScenarioContext, ScenarioMetadata,
};
use vp_reference_core::{
    EvaluationContext, EvaluationOptions, SpecificationContext, SpecificationSummary,
};
use vp_reference_interpreter::Interpreter;
use vp_reference_model::{SpecificationBinding, VerificationResult};

use crate::oracle_error::OracleError;

/// Produces expected outcomes via the reference interpreter public contract.
#[derive(Debug)]
pub struct ReferenceOracle {
    interpreter: Interpreter,
}

impl Default for ReferenceOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferenceOracle {
    #[must_use]
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    /// Bootstrap-compatible constructor.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    /// Returns whether the oracle crate is wired for workspace bootstrap checks.
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        true
    }

    /// Evaluates a loaded scenario through `Interpreter::evaluate`.
    pub fn evaluate(&self, context: &ScenarioContext) -> Result<ComparableResult, OracleError> {
        let evaluation = build_evaluation_context(context)?;
        let verification = self.interpreter.evaluate(&evaluation);
        verification_to_comparable(context, &verification)
    }
}

fn build_evaluation_context(context: &ScenarioContext) -> Result<EvaluationContext, OracleError> {
    let binding = context.specification_binding();
    let specification = SpecificationContext {
        spec_root_identity: context.scenario_id().as_str().to_owned(),
        edition_id: binding.edition_id().map(str::to_owned),
        protocol_version: binding.protocol_version().map(str::to_owned),
        summary: SpecificationSummary::default(),
    };

    EvaluationContext::builder()
        .specification_context(specification)
        .claim(context.claim().clone())
        .evidence(context.evidence().clone())
        .options(EvaluationOptions::default())
        .build()
        .map_err(OracleError::context_build)
}

fn verification_to_comparable(
    context: &ScenarioContext,
    verification: &VerificationResult,
) -> Result<ComparableResult, OracleError> {
    ComparableResult::builder()
        .execution_path(ExecutionPath::reference_oracle())
        .evaluated_claim_id(verification.evaluated_claim_id.clone())
        .outcome(verification.outcome)
        .specification_binding(scenario_binding_from_verification(
            &verification.specification_binding,
        )?)
        .metadata(comparable_metadata(context, verification))
        .build()
        .map_err(OracleError::InvalidResult)
}

fn scenario_binding_from_verification(
    binding: &SpecificationBinding,
) -> Result<ScenarioBinding, OracleError> {
    let mut builder = ScenarioBinding::builder();

    if let Some(edition_id) = &binding.edition_id {
        if !edition_id.is_empty() {
            builder = builder.edition_id(edition_id.clone());
        }
    }

    if let Some(protocol_version) = &binding.protocol_version {
        if !protocol_version.is_empty() {
            builder = builder.protocol_version(protocol_version.clone());
        }
    }

    builder.build().map_err(OracleError::InvalidResult)
}

fn comparable_metadata(
    context: &ScenarioContext,
    verification: &VerificationResult,
) -> ScenarioMetadata {
    let mut entries = context.metadata().entries().clone();

    for (key, value) in verification.metadata.entries() {
        entries.insert(format!("verification.{key}"), value.clone());
    }

    for (index, reason) in verification.reasons.iter().enumerate() {
        entries.insert(format!("reason.{index}"), reason.clone());
    }

    ScenarioMetadata::from_pairs(entries)
}
