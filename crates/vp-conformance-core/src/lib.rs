//! Shared harness contracts for the VerityPay conformance suite.

pub mod adapter;
pub mod build_error;
pub mod comparable_result;
pub mod conformance_result;
pub mod conformance_verdict;
pub mod error;
pub mod scenario_binding;
pub mod scenario_context;
pub mod scenario_id;
pub mod scenario_metadata;

pub use adapter::ImplementationAdapter;
pub use build_error::BuildError;
pub use comparable_result::{ComparableResult, ComparableResultBuilder, ExecutionPath};
pub use conformance_result::{ConformanceResult, ConformanceResultBuilder};
pub use conformance_verdict::ConformanceVerdict;
pub use error::ConformanceError;
pub use scenario_binding::{ScenarioBinding, ScenarioBindingBuilder};
pub use scenario_context::{ScenarioContext, ScenarioContextBuilder};
pub use scenario_id::ScenarioId;
pub use scenario_metadata::ScenarioMetadata;
