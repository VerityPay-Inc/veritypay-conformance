//! Machine-readable JSON rendering of conformance reports.

use serde::Serialize;
use vp_conformance_core::{ComparableResult, ConformanceResult, ExecutionPath, ScenarioBinding};

use crate::conformance_report::ConformanceReport;
use crate::report_render_error::ReportRenderError;

/// Renders a [`ConformanceReport`] as stable JSON for CI and automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JsonReportRenderer;

impl JsonReportRenderer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, report: &ConformanceReport) -> Result<String, ReportRenderError> {
        let document = JsonReportDocument::from_report(report);
        serde_json::to_string_pretty(&document).map_err(ReportRenderError::from)
    }
}

#[derive(Debug, Serialize)]
struct JsonReportDocument {
    summary: JsonSummary,
    results: Vec<JsonScenarioResult>,
}

#[derive(Debug, Serialize)]
struct JsonSummary {
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
    errors: usize,
    success_rate: f64,
}

#[derive(Debug, Serialize)]
struct JsonScenarioResult {
    scenario_id: String,
    verdict: &'static str,
    specification_binding: String,
    oracle: Option<JsonOracleResult>,
    implementation: Option<JsonImplementationResult>,
    notes: Vec<JsonNote>,
}

#[derive(Debug, Serialize)]
struct JsonOracleResult {
    outcome: &'static str,
    claim_id: String,
}

#[derive(Debug, Serialize)]
struct JsonImplementationResult {
    outcome: &'static str,
    claim_id: String,
    adapter_id: String,
}

#[derive(Debug, Serialize)]
struct JsonNote {
    key: String,
    value: String,
}

impl JsonReportDocument {
    fn from_report(report: &ConformanceReport) -> Self {
        Self {
            summary: JsonSummary {
                total: report.total(),
                passed: report.passed(),
                failed: report.failed(),
                skipped: report.skipped(),
                errors: report.errors(),
                success_rate: format_success_rate(report.success_rate()),
            },
            results: report
                .results()
                .iter()
                .map(JsonScenarioResult::from_conformance_result)
                .collect(),
        }
    }
}

impl JsonScenarioResult {
    fn from_conformance_result(result: &ConformanceResult) -> Self {
        Self {
            scenario_id: result.scenario_id().as_str().to_owned(),
            verdict: result.verdict().as_str(),
            specification_binding: binding_summary(result.specification_binding()),
            oracle: oracle_json(result),
            implementation: implementation_json(result),
            notes: notes_json(result),
        }
    }
}

fn oracle_json(result: &ConformanceResult) -> Option<JsonOracleResult> {
    let oracle = result.oracle_result();
    if !oracle.execution_path().is_reference_oracle() {
        return None;
    }

    Some(comparable_oracle_json(oracle))
}

fn implementation_json(result: &ConformanceResult) -> Option<JsonImplementationResult> {
    let implementation = result.implementation_result();
    let ExecutionPath::ImplementationAdapter { adapter_id } = implementation.execution_path()
    else {
        return None;
    };

    Some(JsonImplementationResult {
        outcome: implementation.outcome().as_str(),
        claim_id: implementation.evaluated_claim_id().as_str().to_owned(),
        adapter_id: adapter_id.clone(),
    })
}

fn comparable_oracle_json(oracle: &ComparableResult) -> JsonOracleResult {
    JsonOracleResult {
        outcome: oracle.outcome().as_str(),
        claim_id: oracle.evaluated_claim_id().as_str().to_owned(),
    }
}

fn notes_json(result: &ConformanceResult) -> Vec<JsonNote> {
    result
        .comparison_notes()
        .entries()
        .iter()
        .map(|(key, value)| JsonNote {
            key: key.clone(),
            value: value.clone(),
        })
        .collect()
}

fn binding_summary(binding: &ScenarioBinding) -> String {
    match (binding.edition_id(), binding.protocol_version()) {
        (Some(edition), Some(protocol)) => {
            format!("edition={edition} protocol={protocol}")
        }
        (Some(edition), None) => format!("edition={edition}"),
        (None, Some(protocol)) => format!("protocol={protocol}"),
        (None, None) => "unpinned".to_owned(),
    }
}

fn format_success_rate(rate: f64) -> f64 {
    (rate * 10_000.0).round() / 10_000.0
}
