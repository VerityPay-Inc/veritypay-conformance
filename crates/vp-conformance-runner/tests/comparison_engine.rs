//! Comparison engine tests for Milestone E.1.

use vp_conformance_core::{
    ComparableResult, ConformanceVerdict, ExecutionPath, ScenarioBinding, ScenarioId,
};
use vp_conformance_runner::{ComparisonEngine, RunnerResult};
use vp_reference_model::Outcome;

fn sample_binding() -> ScenarioBinding {
    ScenarioBinding::builder()
        .edition_id("edition-2026")
        .build()
        .expect("binding")
}

fn alternate_binding() -> ScenarioBinding {
    ScenarioBinding::builder()
        .edition_id("edition-2025")
        .build()
        .expect("binding")
}

fn oracle_result(
    binding: &ScenarioBinding,
    claim_id: &str,
    outcome: Outcome,
) -> ComparableResult {
    ComparableResult::builder()
        .execution_path(ExecutionPath::reference_oracle())
        .evaluated_claim_id(claim_id)
        .outcome(outcome)
        .specification_binding(binding.clone())
        .build()
        .expect("oracle result")
}

fn implementation_result(
    binding: &ScenarioBinding,
    claim_id: &str,
    outcome: Outcome,
) -> ComparableResult {
    ComparableResult::builder()
        .execution_path(ExecutionPath::implementation_adapter("stub"))
        .evaluated_claim_id(claim_id)
        .outcome(outcome)
        .specification_binding(binding.clone())
        .build()
        .expect("implementation result")
}

fn runner_result(oracle: ComparableResult, implementation: ComparableResult) -> RunnerResult {
    RunnerResult::new(oracle, implementation)
}

#[test]
fn matching_results_produce_pass() {
    let binding = sample_binding();
    let oracle = oracle_result(&binding, "claim-001", Outcome::Satisfied);
    let implementation = implementation_result(&binding, "claim-001", Outcome::Satisfied);
    let before_oracle = oracle.clone();
    let before_implementation = implementation.clone();
    let runner = runner_result(oracle, implementation);

    let result = ComparisonEngine::new()
        .compare(&ScenarioId::new("VP-CS-0001"), &runner)
        .expect("conformance result");

    assert_eq!(result.verdict(), ConformanceVerdict::Pass);
    assert_eq!(
        result.comparison_notes().get("comparison"),
        Some("pass: all compared fields match")
    );
    assert!(result.comparison_notes().get("outcome.mismatch").is_none());
    assert_eq!(runner.oracle_result(), &before_oracle);
    assert_eq!(runner.implementation_result(), &before_implementation);
}

#[test]
fn outcome_mismatch_produces_fail() {
    let binding = sample_binding();
    let runner = runner_result(
        oracle_result(&binding, "claim-001", Outcome::Satisfied),
        implementation_result(&binding, "claim-001", Outcome::NotSatisfied),
    );

    let result = ComparisonEngine::new()
        .compare(&ScenarioId::new("VP-CS-0001"), &runner)
        .expect("conformance result");

    assert_eq!(result.verdict(), ConformanceVerdict::Fail);
    let note = result
        .comparison_notes()
        .get("outcome.mismatch")
        .expect("outcome mismatch note");
    assert!(note.contains("oracle=satisfied"));
    assert!(note.contains("implementation=not_satisfied"));
}

#[test]
fn claim_id_mismatch_produces_fail() {
    let binding = sample_binding();
    let runner = runner_result(
        oracle_result(&binding, "claim-001", Outcome::Satisfied),
        implementation_result(&binding, "claim-002", Outcome::Satisfied),
    );

    let result = ComparisonEngine::new()
        .compare(&ScenarioId::new("VP-CS-0001"), &runner)
        .expect("conformance result");

    assert_eq!(result.verdict(), ConformanceVerdict::Fail);
    let note = result
        .comparison_notes()
        .get("evaluated_claim_id.mismatch")
        .expect("claim id mismatch note");
    assert!(note.contains("oracle=claim-001"));
    assert!(note.contains("implementation=claim-002"));
}

#[test]
fn specification_binding_mismatch_produces_fail() {
    let oracle_binding = sample_binding();
    let implementation_binding = alternate_binding();
    let runner = runner_result(
        oracle_result(&oracle_binding, "claim-001", Outcome::Satisfied),
        implementation_result(&implementation_binding, "claim-001", Outcome::Satisfied),
    );

    let result = ComparisonEngine::new()
        .compare(&ScenarioId::new("VP-CS-0001"), &runner)
        .expect("conformance result");

    assert_eq!(result.verdict(), ConformanceVerdict::Fail);
    let note = result
        .comparison_notes()
        .get("specification_binding.mismatch")
        .expect("binding mismatch note");
    assert!(note.contains("oracle=edition=edition-2026"));
    assert!(note.contains("implementation=edition=edition-2025"));
}

#[test]
fn multiple_mismatches_produce_multiple_notes() {
    let oracle_binding = sample_binding();
    let implementation_binding = alternate_binding();
    let runner = runner_result(
        oracle_result(&oracle_binding, "claim-001", Outcome::Satisfied),
        implementation_result(
            &implementation_binding,
            "claim-002",
            Outcome::Indeterminate,
        ),
    );

    let result = ComparisonEngine::new()
        .compare(&ScenarioId::new("VP-CS-0001"), &runner)
        .expect("conformance result");

    assert_eq!(result.verdict(), ConformanceVerdict::Fail);
    assert!(result.comparison_notes().get("outcome.mismatch").is_some());
    assert!(result
        .comparison_notes()
        .get("evaluated_claim_id.mismatch")
        .is_some());
    assert!(result
        .comparison_notes()
        .get("specification_binding.mismatch")
        .is_some());
}

#[test]
fn comparison_does_not_mutate_comparable_results() {
    let binding = sample_binding();
    let oracle = oracle_result(&binding, "claim-001", Outcome::Satisfied);
    let implementation = implementation_result(&binding, "claim-001", Outcome::NotSatisfied);
    let oracle_before = oracle.clone();
    let implementation_before = implementation.clone();
    let runner = runner_result(oracle, implementation);

    let _ = ComparisonEngine::new()
        .compare(&ScenarioId::new("VP-CS-0001"), &runner)
        .expect("conformance result");

    assert_eq!(runner.oracle_result(), &oracle_before);
    assert_eq!(runner.implementation_result(), &implementation_before);
}

#[test]
fn successful_comparison_never_produces_skip_or_error_verdict() {
    let binding = sample_binding();
    let matching = runner_result(
        oracle_result(&binding, "claim-001", Outcome::Satisfied),
        implementation_result(&binding, "claim-001", Outcome::Satisfied),
    );
    let mismatching = runner_result(
        oracle_result(&binding, "claim-001", Outcome::Satisfied),
        implementation_result(&binding, "claim-001", Outcome::NotSatisfied),
    );
    let engine = ComparisonEngine::new();
    let scenario_id = ScenarioId::new("VP-CS-0001");

    let pass = engine
        .compare(&scenario_id, &matching)
        .expect("pass result");
    let fail = engine
        .compare(&scenario_id, &mismatching)
        .expect("fail result");

    assert!(matches!(
        pass.verdict(),
        ConformanceVerdict::Pass | ConformanceVerdict::Fail
    ));
    assert!(matches!(
        fail.verdict(),
        ConformanceVerdict::Pass | ConformanceVerdict::Fail
    ));
    assert_ne!(pass.verdict(), ConformanceVerdict::Skip);
    assert_ne!(pass.verdict(), ConformanceVerdict::Error);
    assert_ne!(fail.verdict(), ConformanceVerdict::Skip);
    assert_ne!(fail.verdict(), ConformanceVerdict::Error);
}
