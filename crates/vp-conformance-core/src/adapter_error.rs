//! Errors returned by implementation adapters.

use crate::build_error::BuildError;

/// Failure while executing an implementation adapter path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    /// Adapter execution failed before a comparable result could be produced.
    ExecutionFailed { message: String },
    /// Comparable result construction failed after execution.
    InvalidResult(BuildError),
}

impl AdapterError {
    #[must_use]
    pub fn execution_failed(message: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExecutionFailed { message } => {
                write!(f, "adapter execution failed: {message}")
            }
            Self::InvalidResult(error) => write!(f, "invalid adapter result: {error}"),
        }
    }
}

impl std::error::Error for AdapterError {}
