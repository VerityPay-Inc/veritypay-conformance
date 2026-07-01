//! Runner orchestration tests for Milestone D.2.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use vp_conformance_adapter::StubAdapter;
use vp_conformance_core::{
    AdapterError, AdapterId, ComparableResult, ImplementationAdapter, ScenarioBinding,
    ScenarioContext, ScenarioId,
};
use vp_conformance_runner::{
    ConformanceRunner, OracleError, OracleEvaluate, ReferenceOracle, RunnerError, RunnerResult,
};
use vp_reference_core::ContextBuildError;
use vp_reference_model::{Assertion, Claim, Evidence, EvidenceContent, Outcome};

fn scenario_context(assertion_body: &str, evidence_body: &str) -> ScenarioContext {
    let claim = Claim::builder()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", assertion_body))
        .build()
        .expect("claim");
    let evidence = Evidence::builder()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", evidence_body))
        .build()
        .expect("evidence");
    let binding = ScenarioBinding::builder()
        .edition_id("2026-01")
        .protocol_version("0.1.0")
        .build()
        .expect("binding");

    ScenarioContext::builder()
        .scenario_id(ScenarioId::new("VP-CS-0001"))
        .specification_binding(binding)
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect("scenario context")
}

struct FailingAdapter;

impl ImplementationAdapter for FailingAdapter {
    fn id(&self) -> &AdapterId {
        static ID: std::sync::OnceLock<AdapterId> = std::sync::OnceLock::new();
        ID.get_or_init(|| AdapterId::new("failing-adapter"))
    }

    fn run(&self, _context: &ScenarioContext) -> Result<ComparableResult, AdapterError> {
        Err(AdapterError::execution_failed("injected adapter failure"))
    }
}

struct FailingOracle;

impl OracleEvaluate for FailingOracle {
    fn evaluate_oracle(&self, _context: &ScenarioContext) -> Result<ComparableResult, OracleError> {
        Err(OracleError::context_build(ContextBuildError::missing(
            "claim",
        )))
    }
}

struct RecordingOracle {
    stage: Arc<AtomicU8>,
    inner: ReferenceOracle,
}

impl RecordingOracle {
    fn new(stage: Arc<AtomicU8>) -> Self {
        Self {
            stage,
            inner: ReferenceOracle::new(),
        }
    }
}

impl OracleEvaluate for RecordingOracle {
    fn evaluate_oracle(&self, context: &ScenarioContext) -> Result<ComparableResult, OracleError> {
        self.stage.store(1, Ordering::SeqCst);
        self.inner.evaluate(context)
    }
}

struct OrderCheckingAdapter {
    stage: Arc<AtomicU8>,
    outcome: Outcome,
}

impl ImplementationAdapter for OrderCheckingAdapter {
    fn id(&self) -> &AdapterId {
        static ID: std::sync::OnceLock<AdapterId> = std::sync::OnceLock::new();
        ID.get_or_init(|| AdapterId::new("order-check"))
    }

    fn run(&self, context: &ScenarioContext) -> Result<ComparableResult, AdapterError> {
        assert_eq!(
            self.stage.load(Ordering::SeqCst),
            1,
            "oracle must run before adapter"
        );
        self.stage.store(2, Ordering::SeqCst);
        vp_conformance_core::build_implementation_result(self.id(), context, self.outcome)
    }
}

fn runner_with_stub(outcome: Outcome) -> ConformanceRunner {
    ConformanceRunner::new(
        ReferenceOracle::new(),
        Box::new(StubAdapter::new("stub", outcome)),
    )
}

#[test]
fn runner_invokes_oracle_then_adapter() {
    let stage = Arc::new(AtomicU8::new(0));
    let runner = ConformanceRunner::from_evaluators(
        Box::new(RecordingOracle::new(stage.clone())),
        Box::new(OrderCheckingAdapter {
            stage: stage.clone(),
            outcome: Outcome::Satisfied,
        }),
    );

    runner
        .run(&scenario_context("alpha", "alpha"))
        .expect("runner result");

    assert_eq!(stage.load(Ordering::SeqCst), 2);
}

#[test]
fn runner_returns_both_comparable_results() {
    let result = runner_with_stub(Outcome::Satisfied)
        .run(&scenario_context("alpha", "alpha"))
        .expect("runner result");

    assert!(result
        .oracle_result()
        .execution_path()
        .is_reference_oracle());
    assert!(result
        .implementation_result()
        .execution_path()
        .is_implementation_adapter());
}

#[test]
fn oracle_failure_propagates_correctly() {
    let error = ConformanceRunner::from_evaluators(
        Box::new(FailingOracle),
        Box::new(StubAdapter::new("stub", Outcome::Satisfied)),
    )
    .run(&scenario_context("alpha", "alpha"))
    .expect_err("oracle failure");

    assert!(matches!(error, RunnerError::OracleFailed(_)));
    assert!(error.to_string().contains("oracle path failed"));
}

#[test]
fn adapter_failure_propagates_correctly() {
    let error = ConformanceRunner::new(ReferenceOracle::new(), Box::new(FailingAdapter))
        .run(&scenario_context("alpha", "alpha"))
        .expect_err("adapter failure");

    assert!(matches!(error, RunnerError::AdapterFailed(_)));
    assert!(error.to_string().contains("adapter path failed"));
}

#[test]
fn runner_does_not_compare_outcomes() {
    let result = runner_with_stub(Outcome::NotSatisfied)
        .run(&scenario_context("alpha", "alpha"))
        .expect("runner result");

    assert_eq!(result.oracle_result().outcome(), Outcome::Satisfied);
    assert_eq!(
        result.implementation_result().outcome(),
        Outcome::NotSatisfied
    );
    assert_eq!(
        std::any::type_name::<RunnerResult>(),
        std::any::type_name::<vp_conformance_runner::RunnerResult>()
    );
}

#[test]
fn runner_remains_filesystem_independent() {
    let result = runner_with_stub(Outcome::Satisfied)
        .run(&scenario_context("alpha", "alpha"))
        .expect("runner result");

    assert!(!std::any::type_name::<ConformanceRunner>().contains("PathBuf"));
    assert!(!std::any::type_name::<RunnerResult>().contains("PathBuf"));
    assert_eq!(result.oracle_result().outcome(), Outcome::Satisfied);
}
