//! Stub adapter tests for Milestone C.1.

use std::path::PathBuf;

use vp_conformance_adapter::StubAdapter;
use vp_conformance_core::{
    ExecutionPath, ImplementationAdapter, ScenarioBinding, ScenarioContext, ScenarioId,
};
use vp_conformance_scenarios::{ScenarioLoadOptions, ScenarioLoader};
use vp_reference_model::{Assertion, Claim, Evidence, EvidenceContent, Outcome};

fn sample_context() -> ScenarioContext {
    let claim = Claim::builder()
        .id("claim-001")
        .subject("subject")
        .assertion(Assertion::new("minimal", "body"))
        .build()
        .expect("claim");
    let evidence = Evidence::builder()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "body"))
        .build()
        .expect("evidence");
    let binding = ScenarioBinding::builder()
        .edition_id("edition-2026")
        .build()
        .expect("binding");

    ScenarioContext::builder()
        .scenario_id(ScenarioId::new("VP-CS-0001"))
        .specification_binding(binding)
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect("context")
}

fn loaded_context() -> ScenarioContext {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../vp-conformance-scenarios/tests/fixtures/minimal.toml");
    ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture))
        .expect("loaded scenario")
}

#[test]
fn stub_adapter_returns_comparable_result() {
    let adapter = StubAdapter::new("test-adapter", Outcome::Satisfied);
    let result = adapter.run(&sample_context()).expect("adapter result");

    assert_eq!(result.outcome(), Outcome::Satisfied);
}

#[test]
fn result_path_is_implementation_adapter_with_expected_id() {
    let adapter = StubAdapter::new("acme-wallet", Outcome::Satisfied);
    let result = adapter.run(&sample_context()).expect("adapter result");

    match result.execution_path() {
        ExecutionPath::ImplementationAdapter { adapter_id } => {
            assert_eq!(adapter_id, "acme-wallet");
        }
        ExecutionPath::ReferenceOracle => panic!("expected implementation adapter path"),
    }
    assert_eq!(adapter.id().as_str(), "acme-wallet");
}

#[test]
fn adapter_uses_scenario_claim_id() {
    let adapter = StubAdapter::new("stub", Outcome::Satisfied);
    let context = loaded_context();
    let result = adapter.run(&context).expect("adapter result");

    assert_eq!(
        result.evaluated_claim_id().as_str(),
        context.claim().id.as_str()
    );
}

#[test]
fn adapter_uses_scenario_binding() {
    let adapter = StubAdapter::new("stub", Outcome::Satisfied);
    let context = loaded_context();
    let result = adapter.run(&context).expect("adapter result");

    assert_eq!(
        result.specification_binding().edition_id(),
        context.specification_binding().edition_id()
    );
}

#[test]
fn adapter_can_return_satisfied() {
    let result = StubAdapter::new("stub", Outcome::Satisfied)
        .run(&sample_context())
        .expect("result");
    assert_eq!(result.outcome(), Outcome::Satisfied);
}

#[test]
fn adapter_can_return_not_satisfied() {
    let result = StubAdapter::new("stub", Outcome::NotSatisfied)
        .run(&sample_context())
        .expect("result");
    assert_eq!(result.outcome(), Outcome::NotSatisfied);
}

#[test]
fn adapter_can_return_indeterminate() {
    let result = StubAdapter::new("stub", Outcome::Indeterminate)
        .run(&sample_context())
        .expect("result");
    assert_eq!(result.outcome(), Outcome::Indeterminate);
}

#[test]
fn adapter_does_not_require_filesystem_paths() {
    let context = sample_context();
    let result = StubAdapter::new("stub", Outcome::Satisfied)
        .run(&context)
        .expect("result");

    assert!(!std::any::type_name::<ScenarioContext>().contains("PathBuf"));
    assert!(!std::any::type_name::<StubAdapter>().contains("PathBuf"));
    assert_eq!(result.evaluated_claim_id().as_str(), "claim-001");
}
