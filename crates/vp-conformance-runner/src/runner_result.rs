//! Output of runner orchestration before comparison.

use vp_conformance_core::ComparableResult;

/// Pair of execution-path results produced by [`ConformanceRunner`](crate::runner::ConformanceRunner).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerResult {
    oracle_result: ComparableResult,
    implementation_result: ComparableResult,
}

impl RunnerResult {
    #[must_use]
    pub fn new(oracle_result: ComparableResult, implementation_result: ComparableResult) -> Self {
        Self {
            oracle_result,
            implementation_result,
        }
    }

    #[must_use]
    pub fn oracle_result(&self) -> &ComparableResult {
        &self.oracle_result
    }

    #[must_use]
    pub fn implementation_result(&self) -> &ComparableResult {
        &self.implementation_result
    }
}
