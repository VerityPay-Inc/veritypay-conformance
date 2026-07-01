//! Loads local VP-CS scenario fixtures into immutable [`ScenarioContext`] records.

use std::fs;
use std::path::Path;

use vp_conformance_core::ScenarioContext;

use crate::error::ScenarioLoadError;
use crate::fixture::parse_fixture;
use crate::options::ScenarioLoadOptions;

/// Ingests a minimal local scenario fixture file into a path-free [`ScenarioContext`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScenarioLoader;

impl ScenarioLoader {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Placeholder constructor retained for workspace bootstrap wiring.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    /// Loads a scenario fixture from the path declared in `options`.
    pub fn load(
        &self,
        options: &ScenarioLoadOptions,
    ) -> Result<ScenarioContext, ScenarioLoadError> {
        let path = options.fixture_path();
        let contents = fs::read_to_string(path)
            .map_err(|source| ScenarioLoadError::io(path.to_path_buf(), source))?;
        self.load_str(&contents, path)
    }

    /// Loads a scenario fixture from TOML contents (used by tests and callers with in-memory fixtures).
    pub fn load_str(
        &self,
        contents: &str,
        source_label: impl AsRef<Path>,
    ) -> Result<ScenarioContext, ScenarioLoadError> {
        let fixture = parse_fixture(contents).map_err(|source| ScenarioLoadError::Parse {
            path: source_label.as_ref().to_path_buf(),
            message: source.to_string(),
        })?;
        fixture.into_context()
    }

    /// Returns whether the loader crate is wired for workspace bootstrap checks.
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        true
    }
}
