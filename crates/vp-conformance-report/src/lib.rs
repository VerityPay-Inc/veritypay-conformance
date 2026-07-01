//! Conformance report aggregation and formatting.

pub mod conformance_report;
pub mod human_report_renderer;
pub mod json_report_renderer;
pub mod report;
pub mod report_render_error;

pub use conformance_report::{ConformanceReport, ConformanceReportBuilder};
pub use human_report_renderer::HumanReportRenderer;
pub use json_report_renderer::JsonReportRenderer;
pub use report::Report;
pub use report_render_error::ReportRenderError;
