//! Coordinates reference oracle and implementation adapter execution.

use vp_conformance_core::{ImplementationAdapter, ScenarioContext};

use crate::oracle::ReferenceOracle;
use crate::runner_error::RunnerError;
use crate::runner_result::RunnerResult;

pub trait OracleEvaluate {
    fn evaluate_oracle(
        &self,
        context: &ScenarioContext,
    ) -> Result<vp_conformance_core::ComparableResult, crate::oracle_error::OracleError>;
}

enum OracleSlot {
    Reference(ReferenceOracle),
    #[doc(hidden)]
    Custom(Box<dyn OracleEvaluate>),
}

impl OracleSlot {
    fn evaluate_oracle(
        &self,
        context: &ScenarioContext,
    ) -> Result<vp_conformance_core::ComparableResult, crate::oracle_error::OracleError> {
        match self {
            Self::Reference(oracle) => oracle.evaluate(context),
            Self::Custom(custom) => custom.evaluate_oracle(context),
        }
    }
}

/// Coordinates oracle and adapter execution paths for one scenario.
pub struct ConformanceRunner {
    oracle: OracleSlot,
    adapter: Box<dyn ImplementationAdapter>,
}

impl ConformanceRunner {
    #[must_use]
    pub fn new(oracle: ReferenceOracle, adapter: Box<dyn ImplementationAdapter>) -> Self {
        Self {
            oracle: OracleSlot::Reference(oracle),
            adapter,
        }
    }

    /// Bootstrap-compatible constructor with default oracle and stub adapter wiring.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new(
            ReferenceOracle::new(),
            Box::new(vp_conformance_adapter::StubAdapter::placeholder()),
        )
    }

    /// Hidden constructor for orchestration tests with custom oracle doubles.
    #[doc(hidden)]
    #[must_use]
    pub fn from_evaluators(
        oracle: Box<dyn OracleEvaluate>,
        adapter: Box<dyn ImplementationAdapter>,
    ) -> Self {
        Self {
            oracle: OracleSlot::Custom(oracle),
            adapter,
        }
    }

    /// Returns whether orchestration wiring is available.
    #[must_use]
    pub fn is_bootstrapped(&self) -> bool {
        true
    }

    /// Runs the reference oracle path, then the implementation adapter path.
    ///
    /// Does not compare results or assign pass/fail verdicts.
    pub fn run(&self, context: &ScenarioContext) -> Result<RunnerResult, RunnerError> {
        let oracle_result = self
            .oracle
            .evaluate_oracle(context)
            .map_err(RunnerError::oracle_failed)?;
        let implementation_result = self
            .adapter
            .run(context)
            .map_err(RunnerError::adapter_failed)?;

        Ok(RunnerResult::new(oracle_result, implementation_result))
    }
}
