//! Harness verdict vocabulary for a conformance check.

/// Classification of a conformance comparison — distinct from normative [`Outcome`](vp_reference_model::Outcome).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConformanceVerdict {
    Pass,
    Fail,
    Skip,
    Error,
}

impl ConformanceVerdict {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Skip => "skip",
            Self::Error => "error",
        }
    }
}
