//! Reference oracle tests for Milestone D.1.

use vp_conformance_core::{
    ComparableResult, ExecutionPath, ScenarioBinding, ScenarioContext, ScenarioId,
};
use vp_conformance_runner::ReferenceOracle;
use vp_reference_model::{Assertion, Claim, Evidence, EvidenceContent, Outcome};

fn scenario_context(
    assertion_body: &str,
    evidence_body: &str,
    claim_id: &str,
    evidence_claim_id: &str,
) -> ScenarioContext {
    let claim = Claim::builder()
        .id(claim_id)
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", assertion_body))
        .build()
        .expect("claim");
    let evidence = Evidence::builder()
        .id("evidence-001")
        .claim_id(evidence_claim_id)
        .content(EvidenceContent::new("document", evidence_body))
        .build()
        .expect("evidence");
    let binding = ScenarioBinding::builder()
        .edition_id("2026-01")
        .protocol_version("0.1.0")
        .build()
        .expect("binding");

    ScenarioContext::builder()
        .scenario_id(ScenarioId::new("VP-CS-0001"))
        .specification_binding(binding)
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect("scenario context")
}

#[test]
fn oracle_returns_comparable_result() {
    let context = scenario_context("alpha", "alpha", "claim-001", "claim-001");
    let result = ReferenceOracle::new()
        .evaluate(&context)
        .expect("oracle result");

    assert_eq!(result.outcome(), Outcome::Satisfied);
}

#[test]
fn execution_path_is_reference_oracle() {
    let context = scenario_context("alpha", "alpha", "claim-001", "claim-001");
    let result = ReferenceOracle::new()
        .evaluate(&context)
        .expect("oracle result");

    assert!(result.execution_path().is_reference_oracle());
    assert_eq!(result.execution_path(), &ExecutionPath::reference_oracle());
}

#[test]
fn satisfied_maps_correctly() {
    let result = ReferenceOracle::new()
        .evaluate(&scenario_context(
            "alpha",
            "alpha",
            "claim-001",
            "claim-001",
        ))
        .expect("oracle result");

    assert_eq!(result.outcome(), Outcome::Satisfied);
}

#[test]
fn not_satisfied_maps_correctly() {
    let result = ReferenceOracle::new()
        .evaluate(&scenario_context("alpha", "beta", "claim-001", "claim-001"))
        .expect("oracle result");

    assert_eq!(result.outcome(), Outcome::NotSatisfied);
}

#[test]
fn indeterminate_maps_correctly() {
    let result = ReferenceOracle::new()
        .evaluate(&scenario_context("alpha", "", "claim-001", "claim-001"))
        .expect("oracle result");

    assert_eq!(result.outcome(), Outcome::Indeterminate);
}

#[test]
fn claim_id_preserved() {
    let context = scenario_context("alpha", "alpha", "claim-042", "claim-042");
    let result = ReferenceOracle::new()
        .evaluate(&context)
        .expect("oracle result");

    assert_eq!(result.evaluated_claim_id().as_str(), "claim-042");
    assert_eq!(
        result.evaluated_claim_id().as_str(),
        context.claim().id.as_str()
    );
}

#[test]
fn specification_binding_preserved() {
    let context = scenario_context("alpha", "alpha", "claim-001", "claim-001");
    let result = ReferenceOracle::new()
        .evaluate(&context)
        .expect("oracle result");

    assert_eq!(
        result.specification_binding().edition_id(),
        context.specification_binding().edition_id()
    );
    assert_eq!(
        result.specification_binding().protocol_version(),
        context.specification_binding().protocol_version()
    );
}

#[test]
fn oracle_does_not_require_filesystem_paths() {
    let context = scenario_context("alpha", "alpha", "claim-001", "claim-001");
    let result = ReferenceOracle::new()
        .evaluate(&context)
        .expect("oracle result");

    assert!(!std::any::type_name::<ScenarioContext>().contains("PathBuf"));
    assert!(!std::any::type_name::<ReferenceOracle>().contains("PathBuf"));
    assert!(!std::any::type_name::<ComparableResult>().contains("PathBuf"));
    assert_eq!(result.outcome(), Outcome::Satisfied);
}
