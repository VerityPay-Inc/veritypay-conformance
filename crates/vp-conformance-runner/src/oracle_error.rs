//! Errors surfaced while invoking the reference oracle.

use vp_conformance_core::BuildError;
use vp_reference_core::ContextBuildError;

/// Failure while producing an oracle-path [`ComparableResult`](vp_conformance_core::ComparableResult).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OracleError {
    /// [`EvaluationContext`](vp_reference_core::EvaluationContext) construction failed.
    ContextBuild(ContextBuildError),
    /// Oracle comparable result construction failed.
    InvalidResult(BuildError),
}

impl OracleError {
    #[must_use]
    pub const fn context_build(error: ContextBuildError) -> Self {
        Self::ContextBuild(error)
    }
}

impl std::fmt::Display for OracleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContextBuild(error) => {
                write!(f, "oracle evaluation context build failed: {error}")
            }
            Self::InvalidResult(error) => {
                write!(f, "oracle comparable result build failed: {error}")
            }
        }
    }
}

impl std::error::Error for OracleError {}
