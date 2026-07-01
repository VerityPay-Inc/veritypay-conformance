//! Actionable errors surfaced while loading scenario fixtures.

use std::path::PathBuf;

use vp_conformance_core::BuildError;

/// Failure while loading a local scenario fixture into [`ScenarioContext`](vp_conformance_core::ScenarioContext).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioLoadError {
    /// Fixture file could not be read.
    Io { path: PathBuf, message: String },
    /// Fixture file could not be parsed as TOML.
    Parse { path: PathBuf, message: String },
    /// A required fixture field was absent or empty.
    MissingField { field: &'static str },
    /// Neither `edition_id` nor `protocol_version` was provided.
    InvalidBinding,
    /// Evidence `claim_id` did not match the loaded claim.
    ClaimEvidenceMismatch {
        claim_id: String,
        evidence_claim_id: String,
    },
    /// Domain object construction failed after parsing.
    DomainBuild(BuildError),
}

impl ScenarioLoadError {
    #[must_use]
    pub fn io(path: PathBuf, source: std::io::Error) -> Self {
        Self::Io {
            path,
            message: source.to_string(),
        }
    }

    #[must_use]
    pub const fn missing(field: &'static str) -> Self {
        Self::MissingField { field }
    }
}

impl std::fmt::Display for ScenarioLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, message } => {
                write!(f, "failed to read fixture {}: {message}", path.display())
            }
            Self::Parse { path, message } => {
                write!(f, "failed to parse fixture {}: {message}", path.display())
            }
            Self::MissingField { field } => {
                write!(f, "missing required fixture field: {field}")
            }
            Self::InvalidBinding => {
                write!(
                    f,
                    "invalid specification binding: at least one of edition_id or protocol_version is required"
                )
            }
            Self::ClaimEvidenceMismatch {
                claim_id,
                evidence_claim_id,
            } => {
                write!(
                    f,
                    "evidence claim_id `{evidence_claim_id}` does not match claim id `{claim_id}`"
                )
            }
            Self::DomainBuild(error) => write!(f, "fixture domain build failed: {error}"),
        }
    }
}

impl std::error::Error for ScenarioLoadError {}
