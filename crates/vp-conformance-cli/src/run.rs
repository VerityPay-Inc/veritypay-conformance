//! Single-scenario conformance run wiring for the CLI.

use std::path::PathBuf;

use vp_conformance_adapter::StubAdapter;
use vp_conformance_report::{
    ConformanceReport, HumanReportRenderer, JsonReportRenderer, ReportRenderError,
};
use vp_conformance_runner::{ComparisonEngine, ConformanceRunner, ReferenceOracle};
use vp_conformance_scenarios::{ScenarioLoadError, ScenarioLoadOptions, ScenarioLoader};
use vp_reference_model::Outcome;

/// Output format for a conformance run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Json,
}

impl OutputFormat {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Json => "json",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "human" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            other => Err(format!(
                "invalid format `{other}`; expected human or json"
            )),
        }
    }
}

/// Inputs for a single-scenario conformance run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOptions {
    scenario_path: PathBuf,
    adapter: String,
    adapter_outcome: Outcome,
    format: OutputFormat,
}

impl RunOptions {
    #[must_use]
    pub fn new(
        scenario_path: impl Into<PathBuf>,
        adapter: impl Into<String>,
        adapter_outcome: Outcome,
        format: OutputFormat,
    ) -> Self {
        Self {
            scenario_path: scenario_path.into(),
            adapter: adapter.into(),
            adapter_outcome,
            format,
        }
    }

    #[must_use]
    pub fn scenario_path(&self) -> &PathBuf {
        &self.scenario_path
    }

    #[must_use]
    pub fn adapter(&self) -> &str {
        &self.adapter
    }

    #[must_use]
    pub fn adapter_outcome(&self) -> Outcome {
        self.adapter_outcome
    }

    #[must_use]
    pub fn format(&self) -> OutputFormat {
        self.format
    }
}

/// Successful CLI run output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutput {
    rendered: String,
    report: ConformanceReport,
}

impl RunOutput {
    #[must_use]
    pub fn rendered(&self) -> &str {
        &self.rendered
    }

    #[must_use]
    pub fn report(&self) -> &ConformanceReport {
        &self.report
    }
}

/// CLI failure classification mapped to process exit codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    User(String),
    Harness(String),
}

impl RunError {
    #[must_use]
    pub fn user(message: impl Into<String>) -> Self {
        Self::User(message.into())
    }

    #[must_use]
    pub fn harness(message: impl Into<String>) -> Self {
        Self::Harness(message.into())
    }

    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::User(message) | Self::Harness(message) => message,
        }
    }
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for RunError {}

pub fn parse_adapter_outcome(value: &str) -> Result<Outcome, String> {
    match value {
        "satisfied" => Ok(Outcome::Satisfied),
        "not_satisfied" => Ok(Outcome::NotSatisfied),
        "indeterminate" => Ok(Outcome::Indeterminate),
        other => Err(format!(
            "invalid outcome `{other}`; expected satisfied, not_satisfied, or indeterminate"
        )),
    }
}

/// Executes the conformance pipeline for one scenario fixture.
pub fn run_scenario(options: &RunOptions) -> Result<RunOutput, RunError> {
    if options.adapter() != "stub" {
        return Err(RunError::user(format!(
            "unsupported adapter `{}`; only stub is available",
            options.adapter()
        )));
    }

    let context = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(options.scenario_path()))
        .map_err(map_load_error)?;

    let adapter = StubAdapter::new("stub", options.adapter_outcome());
    let runner = ConformanceRunner::new(ReferenceOracle::new(), Box::new(adapter));
    let runner_result = runner
        .run(&context)
        .map_err(|error| RunError::harness(error.to_string()))?;

    let conformance_result = ComparisonEngine::new()
        .compare(context.scenario_id(), &runner_result)
        .map_err(|error| RunError::harness(error.to_string()))?;

    let report = ConformanceReport::from_results([conformance_result]);
    let rendered = render_report(&report, options.format())?;

    Ok(RunOutput { rendered, report })
}

fn render_report(report: &ConformanceReport, format: OutputFormat) -> Result<String, RunError> {
    match format {
        OutputFormat::Human => Ok(HumanReportRenderer::new().render(report)),
        OutputFormat::Json => JsonReportRenderer::new()
            .render(report)
            .map_err(map_render_error),
    }
}

fn map_load_error(error: ScenarioLoadError) -> RunError {
    RunError::user(error.to_string())
}

fn map_render_error(error: ReportRenderError) -> RunError {
    RunError::harness(error.message().to_owned())
}
