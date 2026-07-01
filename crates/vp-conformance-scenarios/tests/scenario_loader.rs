//! Scenario loader tests for Milestone B.2.

use std::path::PathBuf;

use vp_conformance_scenarios::{ScenarioLoadError, ScenarioLoadOptions, ScenarioLoader};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn fixture_path(name: &str) -> PathBuf {
    fixtures_dir().join(name)
}

#[test]
fn valid_minimal_scenario_loads() {
    let context = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture_path("minimal.toml")))
        .expect("minimal fixture");

    assert_eq!(context.scenario_id().as_str(), "VP-CS-0001");
    assert_eq!(
        context.specification_binding().edition_id(),
        Some("edition-2026")
    );
    assert_eq!(context.claim().id.as_str(), "claim-001");
    assert_eq!(context.claim().subject, "payroll-run-42");
    assert_eq!(context.claim().assertion.assertion_type, "minimal");
    assert_eq!(context.claim().assertion.body, "expected-body");
    assert_eq!(context.evidence().id.as_str(), "evidence-001");
    assert_eq!(context.evidence().claim_id.as_str(), "claim-001");
    assert_eq!(context.evidence().content.content_type, "document");
    assert_eq!(context.evidence().content.body, "expected-body");
    assert!(!std::any::type_name::<vp_conformance_core::ScenarioContext>().contains("PathBuf"));
}

#[test]
fn missing_required_field_fails_clearly() {
    let error = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture_path(
            "missing_scenario_id.toml",
        )))
        .expect_err("missing scenario_id");

    assert!(matches!(
        error,
        ScenarioLoadError::Parse { .. } | ScenarioLoadError::MissingField { .. }
    ));
}

#[test]
fn invalid_binding_fails_clearly() {
    let error = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture_path(
            "invalid_binding.toml",
        )))
        .expect_err("invalid binding");

    assert_eq!(error, ScenarioLoadError::InvalidBinding);
}

#[test]
fn evidence_linked_to_claim_loads_correctly() {
    let context = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture_path("minimal.toml")))
        .expect("minimal fixture");

    assert_eq!(
        context.evidence().claim_id.as_str(),
        context.claim().id.as_str()
    );
}

#[test]
fn metadata_loads_when_present() {
    let context = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture_path("minimal.toml")))
        .expect("minimal fixture");

    assert_eq!(context.metadata().get("source"), Some("local-fixture"));
    assert_eq!(context.metadata().get("harness"), Some("milestone-b2"));
}

#[test]
fn load_str_reports_parse_errors_with_source_label() {
    let error = ScenarioLoader::new()
        .load_str("not valid toml", "inline-fixture.toml")
        .expect_err("parse error");

    assert!(matches!(error, ScenarioLoadError::Parse { .. }));
    assert!(error.to_string().contains("inline-fixture.toml"));
}

#[test]
fn claim_evidence_mismatch_fails_clearly() {
    let error = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(fixture_path(
            "claim_evidence_mismatch.toml",
        )))
        .expect_err("claim mismatch");

    assert_eq!(
        error,
        ScenarioLoadError::ClaimEvidenceMismatch {
            claim_id: "claim-001".to_owned(),
            evidence_claim_id: "claim-999".to_owned(),
        }
    );
}
