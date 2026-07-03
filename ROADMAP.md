# Roadmap

**Capability-based roadmap for `veritypay-conformance`.**

This roadmap is **not date-driven**. Milestones complete when their success criteria are met—not when a quarter ends. Progress aligns with [Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) and the conformance role defined in [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md).

**Current status:** **Repository Ready** — Milestones A–G complete; **G.6** adds Platform 1.3 **`normalized_text`** VP-CS coverage; maintenance and extension mode for broader scenario sets.

---

## Overview

| Milestone | Name | Status |
|-----------|------|--------|
| **A** | Repository scaffold | **Complete** |
| **B** | Load scenario fixtures | **Complete** |
| **C** | Adapter contract | **Complete** |
| **D** | Run reference oracle | **Complete** |
| **E** | Compare implementation output | **Complete** |
| **F** | Produce conformance report | **Complete** |
| **G** | CI integration | **Complete** |

Each milestone below includes **Goal**, **Outputs**, **Success criteria**, and **Not included** so scope stays explicit.

**Canonical milestone order** ([ADR-0003](docs/adrs/0003-conformance-architecture.md)):

| Milestone | Name | Pipeline stage |
|-----------|------|----------------|
| **B** | Load scenario fixtures | ScenarioLoader, ScenarioContext |
| **C** | Adapter contract | ImplementationAdapter, comparable result shape |
| **D** | Run reference oracle | ReferenceOracle |
| **E** | Compare implementation output | ComparisonEngine, ConformanceResult |
| **F** | Produce conformance report | Report |
| **G** | CI integration | CI invocation |

**B → C → D → E → F → G** is fixed. Do not reorder milestones without a successor ADR.

**Ordering note:** Milestone **C** (adapter contract) precedes **D** (reference oracle) so both execution paths share a comparable result shape before **E** (comparison). Comparison receives **two sibling results**—implementation and oracle—not a chained pipeline where oracle follows adapter output.

---

## Milestone A — Repository scaffold

**Goal:** Establish `veritypay-conformance` as a mature **engineering project** before runner code exists—clear purpose, architecture, contribution rules, and boundaries with sibling repositories.

**Outputs:**

- [README.md](README.md) — purpose, boundaries, links to sibling repos
- [ARCHITECTURE.md](ARCHITECTURE.md) — conformance pipeline (conceptual; Rust per ADR-0001)
- [ROADMAP.md](ROADMAP.md) — this document with milestones A–G
- [CONTRIBUTING.md](CONTRIBUTING.md) — contributor expectations and specification boundary
- [docs/adrs/0001-implementation-language.md](docs/adrs/0001-implementation-language.md) — ADR-0001: Rust (Accepted)
- [docs/adrs/0002-cargo-workspace-architecture.md](docs/adrs/0002-cargo-workspace-architecture.md) — ADR-0002: Cargo workspace (Accepted)
- [docs/adrs/0003-conformance-architecture.md](docs/adrs/0003-conformance-architecture.md) — ADR-0003: Conformance pipeline (Accepted)
- [docs/adrs/0004-conformance-public-contract.md](docs/adrs/0004-conformance-public-contract.md) — ADR-0004: Public contract (Accepted)
- [LICENSE](LICENSE) — license terms
- Repository maturity declared: **Scaffold**

**Success criteria:**

- [x] A new contributor can explain what the conformance suite does and does not do in five minutes
- [x] Dependency on `veritypay-spec`, `veritypay-reference`, and `veritypay-tooling` is explicit and one-directional
- [x] Milestones B–G each define goal, outputs, success criteria, and not-included scope
- [x] No runner logic merged under the pretense of "early MVP"
- [x] Oracle dependency on `veritypay-reference` public contract is documented

**Not included:**

- Scenario loading or parsing
- Reference oracle invocation
- Implementation adapters
- Cargo workspace (deferred to Milestone B bootstrap)

---

## Milestone B — Load scenario fixtures

**Goal:** Load **VP-CS scenario fixtures** from a validated specification checkout into a normalized internal representation.

**Prerequisite:** [ADR-0001](docs/adrs/0001-implementation-language.md) — Rust (Accepted); [ADR-0002](docs/adrs/0002-cargo-workspace-architecture.md) — workspace layout (Accepted); [ADR-0003](docs/adrs/0003-conformance-architecture.md) — pipeline (Accepted); `veritypay-tooling` readiness; validated `veritypay-spec` sibling or pin.

### Workspace bootstrap (complete)

**Goal:** Scaffold the Cargo workspace per ADR-0002 before VP-CS loading begins.

**Outputs:**

- Cargo workspace with `vp-conformance-*` crates per [ADR-0002](docs/adrs/0002-cargo-workspace-architecture.md)
- `rust-toolchain.toml`, `rustfmt.toml`, `.gitignore`, `.editorconfig`
- [`.github/workflows/ci.yml`](.github/workflows/ci.yml) — `cargo fmt --check`, `cargo clippy`, `cargo test`
- Placeholder modules aligned with [ADR-0003](docs/adrs/0003-conformance-architecture.md) pipeline stages
- Workspace integration tests in [`tests/workspace.rs`](tests/workspace.rs)
- Repository maturity declared: **Workspace bootstrapped**

**Success criteria:**

- [x] Cargo workspace compiles on stable Rust
- [x] Crate boundaries and dependency graph match ADR-0002 (no cycles)
- [x] `vp-conformance` binary prints bootstrap message
- [x] CI runs fmt, clippy, and test on pull requests
- [x] No VP-CS loading, adapter execution, oracle invocation, or comparison logic yet

**Not included:**

- VP-CS fixture parsing
- `vp-spec-model` dependency wiring
- `veritypay-reference` oracle dependency
- Runner orchestration logic

### B.1 — Conformance domain types (complete)

**Goal:** Replace bootstrap placeholders in `vp-conformance-core` with immutable domain types.

**Outputs:**

- `ScenarioId`, `ScenarioBinding`, `ScenarioMetadata`, `ScenarioContext`, builders
- `ComparableResult`, `ComparableResultBuilder`, `ExecutionPath`
- `ConformanceVerdict`, `ConformanceResult`, `ConformanceResultBuilder`
- Domain tests in [`crates/vp-conformance-core/tests/domain_types.rs`](crates/vp-conformance-core/tests/domain_types.rs)

**Success criteria:**

- [x] Builders validate required fields and produce immutable objects
- [x] `ComparableResult` models a single execution path (reference or implementation)
- [x] `ConformanceResult` records verdict, oracle result, and implementation result
- [x] Core objects construct without filesystem paths

**Not included:**

- VP-CS fixture parsing or `ScenarioLoader` implementation
- Adapter execution, oracle invocation, or comparison logic

### B.2 — Scenario loader scaffold (complete)

**Goal:** Load one minimal local scenario fixture into [`ScenarioContext`](crates/vp-conformance-core/src/scenario_context.rs).

**Outputs:**

- `ScenarioLoader`, `ScenarioLoadOptions`, `ScenarioLoadError` in `vp-conformance-scenarios`
- Minimal local TOML fixture format and example [`minimal.toml`](crates/vp-conformance-scenarios/tests/fixtures/minimal.toml)
- Loader tests in [`crates/vp-conformance-scenarios/tests/scenario_loader.rs`](crates/vp-conformance-scenarios/tests/scenario_loader.rs)

**Success criteria:**

- [x] Valid minimal fixture loads into path-free `ScenarioContext`
- [x] Missing required fields and invalid bindings fail with actionable errors
- [x] Evidence `claim_id` linkage validated against claim
- [x] Optional metadata loads when present
- [x] Claim and evidence constructed through `vp-reference-model` builders

**Not included:**

- VP-CS registry lookup or `veritypay-spec` checkout integration
- Full fixture catalog
- Adapter execution, oracle invocation, comparison, reports, or CLI commands

### Spec-backed VP-CS loading (complete — G.2)

**Outputs:**

- **ScenarioLoader** accepts spec-published fixtures from `veritypay-spec/spec/conformance/scenarios/`
- RFC field aliases (`claim_id`, `evidence_id`, `protocol_version`) through existing parser
- Integration tests in [`spec_scenario_loader.rs`](crates/vp-conformance-scenarios/tests/spec_scenario_loader.rs)

**Success criteria:**

- [x] At least one documented local scenario fixture loads successfully (B.2)
- [x] Malformed fixture input fails with actionable load errors (B.2)
- [x] Loaded scenario binds specification version or Edition pin (B.2)
- [x] Fixture loading from `veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml`
- [x] No normative scenario fields invented beyond accepted spec documents

**Not included:**

- VP-CS registry lookup or automatic catalog discovery
- Adapter contract changes (Milestone C)
- Reference oracle execution changes (Milestone D)
- Outcome comparison changes (Milestone E)

---

## Milestone C — Adapter contract

**Goal:** Define and demonstrate the **implementation adapter** boundary—and the **shared result shape** used by all execution paths.

**Prerequisite:** Milestone B — loaded scenario representation (B.2 local fixture loader complete).

### C.1 — Adapter contract (complete)

**Goal:** Define and demonstrate the implementation adapter boundary.

**Outputs:**

- `ImplementationAdapter`, `AdapterId`, `AdapterError`, `AdapterRunOptions` in `vp-conformance-core`
- `build_implementation_result` helper for implementation-path [`ComparableResult`](crates/vp-conformance-core/src/comparable_result.rs)
- `StubAdapter` in `vp-conformance-adapter`
- Tests in [`crates/vp-conformance-adapter/tests/stub_adapter.rs`](crates/vp-conformance-adapter/tests/stub_adapter.rs)

**Success criteria:**

- [x] Adapter accepts loaded scenario input per contract (`run(&ScenarioContext)`)
- [x] Adapter returns implementation result in the shared comparable shape
- [x] Result path is `ExecutionPath::ImplementationAdapter` with adapter id
- [x] At least one stub adapter demonstrates the boundary
- [x] Harness core does not embed product-specific verification logic
- [x] Adapter execution does not require filesystem paths

**Not included:**

- Reference oracle wiring (Milestone D)
- Outcome comparison (Milestone E)
- Production implementation integrations
- Runner orchestration

**Milestone status:** **Complete** (C.1 satisfies adapter contract success criteria).

**Not included:**

- Reference oracle wiring (Milestone D)
- Outcome comparison (Milestone E)
- Production implementation integrations
- CI publishing (Milestone G)

---

## Milestone D — Run reference oracle

**Goal:** Invoke **`veritypay-reference`** as the **default oracle** for expected outcomes—using the same comparable result shape as adapters.

**Prerequisite:** Milestone C — adapter contract and shared result shape; [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) public contract.

### D.1 — Reference oracle (complete)

**Goal:** Introduce `ReferenceOracle` invoking the stable reference interpreter public contract.

**Outputs:**

- `ReferenceOracle` and `OracleError` in `vp-conformance-runner`
- `ScenarioContext` → `EvaluationContext` → `Interpreter::evaluate` → `ComparableResult` mapping
- Tests in [`crates/vp-conformance-runner/tests/reference_oracle.rs`](crates/vp-conformance-runner/tests/reference_oracle.rs)
- Dependencies on `vp-reference-core` and `vp-reference-interpreter` only (not `vp-reference-cli`)

**Success criteria:**

- [x] Oracle invokes reference interpreter without fork-specific glue
- [x] Oracle output uses the same comparable result shape as Milestone C adapters
- [x] `ExecutionPath::ReferenceOracle` set on oracle results
- [x] Outcomes `satisfied`, `not_satisfied`, and `indeterminate` map correctly
- [x] Claim id and specification binding preserved in `ComparableResult`
- [x] No verification rules reimplemented in the conformance repo
- [x] Oracle evaluation does not require filesystem paths

**Not included:**

- Comparison logic (Milestone E)
- Runner orchestration of adapter + oracle (complete in D.2)
- Fixture-driven suite execution

### D.2 — Runner orchestration (complete)

**Goal:** Introduce `ConformanceRunner` coordinating oracle and adapter execution.

**Outputs:**

- `ConformanceRunner::run(&ScenarioContext) -> Result<RunnerResult, RunnerError>`
- `RunnerResult` with `oracle_result` and `implementation_result` only
- `RunnerError` distinguishing oracle vs adapter failures
- Tests in [`crates/vp-conformance-runner/tests/runner_orchestration.rs`](crates/vp-conformance-runner/tests/runner_orchestration.rs)

**Success criteria:**

- [x] Runner invokes reference oracle then implementation adapter
- [x] Runner returns both `ComparableResult` values without comparison
- [x] Oracle and adapter failures propagate distinctly
- [x] Runner remains filesystem-independent

**Not included:**

- Comparison logic (Milestone E)
- `ConformanceResult` / verdict assignment
- CLI or report wiring

**Milestone status:** **Complete** (D.1 + D.2).

**Not included:**

- Implementation under test beyond stub adapters (Milestone C)
- Comparison logic (Milestone E)
- Full VP-CS catalog coverage

---

## Milestone E — Compare implementation output

**Goal:** **Compare** two execution results—implementation (via adapter, Milestone C) and reference oracle (Milestone D).

**Prerequisite:** Milestone C — adapter contract and shared result shape; Milestone D — reference oracle wired to the same comparable result shape.

**Outputs:**

- **ComparisonEngine** and **ConformanceResult** per [ADR-0003](docs/adrs/0003-conformance-architecture.md) (pass/fail, expected vs actual outcome, trace differences, metadata)
- Outcome mismatch detection (`satisfied` / `not_satisfied` / `indeterminate`)

### E.1 — Comparison engine (complete)

**Goal:** Introduce `ComparisonEngine` comparing oracle and implementation `ComparableResult` values into `ConformanceResult`.

**Outputs:**

- `ComparisonEngine::compare` in `vp-conformance-runner`
- Outcome, evaluated claim id, and specification binding comparison
- Mismatch notes in `ConformanceResult::comparison_notes`
- Tests in [`crates/vp-conformance-runner/tests/comparison_engine.rs`](crates/vp-conformance-runner/tests/comparison_engine.rs)

**Success criteria:**

- [x] Matching outcomes produce pass
- [x] Divergent outcomes produce fail with clear mismatch reason
- [x] Comparison does not introduce non-normative outcome labels
- [x] Claim id and specification binding compared where scenario requires
- [x] Comparison does not mutate either `ComparableResult`
- [x] Trace diff deferred with placeholder note only

**Not included:**

- Runner orchestration changes
- Full trace diff (may expand in Milestone F)
- Conformance report formatting (Milestone F)
- Skip/error verdict assignment from comparison
- Legal certification semantics

**Milestone status:** **Complete** (E.1).

---

## Milestone F — Produce conformance report

**Goal:** Emit **human and machine-readable conformance reports**.

**Prerequisite:** Milestone E — `ConformanceResult` records from comparison.

**Outputs:**

- **Report** formatting from **ConformanceResult** records per [ADR-0003](docs/adrs/0003-conformance-architecture.md)
- Human-readable output for local development
- Structured export suitable for CI consumption
- Exit codes mapped to pass/fail summary

### F.1 — Conformance report aggregation (complete)

**Goal:** Introduce `ConformanceReport` summarizing one or more `ConformanceResult` records.

**Outputs:**

- `ConformanceReport` and `ConformanceReportBuilder` in `vp-conformance-report`
- Derived counts: total, passed, failed, skipped, errors
- Summary helpers: `has_failures()`, `has_errors()`, `success_rate()`
- Tests in [`crates/vp-conformance-report/tests/conformance_report.rs`](crates/vp-conformance-report/tests/conformance_report.rs)

**Success criteria:**

- [x] Report aggregates multiple `ConformanceResult` values immutably
- [x] Verdict counts derived automatically from results
- [x] `success_rate()` returns `passed / total` (0.0 when empty)
- [x] Report does not alter verification outcomes

**Not included:**

- CLI rendering
- JSON serialization
- HTML reports
- File output
- CI integration

### F.2 — Human report renderer (complete)

**Goal:** Render `ConformanceReport` as a deterministic plain-text summary for local development.

**Outputs:**

- `HumanReportRenderer::render` in `vp-conformance-report`
- Summary block with counts and success rate
- Per-scenario results in report order with mismatch notes for failures
- Tests in [`crates/vp-conformance-report/tests/human_report_renderer.rs`](crates/vp-conformance-report/tests/human_report_renderer.rs)

**Success criteria:**

- [x] Renderer produces stable plain-text output without ANSI colors
- [x] Results preserve report order without sorting
- [x] Failures surface comparison mismatch notes in human-readable form
- [x] `ConformanceReport` remains presentation-independent

**Not included:**

- JSON serialization
- CLI integration
- HTML or markdown export
- File output

### F.3 — JSON report renderer (complete)

**Goal:** Render `ConformanceReport` as stable machine-readable JSON for CI and automation.

**Outputs:**

- `JsonReportRenderer::render` in `vp-conformance-report`
- `ReportRenderError` for serialization failures
- Summary and per-scenario results with oracle/implementation summaries
- Tests in [`crates/vp-conformance-report/tests/json_report_renderer.rs`](crates/vp-conformance-report/tests/json_report_renderer.rs)

**Success criteria:**

- [x] JSON uses deterministic field names and report order
- [x] Verdicts and outcomes use lowercase string values
- [x] Output parses as valid JSON with `serde_json`
- [x] `ConformanceReport` remains presentation-independent

**Not included:**

- CLI integration
- File output
- CI workflows
- Schema registry

**Success criteria:**

- [x] Full suite run produces a single report artifact (`ConformanceReport`)
- [x] Report lists per-scenario results with oracle vs implementation summary
- [x] Report does not alter verification outcomes
- [x] Reviewer can understand failures without reading harness source first

**Milestone status:** **Complete** (F.1 + F.2 + F.3).

**Not included:**

- CI workflow publishing (Milestone G)
- Dashboard or web UI
- Cross-repository analytics

---

## Milestone G — CI integration

**Goal:** Run the conformance suite in **continuous integration** pipelines.

**Prerequisite:** Milestone B — scenario loading; Milestone C — adapter contract; Milestone D — reference oracle; Milestone E — comparison; Milestone F — conformance report; [ADR-0004](docs/adrs/0004-conformance-public-contract.md) — public contract (`ScenarioContext` → `ConformanceRunner::run` → `ConformanceResult`); [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) oracle contract.

**Outputs:**

- CI workflow or documented integration pattern
- Readiness gate script (fmt, test, smoke run—language TBD)
- Documentation for running against sibling `veritypay-spec` and `veritypay-reference`

### G.1 — CLI run command (complete)

**Goal:** Expose the conformance pipeline through `vp-conformance run`.

**Outputs:**

- `vp-conformance run --scenario <path> --adapter stub --adapter-outcome <outcome> --format human|json`
- Pipeline wiring: load → runner → compare → report → render
- Exit codes: `0` pass, `1` failure, `2` user error, `3` harness error
- Tests in [`crates/vp-conformance-cli/tests/run_command.rs`](crates/vp-conformance-cli/tests/run_command.rs)

**Success criteria:**

- [x] CLI runs one scenario through existing pipeline components
- [x] Human and JSON report formats supported
- [x] Matching stub outcome exits `0`; mismatch exits `1`
- [x] Invalid input and missing fixtures exit `2`

**Not included:**

- CI workflow changes
- Multi-scenario suite discovery
- External implementation adapters
- File output

### G.2 — Spec-published scenario loading (complete)

**Goal:** Load normative VP-CS scenarios from `veritypay-spec` through the existing `ScenarioLoader`.

**Outputs:**

- Spec fixture field aliases in [`fixture.rs`](crates/vp-conformance-scenarios/src/fixture.rs)
- Readiness gate smoke against `../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml`
- Optional sibling integration tests (skip when checkout absent)

**Success criteria:**

- [x] `vp-conformance run --scenario ../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml` loads and executes
- [x] Local fixture tests unchanged for isolated unit coverage
- [x] No duplicate parsing logic; single loader path for local and spec fixtures
- [x] Conformance executes specification-published VP-CS scenarios

**Not included:**

- VP-CS registry-backed catalog discovery
- Multi-scenario suite runs
- Comparison, oracle, or adapter changes

### G.3 — Readiness gate (complete)

**Goal:** Provide a single script verifying repository health before merge or sibling-repo use.

**Outputs:**

- [`scripts/readiness-gate.sh`](scripts/readiness-gate.sh) — fmt, clippy, test, CLI boot, smoke run
- README readiness gate documentation

**Success criteria:**

- [x] Script runs fmt, clippy, and workspace tests in order
- [x] Script smoke-runs `vp-conformance run` against spec-published **VP-CS-0001** when sibling `veritypay-spec` is present
- [x] Script skips smoke run with clear message when spec fixture is absent
- [x] Script exits non-zero on any failing step

**Not included:**

- GitHub Actions workflow changes
- Additional validation logic
- New CLI commands

### G.4 — Repository readiness (complete)

**Goal:** Declare `veritypay-conformance` ready for downstream repositories.

**Outputs:**

- README repository maturity: **Conformance Platform Ready**
- README capability table and repository readiness criteria
- ROADMAP current status: **Repository Ready**

**Success criteria:**

- [x] Delivered pipeline capabilities documented with checkmarks
- [x] Repository readiness criteria match sibling-repo pattern (readiness gate + public contract + smoke)
- [x] Deferred work (catalog loading, org-wide CI) explicitly called out

**Not included:**

- GitHub Actions workflow expansion
- Normative VP-CS authoring
- Certification semantics

**Success criteria:**

- [x] CI invokes harness through [ADR-0004](docs/adrs/0004-conformance-public-contract.md) public contract (`vp-conformance run`)
- [x] Local and documented readiness gate runs conformance smoke against spec-published **VP-CS-0001** when present
- [x] Failure surfaces via exit codes suitable for CI (`0` / `1` / `2` / `3`)
- [x] Clear skip behavior when spec fixture is absent (readiness gate message)
- [x] Local harness fixtures retained for unit tests only

**Milestone status:** **Complete** (G.1 + G.2 + G.3 + G.4).

### G.5 — Execute VP-CS-0002 (complete)

**Goal:** Run the published **VP-CS-0002** fixture using the reference interpreter implementing **VP-RULE-0002**.

**Prerequisite:** [VP-RFC-0002](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0002-claim-identity-binding.md) fixture in `veritypay-spec`; `veritypay-reference` **VP-RULE-0002** (Milestone D.4).

**Outputs:**

- Readiness gate smoke for `../veritypay-spec/spec/conformance/scenarios/VP-CS-0002.toml`
- Integration tests: matching stub `indeterminate` → PASS; mismatched stub `satisfied` → FAIL
- README Platform 1.0 scenario set documents **VP-CS-0001** and **VP-CS-0002**

**Success criteria:**

- [x] `vp-conformance run --scenario ../veritypay-spec/spec/conformance/scenarios/VP-CS-0002.toml --adapter stub --adapter-outcome indeterminate` returns PASS
- [x] Oracle returns `indeterminate` for **VP-CS-0002** fixture inputs
- [x] Stub adapter outcome mismatch surfaces as conformance failure
- [x] No comparison engine or adapter API changes

**Not included:**

- Multi-scenario suite discovery
- VP-RFC-0002 acceptance in spec
- External implementation adapters

**Milestone status:** **Complete**.

### G.6 — Execute VP-CS-0011–0013 (Platform 1.3 normalized_text)

**Goal:** Run published **`normalized_text`** fixtures (**VP-CS-0011**–**0013**) through the existing pipeline with no CLI or adapter changes.

**Prerequisite:** **VP-CS-0011**–**0013** in `veritypay-spec`; `veritypay-reference` **VP-RULE-0011** and **NormalizedTextEvaluator** (Platform 1.3).

**Outputs:**

- Readiness gate smoke for **VP-CS-0011** (`satisfied`), **VP-CS-0012** (`not_satisfied`), **VP-CS-0013** (`indeterminate`)
- Integration tests: matching stub outcomes → PASS; intentional mismatches → FAIL
- Spec loader tests asserting `assertion_type = normalized_text` flows unchanged into the reference oracle
- README Platform 1.3 semantic coverage table

**Success criteria:**

- [x] `vp-conformance run` executes **VP-CS-0011**–**0013** with matching stub outcomes
- [x] Oracle dispatches **`normalized_text`** via reference interpreter (no conformance-side filtering)
- [x] Stub adapter outcome mismatch surfaces as conformance failure with unchanged exit codes
- [x] No comparison engine, adapter API, or CLI changes

**Not included:**

- VP-RFC-0011 acceptance in spec
- Platform 1.3 platform release declaration (Platform 1.2 remains current)
- Multi-scenario suite discovery
- External implementation adapters

**Milestone status:** **Complete**.

**Not included:**

- Hosted conformance-as-a-service
- Product release certification
- Implementing verification rules locally
- Organization-wide reusable CI workflows (deferred to maintenance)

---

## Repository readiness criteria

Downstream repositories may depend on `veritypay-conformance` when all criteria below are met. This mirrors the readiness pattern in [`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling) and [`veritypay-reference`](https://github.com/VerityPay-Inc/veritypay-reference): workspace health, a readiness gate script, a documented public contract, and a smoke path against representative inputs.

| Criterion | Evidence |
|-----------|----------|
| Workspace builds and tests pass | `cargo test --workspace`; CI `fmt` / `clippy` / `test` |
| Readiness gate available | [`scripts/readiness-gate.sh`](scripts/readiness-gate.sh) |
| Conformance pipeline wired end-to-end | `vp-conformance run` (G.1) |
| Reports suitable for local and CI review | `HumanReportRenderer`, `JsonReportRenderer` (F.2, F.3) |
| Public contract declared | [ADR-0004](docs/adrs/0004-conformance-public-contract.md) |
| Reference oracle baseline documented | [veritypay-reference ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) |
| Spec-published VP-CS smoke | `../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml`, `VP-CS-0002.toml`, **VP-CS-0011**–**0013** (G.2, G.5, G.6) |
| Platform 1.3 semantic coverage | **`normalized_text`** via **VP-CS-0011**–**0013**; **2** reference evaluators; **5** VP-CS fixtures |

**Explicitly deferred:** VP-CS registry-backed catalog discovery, multi-scenario suite discovery, external implementation adapters, org-wide reusable GitHub Actions workflows.

---

## After Milestone G

The conformance suite enters **maintenance and extension** mode: broader VP-CS coverage, richer trace comparison, and Edition-aware scenario sets as spec governance defines them.

### Platform 1.3 — normalized_text conformance (G.6)

Draft [VP-RFC-0011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0011-normalized-text-assertion.md) introduces **`normalized_text`** evaluated by **VP-RULE-0011**. **Platform 1.3** remains **in progress** in `veritypay-spec`; **Platform 1.2** remains the current platform release.

| Principle | Detail |
|-----------|--------|
| **Compare semantics** | Conformance compares **evaluator behavior** and verification outcomes — not evaluator implementation architecture |
| **Oracle baseline** | `veritypay-reference` dispatches **`body_equality`** and **`normalized_text`** via **2** evaluators per [ADR-0009](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0009-assertion-evaluator-architecture.md) |
| **Fixtures** | **VP-CS-0011**–**0013** published and executable; **VP-CS-0001** / **VP-CS-0002** unchanged |
| **Pipeline** | `assertion_type` flows unchanged: ScenarioLoader → ReferenceOracle → reference dispatch |

No CLI or adapter changes were required — the existing execution pipeline reuses spec-published fixtures.

**Explicitly deferred** (see [ADR-0003 — Future extensions](docs/adrs/0003-conformance-architecture.md#future-extensions)):

- Parallel runners and batch execution
- Golden report snapshots
- SDK surface for integrators
- Runtime plugin loading of implementations
- Replacing `veritypay-reference` as oracle without governance process
- Normative scenario authoring in this repository

---

## How to propose roadmap changes

Roadmap changes are **conformance harness governance**, not protocol changes.

1. Open an issue describing the capability gap
2. Confirm the change does not require normative VP-CS edits in `veritypay-spec` without an upstream RFC
3. Propose milestone text (goal, outputs, success criteria, not included)
4. Land documentation or ADR before large code drops

---

*Run scenarios. Oracle from reference. Compare honestly. Report clearly.*
