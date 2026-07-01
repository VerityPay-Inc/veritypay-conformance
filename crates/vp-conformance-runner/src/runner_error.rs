//! Errors surfaced while orchestrating oracle and adapter execution.

use vp_conformance_core::AdapterError;

use crate::oracle_error::OracleError;

/// Failure while running oracle and adapter execution paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunnerError {
    /// Reference oracle path failed.
    OracleFailed(OracleError),
    /// Implementation adapter path failed.
    AdapterFailed(AdapterError),
}

impl RunnerError {
    #[must_use]
    pub const fn oracle_failed(error: OracleError) -> Self {
        Self::OracleFailed(error)
    }

    #[must_use]
    pub const fn adapter_failed(error: AdapterError) -> Self {
        Self::AdapterFailed(error)
    }
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OracleFailed(error) => write!(f, "oracle path failed: {error}"),
            Self::AdapterFailed(error) => write!(f, "adapter path failed: {error}"),
        }
    }
}

impl std::error::Error for RunnerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OracleFailed(error) => Some(error),
            Self::AdapterFailed(error) => Some(error),
        }
    }
}
