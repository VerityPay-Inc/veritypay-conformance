//! Workspace integration tests.

#[test]
fn workspace_crates_are_linkable() {
    use vp_conformance_adapter::StubAdapter;
    use vp_conformance_core::{
        ComparableResult, ConformanceError, ConformanceResult, ImplementationAdapter,
        ScenarioContext,
    };
    use vp_conformance_report::Report;
    use vp_conformance_runner::{ComparisonEngine, ConformanceRunner, ReferenceOracle};
    use vp_conformance_scenarios::ScenarioLoader;

    let context = ScenarioContext::placeholder();
    let adapter = StubAdapter::placeholder();

    let _ = (
        context,
        ComparableResult::placeholder(),
        ConformanceResult::placeholder(),
        ConformanceError::placeholder(),
        adapter.adapter_id(),
        adapter.accepts(&ScenarioContext::placeholder()),
        ScenarioLoader::placeholder(),
        ReferenceOracle::placeholder(),
        ComparisonEngine::placeholder(),
        ConformanceRunner::placeholder(),
        Report::placeholder(),
    );
}

#[test]
fn workspace_runner_reports_bootstrapped() {
    let runner = vp_conformance_runner::ConformanceRunner::placeholder();
    assert!(runner.is_bootstrapped());
}
