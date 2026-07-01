//! Errors while rendering a conformance report.

use core::fmt;

/// Failure while serializing a report for export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportRenderError {
    message: String,
}

impl ReportRenderError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ReportRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ReportRenderError {}

impl From<serde_json::Error> for ReportRenderError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(error.to_string())
    }
}
