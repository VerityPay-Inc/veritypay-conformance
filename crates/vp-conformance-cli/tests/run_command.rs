//! CLI integration tests for Milestone G.1.

use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use vp_conformance_cli::{
    exit_code_from_report, parse_adapter_outcome, run_scenario, OutputFormat, RunOptions,
};
use vp_conformance_cli::{EXIT_CONFORMANCE_FAILURE, EXIT_SUCCESS, EXIT_USER_ERROR};
use vp_reference_model::Outcome;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../vp-conformance-scenarios/tests/fixtures")
        .join(name)
}

fn sibling_spec_vp_cs_0001_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml")
}

fn bin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_vp-conformance"))
}

fn run_cli(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(bin_path())
        .args(args)
        .output()
        .expect("run cli");

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    (output.status.code().unwrap_or(-1), stdout, stderr)
}

#[test]
fn matching_adapter_outcome_exits_zero() {
    let scenario = fixture_path("minimal.toml");
    let (code, stdout, stderr) = run_cli(&[
        "run",
        "--scenario",
        scenario.to_str().expect("path"),
        "--adapter-outcome",
        "satisfied",
    ]);

    assert_eq!(code, EXIT_SUCCESS);
    assert!(stdout.contains("Conformance Report"));
    assert!(stdout.contains("✓ VP-CS-0001"));
    assert!(stderr.is_empty());
}

#[test]
fn mismatched_adapter_outcome_exits_one() {
    let scenario = fixture_path("minimal.toml");
    let (code, stdout, stderr) = run_cli(&[
        "run",
        "--scenario",
        scenario.to_str().expect("path"),
        "--adapter-outcome",
        "not_satisfied",
    ]);

    assert_eq!(code, EXIT_CONFORMANCE_FAILURE);
    assert!(stdout.contains("✗ VP-CS-0001"));
    assert!(stdout.contains("Outcome mismatch"));
    assert!(stderr.is_empty());
}

#[test]
fn json_format_returns_valid_json() {
    let scenario = fixture_path("minimal.toml");
    let (code, stdout, stderr) = run_cli(&[
        "run",
        "--scenario",
        scenario.to_str().expect("path"),
        "--adapter-outcome",
        "satisfied",
        "--format",
        "json",
    ]);

    assert_eq!(code, EXIT_SUCCESS);
    assert!(stderr.is_empty());

    let value: Value = serde_json::from_str(&stdout).expect("valid json");
    assert_eq!(value["summary"]["passed"], 1);
    assert_eq!(value["results"][0]["scenario_id"], "VP-CS-0001");
    assert_eq!(value["results"][0]["verdict"], "pass");
}

#[test]
fn invalid_adapter_outcome_exits_two() {
    let scenario = fixture_path("minimal.toml");
    let (code, _stdout, stderr) = run_cli(&[
        "run",
        "--scenario",
        scenario.to_str().expect("path"),
        "--adapter-outcome",
        "bad_outcome",
    ]);

    assert_eq!(code, EXIT_USER_ERROR);
    assert!(stderr.contains("invalid outcome"));
}

#[test]
fn missing_scenario_file_exits_two() {
    let missing = fixture_path("does-not-exist.toml");
    let (code, _stdout, stderr) = run_cli(&["run", "--scenario", missing.to_str().expect("path")]);

    assert_eq!(code, EXIT_USER_ERROR);
    assert!(stderr.contains("failed to read fixture"));
}

#[test]
fn run_scenario_wires_existing_pipeline_components() {
    let output = run_scenario(&RunOptions::new(
        fixture_path("minimal.toml"),
        "stub",
        Outcome::Satisfied,
        OutputFormat::Human,
    ))
    .expect("pipeline run");

    assert_eq!(output.report().total(), 1);
    assert_eq!(output.report().passed(), 1);
    assert!(output.rendered().contains("Conformance Report"));
    assert_eq!(exit_code_from_report(output.report()), EXIT_SUCCESS);
}

#[test]
fn parse_adapter_outcome_accepts_normative_labels() {
    assert_eq!(
        parse_adapter_outcome("satisfied").expect("satisfied"),
        Outcome::Satisfied
    );
    assert_eq!(
        parse_adapter_outcome("not_satisfied").expect("not_satisfied"),
        Outcome::NotSatisfied
    );
    assert_eq!(
        parse_adapter_outcome("indeterminate").expect("indeterminate"),
        Outcome::Indeterminate
    );
}

#[test]
fn optional_sibling_spec_vp_cs_0001_smoke_passes_when_present() {
    let scenario = sibling_spec_vp_cs_0001_path();
    if !scenario.is_file() {
        eprintln!("skipping: sibling {} not found", scenario.display());
        return;
    }

    let output = run_scenario(&RunOptions::new(
        scenario,
        "stub",
        Outcome::Satisfied,
        OutputFormat::Human,
    ))
    .expect("spec VP-CS-0001 smoke run");

    assert_eq!(output.report().passed(), 1);
    assert!(output.rendered().contains("✓ VP-CS-0001"));
    assert_eq!(exit_code_from_report(output.report()), EXIT_SUCCESS);
}
