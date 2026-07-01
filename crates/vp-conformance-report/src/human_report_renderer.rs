//! Human-readable rendering of conformance reports.

use vp_conformance_core::{ConformanceResult, ConformanceVerdict};

use crate::conformance_report::ConformanceReport;

const MISMATCH_NOTE_ORDER: [(&str, &str); 3] = [
    ("outcome.mismatch", "Outcome mismatch"),
    (
        "evaluated_claim_id.mismatch",
        "Claim id mismatch",
    ),
    (
        "specification_binding.mismatch",
        "Specification binding mismatch",
    ),
];

/// Renders a [`ConformanceReport`] as a deterministic plain-text summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HumanReportRenderer;

impl HumanReportRenderer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn render(&self, report: &ConformanceReport) -> String {
        let mut output = String::new();

        output.push_str("Conformance Report\n");
        output.push('\n');
        output.push_str("Summary\n");
        output.push_str("-------\n");
        output.push_str(&format!("Total: {}\n", report.total()));
        output.push_str(&format!("Passed: {}\n", report.passed()));
        output.push_str(&format!("Failed: {}\n", report.failed()));
        output.push_str(&format!("Skipped: {}\n", report.skipped()));
        output.push_str(&format!("Errors: {}\n", report.errors()));
        output.push_str(&format!(
            "Success Rate: {}\n",
            format_success_rate(report)
        ));
        output.push('\n');
        output.push_str("Results\n");
        output.push('\n');

        for result in report.results() {
            render_result(&mut output, result);
        }

        output
    }
}

fn format_success_rate(report: &ConformanceReport) -> String {
    format!("{:.1}%", report.success_rate() * 100.0)
}

fn verdict_symbol(verdict: ConformanceVerdict) -> &'static str {
    match verdict {
        ConformanceVerdict::Pass => "✓",
        ConformanceVerdict::Fail => "✗",
        ConformanceVerdict::Skip => "-",
        ConformanceVerdict::Error => "!",
    }
}

fn render_result(output: &mut String, result: &ConformanceResult) {
    output.push_str(&format!(
        "{} {}\n",
        verdict_symbol(result.verdict()),
        result.scenario_id().as_str()
    ));

    match result.verdict() {
        ConformanceVerdict::Fail => render_fail_notes(output, result),
        ConformanceVerdict::Skip => output.push_str("Skipped\n"),
        ConformanceVerdict::Error => output.push_str("Error\n"),
        ConformanceVerdict::Pass => {}
    }
}

fn render_fail_notes(output: &mut String, result: &ConformanceResult) {
    let notes = result.comparison_notes();
    let mut mismatch_lines = Vec::new();

    for (key, label) in MISMATCH_NOTE_ORDER {
        if notes.get(key).is_some() {
            mismatch_lines.push(label);
        }
    }

    if mismatch_lines.is_empty() {
        return;
    }

    output.push('\n');
    for line in mismatch_lines {
        output.push_str(line);
        output.push('\n');
    }
}
