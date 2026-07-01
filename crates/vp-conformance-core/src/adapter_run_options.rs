//! Optional execution knobs for adapter runs.

/// Per-run options for [`ImplementationAdapter::run_with_options`](crate::adapter::ImplementationAdapter::run_with_options).
///
/// Intentionally minimal until later milestones add trace or timeout controls.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdapterRunOptions {}

impl AdapterRunOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
