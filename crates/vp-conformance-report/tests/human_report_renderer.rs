//! Human report renderer tests for Milestone F.2.

use vp_conformance_core::{
    ComparableResult, ConformanceResult, ConformanceVerdict, ExecutionPath, ScenarioBinding,
};
use vp_conformance_report::{ConformanceReport, HumanReportRenderer};
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

fn conformance_result_with_notes(
    scenario_id: &str,
    verdict: ConformanceVerdict,
    notes: &[(&str, &str)],
) -> ConformanceResult {
    let binding = sample_binding();
    let mut builder = ConformanceResult::builder()
        .scenario_id(scenario_id)
        .specification_binding(binding.clone())
        .verdict(verdict)
        .oracle_result(oracle_result(&binding))
        .implementation_result(implementation_result(&binding));

    for (key, value) in notes {
        builder = builder.comparison_note(*key, *value);
    }

    builder.build().expect("conformance result")
}

#[test]
fn empty_report_renders_summary_and_empty_results() {
    let rendered = HumanReportRenderer::new().render(&ConformanceReport::from_results([]));

    assert_eq!(
        rendered,
        "\
Conformance Report

Summary
-------
Total: 0
Passed: 0
Failed: 0
Skipped: 0
Errors: 0
Success Rate: 0.0%

Results

"
    );
}

#[test]
fn single_pass_renders_checkmark_line() {
    let report = ConformanceReport::from_results([conformance_result(
        "VP-CS-0001",
        ConformanceVerdict::Pass,
    )]);
    let rendered = HumanReportRenderer::new().render(&report);

    assert!(rendered.contains("Success Rate: 100.0%\n"));
    assert!(rendered.ends_with("Results\n\n✓ VP-CS-0001\n"));
}

#[test]
fn single_fail_renders_mismatch_notes() {
    let report = ConformanceReport::from_results([conformance_result_with_notes(
        "VP-CS-0003",
        ConformanceVerdict::Fail,
        &[
            ("outcome.mismatch", "oracle=satisfied implementation=not_satisfied"),
            (
                "specification_binding.mismatch",
                "oracle=edition=edition-2026 implementation=edition=edition-2025",
            ),
        ],
    )]);
    let rendered = HumanReportRenderer::new().render(&report);

    assert!(rendered.contains("Success Rate: 0.0%\n"));
    assert!(rendered.contains("✗ VP-CS-0003\n\nOutcome mismatch\nSpecification binding mismatch\n"));
}

#[test]
fn mixed_report_preserves_order_and_formats_success_rate() {
    let report = ConformanceReport::builder()
        .result(conformance_result("VP-CS-0001", ConformanceVerdict::Pass))
        .result(conformance_result("VP-CS-0002", ConformanceVerdict::Pass))
        .result(conformance_result_with_notes(
            "VP-CS-0003",
            ConformanceVerdict::Fail,
            &[("outcome.mismatch", "oracle=satisfied implementation=not_satisfied")],
        ))
        .result(conformance_result("VP-CS-0004", ConformanceVerdict::Skip))
        .result(conformance_result("VP-CS-0005", ConformanceVerdict::Error))
        .build();
    let rendered = HumanReportRenderer::new().render(&report);

    assert!(rendered.contains("Total: 5\n"));
    assert!(rendered.contains("Success Rate: 40.0%\n"));
    assert_eq!(
        rendered,
        "\
Conformance Report

Summary
-------
Total: 5
Passed: 2
Failed: 1
Skipped: 1
Errors: 1
Success Rate: 40.0%

Results

✓ VP-CS-0001
✓ VP-CS-0002
✗ VP-CS-0003

Outcome mismatch
- VP-CS-0004
Skipped
! VP-CS-0005
Error
"
    );
}

#[test]
fn render_output_is_deterministic() {
    let report = ConformanceReport::builder()
        .result(conformance_result("VP-CS-0001", ConformanceVerdict::Pass))
        .result(conformance_result_with_notes(
            "VP-CS-0002",
            ConformanceVerdict::Fail,
            &[(
                "evaluated_claim_id.mismatch",
                "oracle=claim-001 implementation=claim-002",
            )],
        ))
        .build();
    let renderer = HumanReportRenderer::new();

    let first = renderer.render(&report);
    let second = renderer.render(&report);

    assert_eq!(first, second);
}

#[test]
fn success_rate_formats_one_decimal_place() {
    let report = ConformanceReport::builder()
        .result(conformance_result("VP-CS-0001", ConformanceVerdict::Pass))
        .result(conformance_result("VP-CS-0002", ConformanceVerdict::Pass))
        .result(conformance_result("VP-CS-0003", ConformanceVerdict::Pass))
        .result(conformance_result("VP-CS-0004", ConformanceVerdict::Fail))
        .build();
    let rendered = HumanReportRenderer::new().render(&report);

    assert!(rendered.contains("Success Rate: 75.0%\n"));
}
