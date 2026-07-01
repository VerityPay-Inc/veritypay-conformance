---
id: ADR-0002
title: Cargo Workspace Architecture
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - docs/adrs/0001-implementation-language.md
  - ARCHITECTURE.md
  - ROADMAP.md
decision_date: 2026-07-06
superseded_by: null
---

# ADR-0002 — Cargo Workspace Architecture

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-06

**Related:** [ADR-0001](0001-implementation-language.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-reference — ADR-0002](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0002-workspace-architecture.md) · [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)

---

## Purpose

Define the **Rust workspace decomposition** for `veritypay-conformance` before implementation begins.

---

## Context

[Milestone A](../../ROADMAP.md) established the conformance scaffold: purpose, architecture, roadmap, and contribution rules.

[ADR-0001](0001-implementation-language.md) chose **Rust** for native integration with **`vp-spec-model`** and **`veritypay-reference`**.

[ARCHITECTURE.md](../../ARCHITECTURE.md) defines the conformance pipeline:

```
VP-CS Scenario → Scenario Loader → Execution paths (Adapter + Oracle) → Comparison → Report
```

A single binary crate would tempt:

- Monolithic modules mixing CLI, scenario loading, adapter wiring, oracle invocation, comparison, and reporting
- Adapter code that parses VP-CS prose and invents scenario meaning
- Reports that re-run evaluation or alter outcomes
- Circular imports as CI and fixture crates arrive

This ADR records crate boundaries, dependency direction, and expansion rules. It does **not** create `Cargo.toml` files or code—that follows in Milestone B implementation PRs.

---

## Decision

**Implement `veritypay-conformance` as a Cargo workspace of focused crates**, not a single binary package.

### Initial workspace members

| Crate | Role |
|-------|------|
| **`vp-conformance-cli`** | Binary entrypoint; CLI arguments; exit codes |
| **`vp-conformance-core`** | `ScenarioContext`, contracts, shared harness types |
| **`vp-conformance-scenarios`** | Load VP-CS fixtures into immutable scenario records |
| **`vp-conformance-adapter`** | Implementation adapter interface and stub adapters |
| **`vp-conformance-runner`** | Orchestration — runs adapter and reference oracle paths |
| **`vp-conformance-report`** | Comparison and conformance report formatting |

### Composition rule

The **CLI** parses arguments and filesystem paths, then delegates to the **runner**. The **runner** loads scenarios via **`vp-conformance-scenarios`**, invokes the **adapter** and **reference oracle**, and passes **comparable results** to **report**. The **report** crate formats outcomes—it does not execute verification or define expected results.

```
CLI → Runner → Scenarios (load)
            → Adapter (implementation path)
            → Reference oracle (veritypay-reference)
            → Report (comparison + output)
```

**Orchestration lives in the runner.** The CLI owns **arguments only**.

---

## Workspace rules

| Rule | Detail |
|------|--------|
| **CLI owns arguments only** | `clap`, paths, exit codes — no scenario semantics or comparison logic |
| **Runner owns orchestration** | Load scenario, execute both paths, invoke comparison, assemble run summary |
| **Adapter never knows VP-CS semantics** | Adapters receive harness-prepared input; they do not parse scenario prose or author meaning |
| **Scenarios are immutable** | Loaded scenario records do not mutate after construction |
| **Reports format results only** | No re-execution, no outcome mutation, no protocol semantics |
| **No protocol semantics** | VP-CS meaning lives in `veritypay-spec`; expected outcomes from `veritypay-reference` |
| **No validator logic** | Registry and link validation remain in `veritypay-tooling` |
| **No dependency cycles** | Graph is acyclic; cycles require ADR supersession |
| **No `utils` or `common` crates** | Shared contracts belong in `vp-conformance-core` with documented ownership |

---

## Crate responsibilities

### `vp-conformance-cli`

| Field | Definition |
|-------|------------|
| **Purpose** | User-facing **CLI** for local development and CI invocation |
| **Responsibilities** | Parse CLI arguments; resolve paths to spec checkout and scenario selections; invoke **runner**; map conformance summary to **exit codes**; `--help` and version output |
| **Does not belong** | Scenario parsing internals; adapter implementations; oracle logic; comparison policy; report formatting internals |
| **Depends on** | `vp-conformance-core`, `vp-conformance-runner`, `vp-conformance-report` (wiring only) |

The CLI is a **thin shell**.

---

### `vp-conformance-core`

| Field | Definition |
|-------|------------|
| **Purpose** | **Stable harness contracts** between loader, runner, adapter, oracle, and report |
| **Responsibilities** | **`ScenarioContext`** and loaded scenario views; **comparable result** types shared by adapter and oracle paths; **comparison** record types; **`ImplementationAdapter`** trait signature; shared harness errors; run lifecycle identifiers |
| **Does not belong** | CLI argument structs; VP-CS file parsing; oracle invocation; report rendering; `vp-spec-model` loading; protocol semantics; validator behavior |
| **Depends on** | Minimal — prefer std only; optional thin types aligned with `veritypay-reference` model where needed without pulling full interpreter graph into every crate |

`vp-conformance-core` holds **contracts**, not orchestration execution.

---

### `vp-conformance-scenarios`

| Field | Definition |
|-------|------------|
| **Purpose** | **Scenario loader** — VP-CS fixture ingestion |
| **Responsibilities** | Load VP-CS fixtures from validated `veritypay-spec` checkout via **`vp-spec-model`** where practical; produce **immutable** scenario records bound to specification pins; emit **scenario load errors** for malformed fixtures |
| **Does not belong** | Running implementations; oracle invocation; comparison; report formatting; corpus validation (tooling); inventing normative scenario fields |
| **Depends on** | `vp-conformance-core`, `vp-spec-model` (external) |

Scenario **meaning** is authored upstream. This crate **loads** fixtures only.

---

### `vp-conformance-adapter`

| Field | Definition |
|-------|------------|
| **Purpose** | **Implementation adapter** boundary |
| **Responsibilities** | Define or re-export **`ImplementationAdapter`** contract; provide **stub/minimal adapters** for harness tests; translate implementation-specific execution into **comparable results** |
| **Does not belong** | VP-CS semantics or scenario authoring; reference oracle; comparison; report rendering; validator logic |
| **Depends on** | `vp-conformance-core` — **not** `vp-conformance-scenarios` (adapters must not parse VP-CS fixtures) |

**Adapter never knows VP-CS semantics.** The runner passes harness-prepared execution input; adapters execute and return comparable results.

---

### `vp-conformance-runner`

| Field | Definition |
|-------|------------|
| **Purpose** | **Orchestration** — execute conformance runs |
| **Responsibilities** | Coordinate scenario load → adapter execution → reference oracle execution → comparison handoff; invoke **`veritypay-reference`** per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md); map oracle output to **comparable result** shape; produce per-scenario run records for **report** |
| **Does not belong** | CLI argument parsing; human/JSON report formatting; VP-CS fixture schema invention; verification rule reimplementation; validator behavior |
| **Depends on** | `vp-conformance-core`, `vp-conformance-scenarios`, `vp-conformance-adapter`, `veritypay-reference` (oracle) — **not** `vp-conformance-cli` or `vp-conformance-report` internals |

The runner **orchestrates**; it does not **define** protocol truth.

---

### `vp-conformance-report`

| Field | Definition |
|-------|------------|
| **Purpose** | **Conformance report** formatting |
| **Responsibilities** | Render comparison summaries for humans; structured export for CI; present per-scenario pass/fail with oracle vs implementation detail |
| **Does not belong** | Running adapter or oracle; altering comparison verdicts; scenario loading; protocol semantics |
| **Depends on** | `vp-conformance-core` — **not** `vp-conformance-runner` internals |

Reporting is **downstream** of orchestration.

---

## Dependency graph

```
                 ┌──────────────────────┐
                 │  vp-conformance-cli  │  (binary; arguments + exit codes)
                 └──────────┬───────────┘
                            │
              ┌─────────────┴─────────────┐
              ▼                           ▼
   ┌────────────────────┐      ┌─────────────────────┐
   │ vp-conformance-    │      │ vp-conformance-     │
   │ runner             │      │ report              │
   │ (orchestration)    │      │ (format only)       │
   └─────────┬──────────┘      └──────────┬──────────┘
             │                            │
    ┌────────┼────────────┐              │
    ▼        ▼            ▼              │
┌─────────┐ ┌──────────┐ ┌──────────────┐ │
│vp-conf- │ │vp-conf-  │ │ veritypay-   │ │
│scenarios│ │adapter   │ │ reference    │ │
│(load)   │ │(impl)    │ │ (oracle)     │ │
└────┬────┘ └────┬─────┘ └──────────────┘ │
     │           │                         │
     └─────┬─────┘                         │
           ▼                               ▼
  ┌────────────────────┐         ┌────────────────────┐
  │ vp-conformance-    │         │  vp-spec-model     │  (external)
  │ core               │         │  (veritypay-       │
  └────────────────────┘         │   tooling)         │
                                 └────────────────────┘
```

**Allowed dependency direction (summary):**

| Crate | May depend on |
|-------|----------------|
| `vp-conformance-cli` | `core`, `runner`, `report` |
| `vp-conformance-runner` | `core`, `scenarios`, `adapter`, `veritypay-reference` |
| `vp-conformance-scenarios` | `core`, `vp-spec-model` |
| `vp-conformance-adapter` | `core` |
| `vp-conformance-report` | `core` |
| `vp-conformance-core` | Minimal / std (leaf contract crate) |

**Forbidden:** `adapter` → `scenarios`; `report` → `runner`; any cycle; adapter or report → `vp-spec-model` directly unless superseded by ADR.

---

## Future workspace members

Deferred until milestones justify them:

| Crate | Purpose |
|-------|---------|
| **`vp-conformance-fixtures`** | Shared golden fixtures and test vectors for harness development |
| **`vp-conformance-ci`** | CI-specific wiring, readiness gates, workflow helpers |

Adding either crate requires the same acyclic rules and an ADR update if boundaries change materially.

---

## Alternatives considered

### 1. Single `vp-conformance` binary crate

One package with modules for CLI, loader, runner, and report.

**Rejected.** Invites the orchestration/report/adapter coupling this architecture deliberately separates.

### 2. Merge scenarios and runner

Load and execute in one crate.

**Rejected.** Scenario immutability and loader testing benefit from a dedicated crate; runner stays focused on orchestration.

### 3. Adapter trait in scenarios crate

Adapters parse VP-CS directly.

**Rejected.** Violates **adapter never knows VP-CS semantics** — adapters must remain ignorant of scenario prose and fixture layout.

### 4. Report crate invokes runner

Formatting triggers re-execution.

**Rejected.** Reports format frozen comparison records only.

---

## Consequences

### Positive

- **Clear ownership** — CLI, load, execute, compare, and report are separable in review and milestones
- **Adapter isolation** — implementations plug in without parsing VP-CS
- **Oracle fidelity** — runner owns `veritypay-reference` invocation in one place
- **Milestone alignment** — B scenarios, C adapter, D oracle, E comparison, F report map to crates
- **CI-ready shape** — future `vp-conformance-ci` attaches without refactoring core

### Negative

- **More crates to bootstrap** before first green CI
- **Contract discipline** — `vp-conformance-core` must stay small and stable
- **Cross-repo dependencies** — runner couples to `veritypay-reference` and `vp-spec-model` versioning

**Acceptable** because boundaries are cheaper to establish before Milestone B code than to extract after orchestration entangles.

---

## Future reconsideration

Revisit this ADR only if:

- Workspace member count creates measurable maintainer burden **without** isolation benefit
- A proposed crate violates acyclic rules and cannot be resolved by trait extraction
- Oracle or scenario loading requires a layout change that cannot be accommodated within these crates
- An accepted ADR supersedes this layout

Splitting or merging crates requires **a successor ADR**—not drive-by refactors.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ADR-0001](0001-implementation-language.md) | Rust workspace recommendation |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Pipeline this ADR implements |
| [ROADMAP.md](../../ROADMAP.md) | Milestones B–G map to these crates |
| [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) | Oracle public contract |

---

## Follow-up

- [ ] Add workspace `Cargo.toml` and crate manifests (Milestone B — separate PR)
- [ ] Document workspace layout in README when directories exist
- [ ] Declare `vp-spec-model` and `veritypay-reference` dependency paths in workspace manifest

---

## Conclusion

`veritypay-conformance` decomposes into **CLI**, **core contracts**, **scenario loading**, **adapters**, **runner orchestration**, and **reporting**—with **no protocol semantics**, **no validator logic**, and **no dependency cycles**. The runner executes; the adapter translates; scenarios load immutably; reports format honestly.

This ADR records workspace architecture only. It does **not** implement code or alter normative specification text.
