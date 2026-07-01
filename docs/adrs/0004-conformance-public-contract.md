---
id: ADR-0004
title: Conformance Public Contract
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0002-cargo-workspace-architecture.md
  - docs/adrs/0003-conformance-architecture.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-06
superseded_by: null
---

# ADR-0004 — Conformance Public Contract

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-06

**Related:** [ADR-0002](0002-cargo-workspace-architecture.md) · [ADR-0003](0003-conformance-architecture.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) · [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)

---

## Purpose

Define the **stable public API** that external callers rely on—before CI integration, downstream automation, and internal runner refactors proliferate around ad hoc signatures.

This contract should **intentionally outlive internal runner refactors**.

> **This ADR defines an engineering contract for the conformance harness.** It does **not** define VerityPay protocol semantics or amend `veritypay-spec`.

---

## Context

[ADR-0003](0003-conformance-architecture.md) recorded the platform pipeline: scenario load → execution paths → comparison → report.

The workspace is bootstrapped per [ADR-0002](0002-cargo-workspace-architecture.md). Upcoming milestones will wire **ScenarioLoader**, **ImplementationAdapter**, **ReferenceOracle**, and **ComparisonEngine** behind **`vp-conformance-runner`**.

External consumers—**CLI**, **CI**, **GitHub Actions**, future **`veritypay-ci`**, and **documentation examples**—need a declared surface that answers:

- What input do I pass?
- What method do I call?
- What result do I receive?

Without a contract, each consumer risks depending on adapter traits, comparison internals, or crate layout that will change as milestones B–F land.

---

## Stable contract

```
ScenarioContext
        ↓
ConformanceRunner::run(...)
        ↓
ConformanceResult
```

Future milestones may add **parallel execution**, **batch runs**, **adapter registries**, and **richer trace comparison**—but should **preserve this core call shape** unless a **successor ADR** explicitly supersedes it.

---

## Decision

**Adopt the call shape above as the stable public contract** between external callers and **`vp-conformance-runner`**.

Callers invoke **`ConformanceRunner::run`** with a **`ScenarioContext`**. The runner owns adapter selection, oracle invocation, and comparison internally. Callers receive a frozen **`ConformanceResult`**.

---

## 1. Stable input

**`ScenarioContext`** (`vp-conformance-core`) is the **single public run input**.

| Requirement | Detail |
|-------------|--------|
| **ScenarioContext only** | No parallel positional parameters (`scenario_id`, `claim`, `paths`, …) on the public entrypoint |
| **Path-free** | No `Path`, `PathBuf`, or live filesystem handles in the contract |
| **Immutable** | Inputs do not mutate during `run` |
| **Specification binding resolved** | Edition or protocol version pin is already bound on the context before `run` |

| Field (conceptual) | Role |
|--------------------|------|
| **`scenario_id`** | Stable VP-CS identifier |
| **`specification_binding`** | Resolved specification or Edition pin for this run |
| **Execution inputs** | Claim, evidence, and harness-prepared material required by adapter and oracle paths |

Scenario loading, CLI path resolution, and VP-CS fixture parsing remain **outside** this contract ([ADR-0002](0002-cargo-workspace-architecture.md), [ADR-0003](0003-conformance-architecture.md)).

Callers that need filesystem access (CLI, CI checkout steps) resolve paths **upstream** and construct **`ScenarioContext`** through **`ScenarioLoader`** or future builders—not inside `run`.

---

## 2. Stable entrypoint

| Element | Contract |
|---------|----------|
| **Type** | **`ConformanceRunner`** (`vp-conformance-runner`) |
| **Method** | **`run(...)`** — accepts **`ScenarioContext`** (and future run options on a dedicated options type, not new overloads) |
| **Return** | **`ConformanceResult`** (or a typed error boundary documented alongside it) |
| **Orchestration** | Runner owns adapter execution, oracle invocation, and comparison internally |
| **Caller stability** | Callers depend on **input/output types and method signature**—not on internal stage wiring |

| Caller rule | Detail |
|-------------|--------|
| **Never invoke adapters directly** | Adapter execution is runner-internal |
| **Never invoke comparison directly** | **`ComparisonEngine`** is runner-internal |
| **Never invoke oracle directly** | **`ReferenceOracle`** is runner-internal (oracle itself calls `veritypay-reference` per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)) |

The runner **does not** expose a growing list of stage methods (`run_adapter`, `run_oracle`, `compare`, …) as the public API. Future run knobs extend a **`RunOptions`** (or equivalent) type passed to `run`—not new public orchestration entrypoints.

Internal evolution (parallel runners, adapter registry, comparison algorithm changes) must not break **`ScenarioContext` → `ConformanceRunner::run` → `ConformanceResult`** without ADR supersession.

---

## 3. Stable output

**`ConformanceResult`** (`vp-conformance-core`) is the **root conformance artifact** returned by `run`.

| Field (conceptual) | Role |
|--------------------|------|
| **Verdict** | **`pass`**, **`fail`**, **`skip`**, or **`error`** — harness classification, not a new protocol outcome label |
| **Oracle result** | Expected comparable result from the reference path |
| **Implementation result** | Actual comparable result from the adapter path |
| **Comparison metadata** | Mismatch reasons, trace diff summary, binding checks, run identifiers |
| **`scenario_id`** | Links result to the VP-CS scenario under test |
| **`specification_binding`** | Pin used for this run |
| **Timestamps / run metadata** | Non-normative audit context (must not decide protocol truth) |

### Verdict vocabulary

| Value | Meaning |
|-------|---------|
| **`pass`** | Implementation and oracle agree per comparison policy |
| **`fail`** | Implementation and oracle disagree on a compared field |
| **`skip`** | Scenario or adapter not applicable for this run (documented reason in metadata) |
| **`error`** | Harness or execution failure before a reliable comparison (adapter error, oracle error, load error propagated from upstream) |

Harness verdicts are **comparison evidence**—distinct from normative verification **`Outcome`** vocabulary (`satisfied`, `not_satisfied`, `indeterminate`) owned by `veritypay-spec`.

**`ConformanceResult`** is **immutable** after `run` returns. **`Report`** and CI formatters read it—they do not mutate it ([ADR-0003](0003-conformance-architecture.md)).

---

## 4. Consumers

This contract is intended for:

| Consumer | Usage |
|----------|--------|
| **CLI** (`vp-conformance-cli`) | Resolve paths upstream; build **`ScenarioContext`**; call **`run`**; map verdicts to exit codes |
| **CI** | Library or subprocess integration against the same `run` surface |
| **GitHub Actions** | Workflow steps that invoke the harness without depending on internal crates |
| **Future `veritypay-ci`** | Cross-repository readiness and conformance gates |
| **Documentation examples** | Demonstrate scenario → run → result without exposing internals |

Consumers **may** depend on:

- **`ScenarioContext`** and **`ConformanceResult`** field presence and semantics as documented in ADRs and core types
- Verdict vocabulary (`pass`, `fail`, `skip`, `error`)
- Stable **`run`** signature
- Comparable result shapes embedded in **`ConformanceResult`**

Consumers **must not** depend on:

- **`ImplementationAdapter`** implementations or registry layout
- **`ComparisonEngine`** algorithm or module structure
- **`ReferenceOracle`** wiring details beyond outcomes recorded in **`ConformanceResult`**
- Cargo workspace crate boundaries or internal module paths
- CLI argument structs (`clap` types)
- Report formatting internals

---

## 5. Boundaries

The public contract **excludes**:

| Excluded | Belongs elsewhere |
|----------|-------------------|
| **Filesystem paths** | CLI, **`ScenarioLoader`**, CI checkout steps |
| **Cargo workspace layout** | [ADR-0002](0002-cargo-workspace-architecture.md) — internal decomposition |
| **Adapter internals** | `vp-conformance-adapter`, runner-private wiring |
| **Comparison implementation** | Runner-internal **`ComparisonEngine`** |
| **Report formatting** | `vp-conformance-report` |
| **CLI argument types** | `vp-conformance-cli` |
| **Protocol semantics** | `veritypay-spec` only |
| **Oracle evaluation rules** | `veritypay-reference` per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) |
| **Validator diagnostics** | `veritypay-tooling` |

The public API is a **library contract** for conformance runs—not a CLI, not a serializer, not a scenario loader.

---

## Alternatives considered

### 1. Expose adapter and oracle as separate public calls

Callers orchestrate execution paths themselves.

**Rejected.** Duplicates runner responsibility; couples CI and CLI to internal stage ordering ([ADR-0003](0003-conformance-architecture.md)).

### 2. Return raw `(oracle, implementation)` tuple without `ConformanceResult`

Minimal return type without root result object.

**Rejected.** **`ConformanceResult`** already aggregates verdict, metadata, and both execution results—required for reports and CI artifacts.

### 3. Defer contract until Milestone G

Declare stability only when CI ships.

**Rejected.** CLI, examples, and early integrators need a declared surface before Milestone G wiring proliferates.

### 4. Public `ComparisonEngine::compare` entrypoint

Expose comparison for advanced callers.

**Rejected.** Comparison is runner-internal; **`ConformanceResult`** is the only comparison output in the public contract.

---

## Consequences

### Positive

- **CI-ready surface** — GitHub Actions and `veritypay-ci` can target one call shape
- **Internal flexibility** — runner architecture, comparison algorithm, and adapter registry may evolve behind `run`
- **Clear consumer boundaries** — CLI and automation share the same types
- **Reviewable stability** — contract changes require explicit ADR supersession

### Negative

- **Contract discipline** — contributors must not extend `run` casually
- **`ScenarioContext` growth** — new execution inputs must fit the input object carefully
- **Milestone overlap** — contract declared before Milestones B–F complete implementation

**Acceptable** because declaring stability now is cheaper than retrofitting CI and downstream repos after ad hoc integrations.

---

## Future

| Topic | Direction |
|-------|-----------|
| **Contract change** | Requires **successor ADR**—not drive-by signature changes |
| **Internal runner architecture** | May evolve freely behind `run` |
| **Comparison algorithm** | May evolve; **`ConformanceResult`** shape preserved |
| **Adapter registry** | May evolve; not part of public contract |
| **Parallel execution** | May batch or parallelize runs internally; single-scenario `run` contract unchanged |
| **Serialization schema** | May stabilize JSON export for CI; **`ConformanceResult`** remains the in-memory contract |
| **Additional run options** | Added via **`RunOptions`** (or equivalent)—not new public orchestration methods |

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0002](0002-cargo-workspace-architecture.md) | Internal crate boundaries behind the contract |
| [ADR-0003](0003-conformance-architecture.md) | Pipeline stages `run` orchestrates |
| [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) | Oracle sub-contract inside runner |
| [ROADMAP.md](../../ROADMAP.md) | Milestone G depends on this contract |

---

## Follow-up

- [ ] Implement `ConformanceRunner::run` when Milestones C–E land (separate PRs)
- [ ] Align CLI to construct `ScenarioContext` and call `run` only
- [ ] Align `vp-conformance-report` to read `ConformanceResult` without re-running
- [ ] Document CI integration against this contract in Milestone G

---

## Conclusion

The stable public contract of `veritypay-conformance` is:

**`ScenarioContext` → `ConformanceRunner::run` → `ConformanceResult`**

Inputs stay **path-free** and **immutable**. The entrypoint stays **singular**. Outputs stay **`ConformanceResult`** with harness verdict vocabulary. Internal runner architecture may change; **this call shape should not**—unless a future ADR supersedes this one.

**External code depends on the contract. Internal code may change freely behind it.**

This ADR records the contract only. It does **not** implement code or alter normative specification text.
