//! Conformance report tests for Milestone F.1.

use vp_conformance_core::{
    ComparableResult, ConformanceResult, ConformanceVerdict, ExecutionPath, ScenarioBinding,
};
use vp_conformance_report::ConformanceReport;
use vp_reference_model::Outcome;

fn sample_binding() -> ScenarioBinding {
    ScenarioBinding::builder()
        .edition_id("edition-2026")
        .build()
        .expect("binding")
}

fn oracle_result(binding: &ScenarioBinding) -> ComparableResult {
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

fn conformance_result(scenario_id: &str, verdict: ConformanceVerdict) -> ConformanceResult {
    let binding = sample_binding();
    ConformanceResult::builder()
        .scenario_id(scenario_id)
        .specification_binding(binding.clone())
        .verdict(verdict)
        .oracle_result(oracle_result(&binding))
        .implementation_result(implementation_result(&binding))
        .build()
        .expect("conformance result")
}

#[test]
fn empty_report_has_zero_counts_and_success_rate() {
    let report = ConformanceReport::from_results([]);

    assert_eq!(report.total(), 0);
    assert_eq!(report.passed(), 0);
    assert_eq!(report.failed(), 0);
    assert_eq!(report.skipped(), 0);
    assert_eq!(report.errors(), 0);
    assert!(report.results().is_empty());
    assert!(!report.has_failures());
    assert!(!report.has_errors());
    assert_eq!(report.success_rate(), 0.0);
}

#[test]
fn one_pass_report() {
    let result = conformance_result("VP-CS-0001", ConformanceVerdict::Pass);
    let report = ConformanceReport::builder().result(result).build();

    assert_eq!(report.total(), 1);
    assert_eq!(report.passed(), 1);
    assert_eq!(report.failed(), 0);
    assert_eq!(report.results().len(), 1);
    assert_eq!(report.results()[0].scenario_id().as_str(), "VP-CS-0001");
    assert!(!report.has_failures());
    assert!(!report.has_errors());
    assert_eq!(report.success_rate(), 1.0);
}

#[test]
fn one_fail_report() {
    let report = ConformanceReport::from_results([conformance_result(
        "VP-CS-0001",
        ConformanceVerdict::Fail,
    )]);

    assert_eq!(report.total(), 1);
    assert_eq!(report.passed(), 0);
    assert_eq!(report.failed(), 1);
    assert!(report.has_failures());
    assert!(!report.has_errors());
    assert_eq!(report.success_rate(), 0.0);
}

#[test]
fn mixed_results_report_counts_each_verdict() {
    let report = ConformanceReport::builder()
        .result(conformance_result("VP-CS-0001", ConformanceVerdict::Pass))
        .result(conformance_result("VP-CS-0002", ConformanceVerdict::Fail))
        .result(conformance_result("VP-CS-0003", ConformanceVerdict::Skip))
        .result(conformance_result("VP-CS-0004", ConformanceVerdict::Error))
        .build();

    assert_eq!(report.total(), 4);
    assert_eq!(report.passed(), 1);
    assert_eq!(report.failed(), 1);
    assert_eq!(report.skipped(), 1);
    assert_eq!(report.errors(), 1);
    assert!(report.has_failures());
    assert!(report.has_errors());
    assert_eq!(report.success_rate(), 0.25);
    assert_eq!(report.results().len(), 4);
}

#[test]
fn report_owns_cloned_results_without_mutation() {
    let original = conformance_result("VP-CS-0001", ConformanceVerdict::Pass);
    let report = ConformanceReport::from_results([original.clone()]);

    assert_eq!(report.results()[0], original);
    assert_eq!(original.verdict(), ConformanceVerdict::Pass);
}
