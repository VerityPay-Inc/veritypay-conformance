//! Integration tests for spec-published VP-CS fixtures (Milestone G.2).

use std::path::PathBuf;

use vp_conformance_scenarios::{ScenarioLoadOptions, ScenarioLoader};

fn sibling_spec_vp_cs_0001_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml")
}

#[test]
fn optional_sibling_spec_vp_cs_0001_loads_when_present() {
    let fixture = sibling_spec_vp_cs_0001_path();
    if !fixture.is_file() {
        eprintln!("skipping: sibling {} not found", fixture.display());
        return;
    }

    let context = ScenarioLoader::new()
        .load(&ScenarioLoadOptions::new(&fixture))
        .expect("load spec VP-CS-0001");

    assert_eq!(context.scenario_id().as_str(), "VP-CS-0001");
    assert_eq!(
        context.specification_binding().protocol_version(),
        Some("vp-protocol-draft")
    );
    assert_eq!(context.claim().id.as_str(), "claim-001");
    assert_eq!(context.claim().subject, "subject-alpha");
    assert_eq!(context.claim().assertion.assertion_type, "minimal");
    assert_eq!(context.claim().assertion.body, "alpha");
    assert_eq!(context.evidence().id.as_str(), "evidence-001");
    assert_eq!(context.evidence().claim_id.as_str(), "claim-001");
    assert_eq!(context.evidence().content.content_type, "document");
    assert_eq!(context.evidence().content.body, "alpha");
    assert_eq!(context.metadata().get("source_rfc"), Some("VP-RFC-0001"));
    assert_eq!(context.metadata().get("status"), Some("draft"));
}
