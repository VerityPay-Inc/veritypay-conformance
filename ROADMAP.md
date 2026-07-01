# Roadmap

**Capability-based roadmap for `veritypay-conformance`.**

This roadmap is **not date-driven**. Milestones complete when their success criteria are met—not when a quarter ends. Progress aligns with [Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) and the conformance role defined in [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md).

**Current milestone:** **B — Load scenario fixtures** *(workspace bootstrapped; VP-CS loading not started)*

---

## Overview

| Milestone | Name | Status |
|-----------|------|--------|
| **A** | Repository scaffold | **Complete** |
| **B** | Load scenario fixtures | **In progress** |
| **C** | Adapter contract | Not started |
| **D** | Run reference oracle | Not started |
| **E** | Compare implementation output | Not started |
| **F** | Produce conformance report | Not started |
| **G** | CI integration | Not started |

Each milestone below includes **Goal**, **Outputs**, **Success criteria**, and **Not included** so scope stays explicit.

**Milestone ordering note:** The **adapter contract** (C) precedes the **reference oracle** (D) so both execution paths share a comparable result shape before comparison (E). Comparison then receives **two results**—implementation and oracle—not a chained execution pipeline. Pipeline stages are defined in [ADR-0003](docs/adrs/0003-conformance-architecture.md).

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
- [x] No VP-CS loading, oracle invocation, adapter execution, or comparison logic yet

**Not included:**

- VP-CS fixture parsing
- `vp-spec-model` dependency wiring
- `veritypay-reference` oracle dependency
- Runner orchestration logic

### VP-CS scenario loading (not started)

**Outputs:**

- **ScenarioLoader** and **ScenarioContext** per [ADR-0003](docs/adrs/0003-conformance-architecture.md) (`vp-conformance-scenarios`, `vp-conformance-core`)
- Minimal VP-CS fixture format aligned with accepted spec documents
- Load errors surfaced before oracle or adapter invocation

**Success criteria:**

- [ ] At least one documented VP-CS fixture loads successfully
- [ ] Malformed fixture input fails with actionable load errors
- [ ] Loaded scenario binds specification version or Edition pin
- [ ] No normative scenario fields invented beyond accepted spec documents

**Not included:**

- Adapter contract (Milestone C)
- Reference oracle execution (Milestone D)
- Outcome comparison (Milestone E)

---

## Milestone C — Adapter contract

**Goal:** Define and demonstrate the **implementation adapter** boundary—and the **shared result shape** used by all execution paths.

**Prerequisite:** Milestone B — loaded scenario representation.

**Outputs:**

- **ImplementationAdapter** contract per [ADR-0003](docs/adrs/0003-conformance-architecture.md) (`vp-conformance-adapter`)
- Comparable result record (outcome, binding, optional trace) shared by adapters and oracle
- Stub or minimal adapter implementation for local testing
- Adapter errors isolated from harness core

**Success criteria:**

- [ ] Adapter accepts loaded scenario input per contract
- [ ] Adapter returns implementation result in the shared comparable shape
- [ ] At least one stub adapter demonstrates the boundary
- [ ] Harness core does not embed product-specific verification logic

**Not included:**

- Reference oracle wiring (Milestone D)
- Outcome comparison (Milestone E)
- Production implementation integrations
- CI publishing (Milestone G)

---

## Milestone D — Run reference oracle

**Goal:** Invoke **`veritypay-reference`** as the **default oracle** for expected outcomes—using the same comparable result shape as adapters.

**Prerequisite:** Milestone C — adapter contract and shared result shape; [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) public contract.

**Outputs:**

- **ReferenceOracle** wiring `EvaluationContext` → `Interpreter::evaluate` → `VerificationResult` mapped to comparable result per [ADR-0003](docs/adrs/0003-conformance-architecture.md)
- Fixture-driven test that oracle produces expected outcome for a minimal scenario

**Success criteria:**

- [ ] Oracle invokes reference interpreter without fork-specific glue
- [ ] Oracle output uses the same comparable result shape as Milestone C adapters
- [ ] `VerificationResult` matches scenario expectation for at least one fixture
- [ ] Specification binding on oracle result matches loaded scenario pin
- [ ] No verification rules reimplemented in the conformance repo

**Not included:**

- Implementation under test beyond stub adapters (Milestone C)
- Comparison logic (Milestone E)
- Full VP-CS catalog coverage

---

## Milestone E — Compare implementation output

**Goal:** **Compare** two execution results—implementation (via adapter) and reference oracle.

**Prerequisite:** Milestones C and D — both paths produce comparable results.

**Outputs:**

- **ComparisonEngine** and **ConformanceResult** per [ADR-0003](docs/adrs/0003-conformance-architecture.md) (pass/fail, expected vs actual outcome, trace differences, metadata)
- Outcome mismatch detection (`satisfied` / `not_satisfied` / `indeterminate`)

**Success criteria:**

- [ ] Matching outcomes produce pass
- [ ] Divergent outcomes produce fail with clear mismatch reason
- [ ] Comparison does not introduce non-normative outcome labels
- [ ] Claim id and specification binding compared where scenario requires

**Not included:**

- Full trace diff (may expand in Milestone F)
- Conformance report formatting (Milestone F)
- Legal certification semantics

---

## Milestone F — Produce conformance report

**Goal:** Emit **human and machine-readable conformance reports**.

**Outputs:**

- **Report** formatting from **ConformanceResult** records per [ADR-0003](docs/adrs/0003-conformance-architecture.md)
- Human-readable output for local development
- Structured export suitable for CI consumption
- Exit codes mapped to pass/fail summary

**Success criteria:**

- [ ] Full suite run produces a single report artifact
- [ ] Report lists per-scenario results with oracle vs implementation summary
- [ ] Report does not alter verification outcomes
- [ ] Reviewer can understand failures without reading harness source first

**Not included:**

- CI workflow publishing (Milestone G)
- Dashboard or web UI
- Cross-repository analytics

---

## Milestone G — CI integration

**Goal:** Run the conformance suite in **continuous integration** pipelines.

**Outputs:**

- CI workflow or documented integration pattern
- Readiness gate script (fmt, test, smoke run—language TBD)
- Documentation for running against sibling `veritypay-spec` and `veritypay-reference`

**Success criteria:**

- [ ] CI runs conformance suite on pull requests or scheduled basis
- [ ] Failure blocks merge when configured scenarios mismatch oracle
- [ ] Clear skip behavior when sibling repos or adapters are absent
- [ ] At least VP-CS-0001 or agreed minimal scenario included when available upstream

**Not included:**

- Hosted conformance-as-a-service
- Product release certification
- Implementing verification rules locally

---

## After Milestone G

The conformance suite enters **maintenance and extension** mode: broader VP-CS coverage, richer trace comparison, and Edition-aware scenario sets as spec governance defines them.

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
