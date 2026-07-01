//! Conformance report aggregation and formatting.

pub mod conformance_report;
pub mod human_report_renderer;
pub mod report;

pub use conformance_report::{ConformanceReport, ConformanceReportBuilder};
pub use human_report_renderer::HumanReportRenderer;
pub use report::Report;
