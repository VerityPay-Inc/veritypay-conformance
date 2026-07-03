//! VP-CS scenario TOML fixture representation.
//!
//! Accepts the local harness scaffold format (Milestone B.2) and spec-published
//! fixtures from `veritypay-spec/spec/conformance/scenarios/`.

use serde::Deserialize;
use vp_conformance_core::{
    BuildError, ScenarioBinding, ScenarioContext, ScenarioId, ScenarioMetadata,
};
use vp_reference_model::{Assertion, Claim, Evidence, EvidenceContent};

use crate::error::ScenarioLoadError;

/// Scenario fixture schema for harness and spec-published VP-CS inputs.
///
/// Spec fixtures use RFC field names (`claim_id`, `evidence_id`, `protocol_version`);
/// local scaffold fixtures use harness field names (`id`, `edition_id`). Both map to
/// the same [`ScenarioContext`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct ScenarioFixture {
    pub scenario_id: String,
    pub edition_id: Option<String>,
    pub protocol_version: Option<String>,
    pub claim: ClaimFixture,
    pub evidence: EvidenceFixture,
    #[serde(default)]
    pub metadata: Option<MetadataFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct ClaimFixture {
    #[serde(alias = "claim_id")]
    pub id: String,
    pub subject: String,
    pub assertion_type: String,
    pub assertion_body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct EvidenceFixture {
    #[serde(alias = "evidence_id")]
    pub id: String,
    pub claim_id: String,
    pub content_type: String,
    pub content_body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct MetadataFixture {
    #[serde(flatten)]
    pub entries: std::collections::BTreeMap<String, String>,
}

impl ScenarioFixture {
    pub(crate) fn into_context(self) -> Result<ScenarioContext, ScenarioLoadError> {
        require_non_empty(&self.scenario_id, "scenario_id")?;
        let binding = build_binding(&self.edition_id, &self.protocol_version)?;

        let claim = build_claim(&self.claim)?;
        let evidence = build_evidence(&self.evidence)?;

        let mut builder = ScenarioContext::builder()
            .scenario_id(ScenarioId::new(self.scenario_id))
            .specification_binding(binding)
            .claim(claim)
            .evidence(evidence);

        if let Some(metadata) = self.metadata {
            builder = builder.metadata(ScenarioMetadata::from_pairs(metadata.entries));
        }

        builder.build().map_err(ScenarioLoadError::DomainBuild)
    }
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), ScenarioLoadError> {
    if value.trim().is_empty() {
        Err(ScenarioLoadError::missing(field))
    } else {
        Ok(())
    }
}

fn build_binding(
    edition_id: &Option<String>,
    protocol_version: &Option<String>,
) -> Result<ScenarioBinding, ScenarioLoadError> {
    let mut builder = ScenarioBinding::builder();

    if let Some(edition_id) = edition_id {
        if !edition_id.trim().is_empty() {
            builder = builder.edition_id(edition_id.clone());
        }
    }

    if let Some(protocol_version) = protocol_version {
        if !protocol_version.trim().is_empty() {
            builder = builder.protocol_version(protocol_version.clone());
        }
    }

    builder.build().map_err(|error| {
        if error.field == "specification_binding" {
            ScenarioLoadError::InvalidBinding
        } else {
            ScenarioLoadError::DomainBuild(error)
        }
    })
}

fn build_claim(fixture: &ClaimFixture) -> Result<Claim, ScenarioLoadError> {
    require_non_empty(&fixture.id, "claim.id")?;
    require_non_empty(&fixture.subject, "claim.subject")?;
    require_non_empty(&fixture.assertion_type, "claim.assertion_type")?;
    require_non_empty(&fixture.assertion_body, "claim.assertion_body")?;

    Claim::builder()
        .id(fixture.id.clone())
        .subject(fixture.subject.clone())
        .assertion(Assertion::new(
            fixture.assertion_type.clone(),
            fixture.assertion_body.clone(),
        ))
        .build()
        .map_err(|error| ScenarioLoadError::DomainBuild(BuildError::missing(error.field)))
}

fn build_evidence(fixture: &EvidenceFixture) -> Result<Evidence, ScenarioLoadError> {
    require_non_empty(&fixture.id, "evidence.id")?;
    require_non_empty(&fixture.claim_id, "evidence.claim_id")?;
    require_non_empty(&fixture.content_type, "evidence.content_type")?;
    // content_body may be empty or whitespace-only — protocol rules determine outcome.

    Evidence::builder()
        .id(fixture.id.clone())
        .claim_id(fixture.claim_id.clone())
        .content(EvidenceContent::new(
            fixture.content_type.clone(),
            fixture.content_body.clone(),
        ))
        .build()
        .map_err(|error| ScenarioLoadError::DomainBuild(BuildError::missing(error.field)))
}

pub(crate) fn parse_fixture(contents: &str) -> Result<ScenarioFixture, toml::de::Error> {
    toml::from_str(contents)
}
