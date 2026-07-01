//! Aggregated conformance summary over one or more scenario results.

use vp_conformance_core::{ConformanceResult, ConformanceVerdict};

/// Immutable summary of one or more [`ConformanceResult`] records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceReport {
    results: Vec<ConformanceResult>,
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
    errors: usize,
}

impl ConformanceReport {
    #[must_use]
    pub fn builder() -> ConformanceReportBuilder {
        ConformanceReportBuilder::new()
    }

    /// Builds a report from cloned scenario results with derived counts.
    #[must_use]
    pub fn from_results(results: impl IntoIterator<Item = ConformanceResult>) -> Self {
        let results: Vec<ConformanceResult> = results.into_iter().collect();
        Self::from_owned_results(results)
    }

    #[must_use]
    pub fn total(&self) -> usize {
        self.total
    }

    #[must_use]
    pub fn passed(&self) -> usize {
        self.passed
    }

    #[must_use]
    pub fn failed(&self) -> usize {
        self.failed
    }

    #[must_use]
    pub fn skipped(&self) -> usize {
        self.skipped
    }

    #[must_use]
    pub fn errors(&self) -> usize {
        self.errors
    }

    #[must_use]
    pub fn results(&self) -> &[ConformanceResult] {
        &self.results
    }

    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.failed > 0
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.passed as f64 / self.total as f64
    }

    fn from_owned_results(results: Vec<ConformanceResult>) -> Self {
        let mut passed = 0usize;
        let mut failed = 0usize;
        let mut skipped = 0usize;
        let mut errors = 0usize;

        for result in &results {
            match result.verdict() {
                ConformanceVerdict::Pass => passed += 1,
                ConformanceVerdict::Fail => failed += 1,
                ConformanceVerdict::Skip => skipped += 1,
                ConformanceVerdict::Error => errors += 1,
            }
        }

        let total = results.len();

        Self {
            results,
            total,
            passed,
            failed,
            skipped,
            errors,
        }
    }
}

/// Constructs a [`ConformanceReport`] from scenario results.
#[derive(Debug, Default)]
pub struct ConformanceReportBuilder {
    results: Vec<ConformanceResult>,
}

impl ConformanceReportBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn result(mut self, result: ConformanceResult) -> Self {
        self.results.push(result);
        self
    }

    #[must_use]
    pub fn results(mut self, results: impl IntoIterator<Item = ConformanceResult>) -> Self {
        self.results.extend(results);
        self
    }

    #[must_use]
    pub fn build(self) -> ConformanceReport {
        ConformanceReport::from_owned_results(self.results)
    }
}
