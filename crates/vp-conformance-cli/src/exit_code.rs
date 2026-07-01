//! Process exit codes for the conformance CLI.

use vp_conformance_report::ConformanceReport;

/// Exit code `0` when the report has no failures or errors.
pub const EXIT_SUCCESS: i32 = 0;

/// Exit code `1` when the conformance report records failures or errors.
pub const EXIT_CONFORMANCE_FAILURE: i32 = 1;

/// Exit code `2` for CLI or user input errors.
pub const EXIT_USER_ERROR: i32 = 2;

/// Exit code `3` for harness or internal execution errors.
pub const EXIT_HARNESS_ERROR: i32 = 3;

#[must_use]
pub fn exit_code_from_report(report: &ConformanceReport) -> i32 {
    if report.has_failures() || report.has_errors() {
        EXIT_CONFORMANCE_FAILURE
    } else {
        EXIT_SUCCESS
    }
}
