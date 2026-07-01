//! JSON report renderer tests for Milestone F.3.

use serde_json::Value;
use vp_conformance_core::{
    ComparableResult, ConformanceResult, ConformanceVerdict, ExecutionPath, ScenarioBinding,
};
use vp_conformance_report::{ConformanceReport, JsonReportRenderer};
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

fn parse_json(rendered: &str) -> Value {
    serde_json::from_str(rendered).expect("valid json")
}

#[test]
fn empty_report_renders_zero_summary_and_empty_results() {
    let rendered = JsonReportRenderer::new()
        .render(&ConformanceReport::from_results([]))
        .expect("render json");
    let value = parse_json(&rendered);

    assert_eq!(value["summary"]["total"], 0);
    assert_eq!(value["summary"]["success_rate"], 0.0);
    assert_eq!(value["results"], Value::Array(vec![]));
}

#[test]
fn single_pass_renders_lowercase_verdict_and_outcomes() {
    let rendered = JsonReportRenderer::new()
        .render(&ConformanceReport::from_results([conformance_result(
            "VP-CS-0001",
            ConformanceVerdict::Pass,
        )]))
        .expect("render json");
    let value = parse_json(&rendered);

    assert_eq!(value["summary"]["total"], 1);
    assert_eq!(value["summary"]["passed"], 1);
    assert_eq!(value["summary"]["success_rate"], 1.0);
    assert_eq!(value["results"][0]["scenario_id"], "VP-CS-0001");
    assert_eq!(value["results"][0]["verdict"], "pass");
    assert_eq!(value["results"][0]["oracle"]["outcome"], "satisfied");
    assert_eq!(value["results"][0]["oracle"]["claim_id"], "claim-001");
    assert_eq!(
        value["results"][0]["implementation"]["outcome"],
        "satisfied"
    );
    assert_eq!(
        value["results"][0]["implementation"]["adapter_id"],
        "stub"
    );
    assert_eq!(value["results"][0]["notes"], Value::Array(vec![]));
}

#[test]
fn single_fail_includes_notes_and_binding_summary() {
    let rendered = JsonReportRenderer::new()
        .render(&ConformanceReport::from_results([conformance_result_with_notes(
            "VP-CS-0003",
            ConformanceVerdict::Fail,
            &[("outcome.mismatch", "oracle=satisfied implementation=not_satisfied")],
        )]))
        .expect("render json");
    let value = parse_json(&rendered);

    assert_eq!(value["summary"]["failed"], 1);
    assert_eq!(value["summary"]["success_rate"], 0.0);
    assert_eq!(value["results"][0]["verdict"], "fail");
    assert_eq!(
        value["results"][0]["specification_binding"],
        "edition=edition-2026"
    );
    assert_eq!(value["results"][0]["notes"][0]["key"], "outcome.mismatch");
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
        .build();
    let rendered = JsonReportRenderer::new()
        .render(&report)
        .expect("render json");
    let value = parse_json(&rendered);

    assert_eq!(value["summary"]["total"], 3);
    assert_eq!(value["summary"]["passed"], 2);
    assert_eq!(value["summary"]["failed"], 1);
    assert_eq!(value["summary"]["success_rate"], 0.6667);
    assert_eq!(value["results"][0]["scenario_id"], "VP-CS-0001");
    assert_eq!(value["results"][1]["scenario_id"], "VP-CS-0002");
    assert_eq!(value["results"][2]["scenario_id"], "VP-CS-0003");
    assert_eq!(value["results"][2]["verdict"], "fail");
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
    let renderer = JsonReportRenderer::new();

    let first = renderer.render(&report).expect("first render");
    let second = renderer.render(&report).expect("second render");

    assert_eq!(first, second);
}

#[test]
fn rendered_json_parses_with_serde_json() {
    let report = ConformanceReport::builder()
        .result(conformance_result("VP-CS-0001", ConformanceVerdict::Skip))
        .result(conformance_result("VP-CS-0002", ConformanceVerdict::Error))
        .build();
    let rendered = JsonReportRenderer::new()
        .render(&report)
        .expect("render json");

    let value: Value = serde_json::from_str(&rendered).expect("parse json");

    assert_eq!(value["results"][0]["verdict"], "skip");
    assert_eq!(value["results"][1]["verdict"], "error");
    assert!(!rendered.contains('✓'));
    assert!(!rendered.contains('✗'));
}
