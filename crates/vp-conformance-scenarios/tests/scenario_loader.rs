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

#[test]
fn spec_published_fixture_format_loads_from_toml() {
    let contents = r#"
scenario_id = "VP-CS-0001"
name = "Minimal claim is satisfied by matching evidence"
rule_id = "VP-RULE-0001"
rfc = "VP-RFC-0001"
protocol_version = "vp-protocol-draft"

[claim]
claim_id = "claim-001"
claim_type = "minimal"
subject = "subject-alpha"
assertion_type = "minimal"
assertion_body = "alpha"
specification_version = "vp-protocol-draft"

[evidence]
evidence_id = "evidence-001"
claim_id = "claim-001"
evidence_type = "document"
content_type = "document"
content_body = "alpha"

[expected]
outcome = "satisfied"

[metadata]
status = "draft"
source_rfc = "VP-RFC-0001"
description = "Minimal claim is satisfied by matching evidence."
"#;

    let context = ScenarioLoader::new()
        .load_str(contents, "VP-CS-0001.toml")
        .expect("spec fixture format");

    assert_eq!(context.scenario_id().as_str(), "VP-CS-0001");
    assert_eq!(
        context.specification_binding().protocol_version(),
        Some("vp-protocol-draft")
    );
    assert_eq!(context.claim().assertion.body, "alpha");
    assert_eq!(context.evidence().content.body, "alpha");
}
