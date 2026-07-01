//! CLI library for the VerityPay conformance suite.

pub mod exit_code;
pub mod run;

pub use exit_code::{
    exit_code_from_report, EXIT_CONFORMANCE_FAILURE, EXIT_HARNESS_ERROR, EXIT_SUCCESS,
    EXIT_USER_ERROR,
};
pub use run::{parse_adapter_outcome, run_scenario, OutputFormat, RunError, RunOptions, RunOutput};
