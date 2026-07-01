//! Conformance run orchestration.

pub mod comparison_engine;
pub mod oracle;
pub mod oracle_error;
pub mod runner;
pub mod runner_error;
pub mod runner_result;

pub use comparison_engine::ComparisonEngine;
pub use oracle::ReferenceOracle;
pub use oracle_error::OracleError;
pub use runner::ConformanceRunner;
#[doc(hidden)]
pub use runner::OracleEvaluate;
pub use runner_error::RunnerError;
pub use runner_result::RunnerResult;
