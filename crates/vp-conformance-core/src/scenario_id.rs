//! Stable VP-CS scenario identifiers.

use std::fmt;

/// Stable identifier for a conformance scenario.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScenarioId(String);

impl ScenarioId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ScenarioId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ScenarioId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for ScenarioId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
