//! Workspace integration tests.

use vp_conformance_adapter::StubAdapter;
use vp_conformance_core::{
    ComparableResult, ConformanceError, ConformanceResult, ConformanceVerdict, ExecutionPath,
    ImplementationAdapter, ScenarioBinding, ScenarioContext, ScenarioId,
};
use vp_conformance_report::Report;
use vp_conformance_runner::{ComparisonEngine, ConformanceRunner, ReferenceOracle};
use vp_conformance_scenarios::ScenarioLoader;
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
        .specification_binding(binding.clone())
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect("context")
}

#[test]
fn workspace_crates_are_linkable() {
    let context = sample_context();
    let binding = context.specification_binding().clone();
    let adapter = StubAdapter::placeholder();

    let oracle = ComparableResult::builder()
        .execution_path(ExecutionPath::reference_oracle())
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .specification_binding(binding.clone())
        .build()
        .expect("oracle");
    let implementation = ComparableResult::builder()
        .execution_path(ExecutionPath::implementation_adapter("stub"))
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .specification_binding(binding.clone())
        .build()
        .expect("implementation");
    let result = ConformanceResult::builder()
        .scenario_id(context.scenario_id().clone())
        .specification_binding(binding)
        .verdict(ConformanceVerdict::Pass)
        .oracle_result(oracle)
        .implementation_result(implementation)
        .build()
        .expect("result");

    let _ = (
        context,
        result,
        ConformanceError::placeholder(),
        adapter.adapter_id(),
        adapter.accepts(&sample_context()),
        ScenarioLoader::placeholder(),
        ReferenceOracle::placeholder(),
        ComparisonEngine::placeholder(),
        ConformanceRunner::placeholder(),
        Report::placeholder(),
    );
}

#[test]
fn workspace_runner_reports_bootstrapped() {
    let runner = vp_conformance_runner::ConformanceRunner::placeholder();
    assert!(runner.is_bootstrapped());
}
