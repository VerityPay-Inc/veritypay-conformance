---
id: ADR-0003
title: Conformance Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0001-implementation-language.md
  - docs/adrs/0002-cargo-workspace-architecture.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-06
superseded_by: null
---

# ADR-0003 — Conformance Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-06

**Related:** [ADR-0001](0001-implementation-language.md) · [ADR-0002](0002-cargo-workspace-architecture.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) · [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)

---

## Purpose

Record the **platform conformance pipeline**—component boundaries, data flow, and ownership rules—before Milestone B implementation begins.

---

## Context

[Milestone A](../../ROADMAP.md) established repository purpose, contribution boundaries, and a conceptual pipeline in [ARCHITECTURE.md](../../ARCHITECTURE.md).

[ADR-0001](0001-implementation-language.md) chose **Rust**. [ADR-0002](0002-cargo-workspace-architecture.md) mapped pipeline stages to **`vp-conformance-*`** crates.

Before code lands, the project must fix **what each stage does**, **what it must not do**, and **where normative truth lives**:

| Source | Owns |
|--------|------|
| **`veritypay-spec`** | VP-CS scenario **meaning** — inputs, bindings, verification questions |
| **`veritypay-reference`** | **Expected outcomes** via the reference oracle |
| **`veritypay-conformance`** | **Comparison and reporting** — pass/fail evidence, not protocol semantics |

This ADR is the **engineering contract** for the harness pipeline. It does **not** amend `veritypay-spec` or redefine verification rules.

---

## Decision

**Adopt the following platform pipeline** as the canonical conformance architecture.

```
Scenario
    ↓
ScenarioLoader
    ↓
ScenarioContext
    ↓
ImplementationAdapter
          ↓
ReferenceOracle
          ↓
ComparisonEngine
          ↓
ConformanceResult
          ↓
Report
```

### Execution clarification

**`ImplementationAdapter`** and **`ReferenceOracle`** are **sibling execution paths**. Both consume **`ScenarioContext`** and each produces a **comparable result**. Neither path defines protocol meaning or expected outcomes by itself.

The diagram shows orchestration order in the runner; it does **not** mean the oracle consumes adapter output. **`ComparisonEngine`** receives **both** results and determines agreement.

```
ScenarioContext
       ├──────────────────┬──────────────────┐
       ▼                  ▼                  │
ImplementationAdapter   ReferenceOracle       │
       │                  │                  │
       └────────┬─────────┘                  │
                ▼                            │
         ComparisonEngine                    │
                ▼                            │
         ConformanceResult                   │
                ▼                            │
              Report ◄───────────────────────┘
```

**`Report`** formats **`ConformanceResult`** records—it does not re-execute verification or alter outcomes.

---

## Platform rules

| Rule | Detail |
|------|--------|
| **Conformance never defines protocol meaning** | Outcome vocabulary, claim semantics, and scenario intent come from accepted spec documents |
| **The oracle comes from `veritypay-reference`** | Expected outcomes via `Interpreter::evaluate(&EvaluationContext) -> VerificationResult` per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) |
| **Scenario meaning comes from `veritypay-spec`** | VP-CS fixtures are loaded, not authored, in this repository |
| **Comparison never modifies outcomes** | **`ComparisonEngine`** classifies agreement; it does not coerce, patch, or re-label results |

---

## Component responsibilities

### Scenario

| Field | Definition |
|-------|------------|
| **Purpose** | Normative conformance unit from **`veritypay-spec`** |
| **Responsibilities** | Define **expected inputs only** — claim, evidence, specification binding, scenario identity |
| **Does not belong** | Expected verification outcomes; harness comparison policy; implementation-specific types |
| **Owner** | **`veritypay-spec`** (authored upstream) |

The harness **loads** scenarios. It does **not** rewrite normative VP-CS prose or invent scenario fields.

---

### ScenarioLoader

| Field | Definition |
|-------|------------|
| **Purpose** | Ingest VP-CS fixtures into harness-ready records |
| **Responsibilities** | Resolve scenario paths or IDs; load fixture metadata; bind specification or Edition pins; surface load errors before execution |
| **Does not belong** | Verification execution; comparison; report formatting; registry validation (`veritypay-tooling`) |
| **Crate mapping** | **`vp-conformance-scenarios`** ([ADR-0002](0002-cargo-workspace-architecture.md)) |

Loaded scenarios are **immutable** after construction.

---

### ScenarioContext

| Field | Definition |
|-------|------------|
| **Purpose** | Path-free, harness-stable view of a loaded scenario for execution |
| **Responsibilities** | Carry scenario identity, specification binding, and execution inputs prepared for adapter and oracle paths |
| **Does not belong** | Verification outcomes; comparison verdicts; filesystem handles in execution contracts |
| **Crate mapping** | **`vp-conformance-core`** ([ADR-0002](0002-cargo-workspace-architecture.md)) |

**`ScenarioContext`** is the **handoff point** from loading to execution. Both execution paths read the same context for a given run.

---

### ImplementationAdapter

| Field | Definition |
|-------|------------|
| **Purpose** | Invoke the **implementation under test** |
| **Responsibilities** | Accept harness-prepared input from **`ScenarioContext`**; call the implementation (library, CLI, or subprocess—integration TBD); return a **comparable result** (outcome, binding, optional trace) |
| **Does not belong** | VP-CS semantics; parsing scenario fixtures; defining expected outcomes; comparison logic |
| **Crate mapping** | **`vp-conformance-adapter`** ([ADR-0002](0002-cargo-workspace-architecture.md)) |

The adapter **calls the implementation**. It does **not** call the reference oracle.

---

### ReferenceOracle

| Field | Definition |
|-------|------------|
| **Purpose** | Produce **expected** verification outcomes |
| **Responsibilities** | Build **`EvaluationContext`** from **`ScenarioContext`**; **call `veritypay-reference`** per public contract; map **`VerificationResult`** to the same comparable result shape as adapters |
| **Does not belong** | Reimplementing verification rules locally; substituting ad hoc expected outcomes; modifying results after evaluation |
| **Crate mapping** | Oracle path in **`vp-conformance-runner`** ([ADR-0002](0002-cargo-workspace-architecture.md)) |

The oracle **calls `veritypay-reference`**. It is not a second adapter for the implementation under test.

---

### ComparisonEngine

| Field | Definition |
|-------|------------|
| **Purpose** | Determine whether implementation and oracle **agree** |
| **Responsibilities** | Compare comparable results from both execution paths; classify outcome agreement (`satisfied`, `not_satisfied`, `indeterminate`); detect binding or identity drift where scenarios require; record trace differences when milestones support them |
| **Does not belong** | Introducing non-normative outcome labels; mutating either execution result; re-running verification |
| **Crate mapping** | Comparison logic in **`vp-conformance-runner`**; record types in **`vp-conformance-core`** |

**Comparison never modifies outcomes.** Mismatch explanation is evidence for **`ConformanceResult`**—not a new verification verdict.

---

### ConformanceResult

| Field | Definition |
|-------|------------|
| **Purpose** | Frozen per-scenario record of a conformance check |
| **Responsibilities** | Record the following for each scenario run: |

| Field | Source |
|-------|--------|
| **pass/fail** | **`ComparisonEngine`** classification |
| **expected outcome** | Reference oracle comparable result |
| **actual outcome** | Implementation adapter comparable result |
| **trace differences** | Diff summary when traces are present and comparable |
| **metadata** | Scenario id, specification pin, run identifiers, timestamps, adapter identity |

| **Does not belong** | Live execution handles; mutated outcomes; normative scenario definitions |
| **Crate mapping** | **`vp-conformance-core`** ([ADR-0002](0002-cargo-workspace-architecture.md)) |

**`ConformanceResult`** is **immutable** once comparison completes. Downstream stages read it; they do not rewrite it.

---

### Report

| Field | Definition |
|-------|------------|
| **Purpose** | Present conformance evidence for humans and CI |
| **Responsibilities** | Aggregate **`ConformanceResult`** records; human-readable summaries; structured export; map suite summary to exit codes (CLI wiring) |
| **Does not belong** | Re-execution; outcome mutation; protocol semantics |
| **Crate mapping** | **`vp-conformance-report`** ([ADR-0002](0002-cargo-workspace-architecture.md)) |

---

## Data ownership

| Concept | Owner |
|---------|-------|
| VP-CS scenario meaning | `veritypay-spec` |
| Expected `VerificationResult` | `veritypay-reference` |
| Implementation actual result | Implementation under test (via adapter) |
| Pass/fail and comparison evidence | `veritypay-conformance` |
| Outcome vocabulary | `veritypay-spec` |

---

## Milestone mapping

| Pipeline stage | Roadmap milestone |
|----------------|-------------------|
| ScenarioLoader, ScenarioContext | **B** — Load scenario fixtures |
| ImplementationAdapter, comparable result shape | **C** — Adapter contract |
| ReferenceOracle | **D** — Run reference oracle |
| ComparisonEngine, ConformanceResult | **E** — Compare implementation output |
| Report | **F** — Produce conformance report |
| CI invocation | **G** — CI integration |

Orchestration across stages lives in **`vp-conformance-runner`** per [ADR-0002](0002-cargo-workspace-architecture.md).

---

## Future extensions

Deferred until milestones or scale justify them:

| Extension | Notes |
|-----------|-------|
| **Parallel runners** | Execute independent scenarios concurrently; preserve per-scenario **`ConformanceResult`** immutability |
| **CI** | Pipeline integration via future **`vp-conformance-ci`** crate |
| **Batch execution** | Suite-wide runs with selective scenario filters |
| **Golden reports** | Snapshot or fixture-backed report artifacts for regression detection |

Each extension must preserve platform rules: no protocol semantics in the harness, oracle from reference, comparison without outcome mutation.

---

## Alternatives considered

### 1. Adapter output feeds the oracle

Run reference evaluation only after the implementation returns.

**Rejected.** Oracle expected outcomes must not depend on implementation behavior. Both paths read the same **`ScenarioContext`**.

### 2. Comparison inside Report

Formatting stage also classifies pass/fail.

**Rejected.** Separates **`ConformanceResult`** (frozen evidence) from presentation (**`Report`**).

### 3. Harness defines expected outcomes

Store expected `VerificationResult` in fixtures and skip the oracle.

**Rejected.** Violates **oracle comes from `veritypay-reference`**. Fixtures define **inputs**; reference evaluation defines **expected outcomes**.

### 4. Comparison coerces close matches

Normalize implementation results to pass when "close enough."

**Rejected.** Violates **comparison never modifies outcomes**.

---

## Consequences

### Positive

- **Clear normative boundaries** — spec, reference, and harness roles are explicit
- **Stable integration point** — oracle contract is already declared in reference ADR-0007
- **Auditable failures** — **`ConformanceResult`** preserves expected vs actual with trace diffs
- **Milestone alignment** — pipeline stages map directly to ROADMAP B–G

### Negative

- **Two execution paths** — runner must keep comparable result shapes in sync
- **Oracle coupling** — harness behavior tracks `veritypay-reference` releases
- **Trace comparison complexity** — full trace diff may arrive after initial outcome-only comparison

**Acceptable** because the architecture mirrors [CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) philosophy: shared meaning, independent implementations, honest comparison.

---

## Future reconsideration

Revisit this ADR only if:

- An accepted spec change redefines conformance demonstration in ways this pipeline cannot express
- Oracle contract in `veritypay-reference` is superseded materially
- A proposed change violates **comparison never modifies outcomes** or **conformance never defines protocol meaning**

Pipeline or ownership changes require **a successor ADR**.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0002](0002-cargo-workspace-architecture.md) | Crate mapping for pipeline stages |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Conceptual architecture this ADR formalizes |
| [ROADMAP.md](../../ROADMAP.md) | Milestone delivery order |
| [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) | Oracle public contract |

---

## Follow-up

- [ ] Implement ScenarioLoader and ScenarioContext (Milestone B)
- [ ] Define comparable result shape and adapter trait (Milestone C)
- [ ] Wire ReferenceOracle to `veritypay-reference` (Milestone D)
- [ ] Implement ComparisonEngine and ConformanceResult (Milestone E)
- [ ] Emit Report from ConformanceResult records (Milestone F)

---

## Conclusion

The conformance platform loads **scenario inputs** from **`veritypay-spec`**, executes **implementation** and **reference oracle** paths from shared **`ScenarioContext`**, **compares without mutating outcomes**, records **`ConformanceResult`**, and **reports** honestly.

This ADR records architecture only. It does **not** implement code or alter normative specification text.
