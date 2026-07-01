//! Compares adapter and oracle results into a conformance verdict.

use vp_conformance_core::{
    BuildError, ConformanceResult, ConformanceVerdict, ScenarioBinding, ScenarioId,
};

use crate::runner_result::RunnerResult;

/// Determines whether implementation and oracle execution paths agree.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComparisonEngine;

impl ComparisonEngine {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Placeholder constructor retained for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    /// Comparison logic is available (Milestone E.1).
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        true
    }

    /// Compares oracle and implementation results from a successful runner invocation.
    ///
    /// `scenario_id` is required because [`RunnerResult`] does not carry scenario identity.
    /// Comparison does not mutate either [`ComparableResult`].
    pub fn compare(
        &self,
        scenario_id: &ScenarioId,
        runner_result: &RunnerResult,
    ) -> Result<ConformanceResult, BuildError> {
        let oracle = runner_result.oracle_result();
        let implementation = runner_result.implementation_result();

        let mut builder = ConformanceResult::builder()
            .scenario_id(scenario_id.clone())
            .specification_binding(oracle.specification_binding().clone())
            .oracle_result(oracle.clone())
            .implementation_result(implementation.clone())
            .comparison_note(
                "trace.comparison",
                "deferred; trace diff not compared in this milestone",
            );

        let mut mismatch_count = 0usize;

        if oracle.outcome() != implementation.outcome() {
            builder = builder.comparison_note(
                "outcome.mismatch",
                format!(
                    "oracle={} implementation={}",
                    oracle.outcome().as_str(),
                    implementation.outcome().as_str(),
                ),
            );
            mismatch_count += 1;
        }

        if oracle.evaluated_claim_id() != implementation.evaluated_claim_id() {
            builder = builder.comparison_note(
                "evaluated_claim_id.mismatch",
                format!(
                    "oracle={} implementation={}",
                    oracle.evaluated_claim_id().as_str(),
                    implementation.evaluated_claim_id().as_str(),
                ),
            );
            mismatch_count += 1;
        }

        if oracle.specification_binding() != implementation.specification_binding() {
            builder = builder.comparison_note(
                "specification_binding.mismatch",
                format!(
                    "oracle={} implementation={}",
                    binding_summary(oracle.specification_binding()),
                    binding_summary(implementation.specification_binding()),
                ),
            );
            mismatch_count += 1;
        }

        let verdict = if mismatch_count == 0 {
            builder = builder.comparison_note("comparison", "pass: all compared fields match");
            ConformanceVerdict::Pass
        } else {
            builder = builder.comparison_note(
                "comparison",
                format!("fail: {mismatch_count} compared field(s) differ"),
            );
            ConformanceVerdict::Fail
        };

        builder.verdict(verdict).build()
    }
}

fn binding_summary(binding: &ScenarioBinding) -> String {
    match (binding.edition_id(), binding.protocol_version()) {
        (Some(edition), Some(protocol)) => {
            format!("edition={edition} protocol={protocol}")
        }
        (Some(edition), None) => format!("edition={edition}"),
        (None, Some(protocol)) => format!("protocol={protocol}"),
        (None, None) => "unpinned".to_owned(),
    }
}
