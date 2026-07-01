//! Domain type tests for Milestone B.1.

use vp_conformance_core::{
    BuildError, ComparableResult, ComparableResultBuilder, ConformanceResult, ConformanceVerdict,
    ExecutionPath, ScenarioBinding, ScenarioBindingBuilder, ScenarioContext,
    ScenarioContextBuilder, ScenarioId,
};
use vp_reference_model::{Assertion, Claim, Evidence, EvidenceContent, Outcome};

fn sample_binding() -> ScenarioBinding {
    ScenarioBinding::builder()
        .edition_id("edition-2026")
        .build()
        .expect("binding")
}

fn sample_claim() -> Claim {
    Claim::builder()
        .id("claim-001")
        .subject("subject")
        .assertion(Assertion::new("minimal", "body"))
        .build()
        .expect("claim")
}

fn sample_evidence(claim: &Claim) -> Evidence {
    Evidence::builder()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "body"))
        .build()
        .expect("evidence")
}

fn sample_context() -> ScenarioContext {
    let claim = sample_claim();
    let evidence = sample_evidence(&claim);

    ScenarioContext::builder()
        .scenario_id(ScenarioId::new("VP-CS-0001"))
        .specification_binding(sample_binding())
        .claim(claim)
        .evidence(evidence)
        .metadata_entry("fixture", "manual")
        .build()
        .expect("scenario context")
}

fn reference_result(binding: &ScenarioBinding) -> ComparableResult {
    ComparableResult::builder()
        .execution_path(ExecutionPath::reference_oracle())
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .specification_binding(binding.clone())
        .build()
        .expect("oracle result")
}

fn implementation_result(binding: &ScenarioBinding) -> ComparableResult {
    ComparableResult::builder()
        .execution_path(ExecutionPath::implementation_adapter("stub"))
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .specification_binding(binding.clone())
        .build()
        .expect("implementation result")
}

#[test]
fn scenario_context_builder_succeeds_with_required_fields() {
    let context = sample_context();

    assert_eq!(context.scenario_id().as_str(), "VP-CS-0001");
    assert_eq!(
        context.specification_binding().edition_id(),
        Some("edition-2026")
    );
    assert_eq!(context.claim().id.as_str(), "claim-001");
    assert_eq!(context.evidence().id.as_str(), "evidence-001");
    assert_eq!(context.metadata().get("fixture"), Some("manual"));
}

#[test]
fn scenario_context_builder_reports_missing_required_fields() {
    let error = ScenarioContextBuilder::new()
        .build()
        .expect_err("missing fields");

    assert_eq!(error, BuildError::missing("scenario_id"));

    let claim = sample_claim();
    let evidence = sample_evidence(&claim);
    let error = ScenarioContextBuilder::new()
        .scenario_id("VP-CS-0001")
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect_err("missing binding");

    assert_eq!(error, BuildError::missing("specification_binding"));
}

#[test]
fn scenario_binding_requires_at_least_one_pin() {
    let error = ScenarioBindingBuilder::new()
        .build()
        .expect_err("empty binding");

    assert_eq!(
        error,
        BuildError::invalid(
            "specification_binding",
            "at least one of edition_id or protocol_version is required"
        )
    );
}

#[test]
fn comparable_result_represents_reference_path() {
    let binding = sample_binding();
    let result = reference_result(&binding);

    assert!(result.execution_path().is_reference_oracle());
    assert_eq!(result.outcome(), Outcome::Satisfied);
    assert_eq!(result.evaluated_claim_id().as_str(), "claim-001");
}

#[test]
fn comparable_result_represents_implementation_path() {
    let binding = sample_binding();
    let result = implementation_result(&binding);

    assert!(result.execution_path().is_implementation_adapter());
    match result.execution_path() {
        ExecutionPath::ImplementationAdapter { adapter_id } => {
            assert_eq!(adapter_id, "stub");
        }
        ExecutionPath::ReferenceOracle => panic!("expected implementation adapter path"),
    }
}

#[test]
fn comparable_result_builder_reports_missing_execution_path() {
    let error = ComparableResultBuilder::new()
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .specification_binding(sample_binding())
        .build()
        .expect_err("missing execution path");

    assert_eq!(error, BuildError::missing("execution_path"));
}

#[test]
fn conformance_result_builder_supports_pass_fail_skip_error() {
    let binding = sample_binding();
    let oracle = reference_result(&binding);
    let implementation = implementation_result(&binding);

    for verdict in [
        ConformanceVerdict::Pass,
        ConformanceVerdict::Fail,
        ConformanceVerdict::Skip,
        ConformanceVerdict::Error,
    ] {
        let result = ConformanceResult::builder()
            .scenario_id("VP-CS-0001")
            .specification_binding(binding.clone())
            .verdict(verdict)
            .oracle_result(oracle.clone())
            .implementation_result(implementation.clone())
            .comparison_note("verdict", verdict.as_str())
            .build()
            .expect("conformance result");

        assert_eq!(result.verdict(), verdict);
        assert_eq!(
            result.comparison_notes().get("verdict"),
            Some(verdict.as_str())
        );
    }
}

#[test]
fn conformance_result_builder_rejects_mismatched_execution_paths() {
    let binding = sample_binding();
    let oracle = implementation_result(&binding);
    let implementation = implementation_result(&binding);

    let error = ConformanceResult::builder()
        .scenario_id("VP-CS-0001")
        .specification_binding(binding)
        .verdict(ConformanceVerdict::Fail)
        .oracle_result(oracle)
        .implementation_result(implementation)
        .build()
        .expect_err("oracle path mismatch");

    assert_eq!(
        error,
        BuildError::invalid("oracle_result", "execution_path must be ReferenceOracle")
    );
}

#[test]
fn core_objects_construct_without_filesystem_paths() {
    let context = sample_context();
    let binding = context.specification_binding().clone();
    let oracle = reference_result(&binding);
    let implementation = implementation_result(&binding);
    let result = ConformanceResult::builder()
        .scenario_id(context.scenario_id().clone())
        .specification_binding(binding)
        .verdict(ConformanceVerdict::Pass)
        .oracle_result(oracle)
        .implementation_result(implementation)
        .build()
        .expect("conformance result");

    assert!(!std::any::type_name::<ScenarioContext>().contains("PathBuf"));
    assert!(!std::any::type_name::<ComparableResult>().contains("PathBuf"));
    assert!(!std::any::type_name::<ConformanceResult>().contains("PathBuf"));
    assert_eq!(result.scenario_id().as_str(), "VP-CS-0001");
}
