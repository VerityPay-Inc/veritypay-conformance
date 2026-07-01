//! Conformance run orchestration (placeholder until Milestones D–E).

pub mod comparison_engine;
pub mod oracle;
pub mod oracle_error;
pub mod runner;

pub use comparison_engine::ComparisonEngine;
pub use oracle::ReferenceOracle;
pub use oracle_error::OracleError;
pub use runner::ConformanceRunner;
