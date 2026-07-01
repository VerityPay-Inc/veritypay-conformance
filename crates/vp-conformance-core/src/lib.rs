//! Shared harness contracts for the VerityPay conformance suite.

pub mod adapter;
pub mod comparable_result;
pub mod conformance_result;
pub mod error;
pub mod scenario_context;

pub use adapter::ImplementationAdapter;
pub use comparable_result::ComparableResult;
pub use conformance_result::ConformanceResult;
pub use error::ConformanceError;
pub use scenario_context::ScenarioContext;
