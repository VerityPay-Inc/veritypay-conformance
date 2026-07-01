//! Filesystem inputs for scenario fixture loading.

use std::path::{Path, PathBuf};

/// Path to a local scenario fixture file on disk.
///
/// [`ScenarioContext`](vp_conformance_core::ScenarioContext) produced by the loader
/// remains path-free per [ADR-0004](../../../docs/adrs/0004-conformance-public-contract.md).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioLoadOptions {
    fixture_path: PathBuf,
}

impl ScenarioLoadOptions {
    #[must_use]
    pub fn new(fixture_path: impl Into<PathBuf>) -> Self {
        Self {
            fixture_path: fixture_path.into(),
        }
    }

    #[must_use]
    pub fn fixture_path(&self) -> &Path {
        &self.fixture_path
    }
}
