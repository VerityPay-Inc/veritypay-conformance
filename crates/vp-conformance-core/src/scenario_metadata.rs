//! Non-normative metadata attached to conformance domain objects.

use std::collections::BTreeMap;

/// Non-normative harness metadata that must not decide protocol truth.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScenarioMetadata {
    entries: BTreeMap<String, String>,
}

impl ScenarioMetadata {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_pairs(
        pairs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let mut metadata = Self::new();
        for (key, value) in pairs {
            metadata.entries.insert(key.into(), value.into());
        }
        metadata
    }

    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
