//! Errors returned when required builder fields are missing or invalid.

/// A builder failed before producing an immutable domain object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildError {
    pub field: &'static str,
    pub reason: &'static str,
}

impl BuildError {
    #[must_use]
    pub const fn missing(field: &'static str) -> Self {
        Self {
            field,
            reason: "missing required field",
        }
    }

    #[must_use]
    pub const fn invalid(field: &'static str, reason: &'static str) -> Self {
        Self { field, reason }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.reason)
    }
}

impl std::error::Error for BuildError {}
