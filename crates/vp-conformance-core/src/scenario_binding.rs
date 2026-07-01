//! Resolved specification binding for a conformance scenario.

use crate::build_error::BuildError;

/// Edition or protocol version pin governing a scenario run.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScenarioBinding {
    edition_id: Option<String>,
    protocol_version: Option<String>,
}

impl ScenarioBinding {
    #[must_use]
    pub fn edition_id(&self) -> Option<&str> {
        self.edition_id.as_deref()
    }

    #[must_use]
    pub fn protocol_version(&self) -> Option<&str> {
        self.protocol_version.as_deref()
    }

    #[must_use]
    pub fn builder() -> ScenarioBindingBuilder {
        ScenarioBindingBuilder::new()
    }
}

/// Constructs a [`ScenarioBinding`] with at least one pin set.
#[derive(Debug, Default)]
pub struct ScenarioBindingBuilder {
    edition_id: Option<String>,
    protocol_version: Option<String>,
}

impl ScenarioBindingBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn edition_id(mut self, edition_id: impl Into<String>) -> Self {
        self.edition_id = Some(edition_id.into());
        self
    }

    #[must_use]
    pub fn protocol_version(mut self, protocol_version: impl Into<String>) -> Self {
        self.protocol_version = Some(protocol_version.into());
        self
    }

    pub fn build(self) -> Result<ScenarioBinding, BuildError> {
        if self.edition_id.is_none() && self.protocol_version.is_none() {
            return Err(BuildError::invalid(
                "specification_binding",
                "at least one of edition_id or protocol_version is required",
            ));
        }

        Ok(ScenarioBinding {
            edition_id: self.edition_id,
            protocol_version: self.protocol_version,
        })
    }
}
