# Architecture

**Long-term architecture for the VerityPay conformance suite.**

This document describes **components, responsibilities, and data flow**. It does not specify implementation language, module layout, or runtime topology. Those decisions follow milestones in [ROADMAP.md](ROADMAP.md) and ADRs in this repository when code begins.

**Audience:** maintainers, contributors, implementers, auditors, and grant reviewers who need to understand what the conformance suite will become—not how it is coded today.

**Upstream dependency:** [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec) defines VP-CS scenario meaning, verification outcomes, and conformance philosophy. The suite **runs and compares**—it does not invent normative requirements.

**Oracle dependency:** [`veritypay-reference`](https://github.com/VerityPay-Inc/veritypay-reference) supplies expected outcomes via the stable public contract ([ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)).

---

## Design stance

| Principle | Meaning |
|-----------|---------|
| **Specification upstream** | Scenario meaning derives from accepted VP-CS documents in `veritypay-spec` |
| **Oracle from reference** | Expected outcomes come from `veritypay-reference`, not from this repository |
| **Compare, don't define** | The suite detects divergence; it does not assign new protocol truth |
| **Adapter-pluggable** | Implementations connect through a documented adapter contract |
| **Reproducible reports** | Same scenario + same inputs → comparable conformance results |
| **Not a gatekeeper** | Passing VP-CS here is evidence of alignment—not a product certification |

---

## Conformance pipeline

Conceptual flow aligned with [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md):

```
VP-CS Scenario
        ↓
Scenario Loader
        ↓
   Execution paths (shared comparable result shape)
   ├── Implementation Adapter
   └── Reference Oracle
        ↓
Comparison
        ↓
Conformance Report
```

**Comparison is separate from execution.** The loader prepares scenario input; **adapter** and **oracle** are sibling execution paths that each produce a comparable result; **comparison** diff those results. Neither path defines protocol meaning or expected outcomes by itself.

Each stage below is a **component boundary**. Exact APIs and file layout are deferred until implementation milestones.

---

## Major components

### VP-CS Scenario

**Purpose:** The normative conformance unit authored in `veritypay-spec`.

**Responsibilities:**

- Identify scenario by stable VP-CS ID
- Bind claim, evidence, and specification context for the run
- Document intended verification question in prose upstream

**Boundaries:**

- Scenario **meaning** is authored in `veritypay-spec`—not in this repository
- The suite loads scenario **fixtures**; it does not rewrite normative text

**Inputs:** VP-CS documents and machine-readable fixtures from validated spec tree.

**Outputs:** Structured scenario record for loader and adapters.

---

### Scenario Loader

**Purpose:** Load VP-CS scenario fixtures into a normalized internal representation.

**Responsibilities:**

- Resolve scenario paths or IDs against a validated `veritypay-spec` checkout
- Parse fixture metadata (specification binding, claim, evidence references)
- Surface load errors before any implementation or oracle is invoked
- Honor Edition or protocol version pins declared by the scenario

**Boundaries:**

- Does **not** validate registry YAML (that is `veritypay-tooling`)
- Does **not** execute verification rules
- Does **not** invent scenario fields beyond accepted fixture formats

**Inputs:** VP-CS fixture files, spec checkout identity.

**Outputs:** Loaded scenario ready for adapter and oracle wiring.

---

### Implementation Adapter

**Purpose:** Bridge between the conformance harness and an **implementation under test**.

**Responsibilities:**

- Accept loaded scenario input per the documented adapter contract (Milestone C)
- Invoke the implementation's verification path (library, CLI, or subprocess—TBD)
- Return a **comparable result** in the shared execution result shape (outcome, binding, optional trace)
- Isolate implementation-specific failures from harness logic

**Boundaries:**

- Adapters **translate**; they do not define expected outcomes
- The reference oracle is a **separate execution path**, not a downstream step after the adapter
- No payment-product business logic in the harness core

**Inputs:** Loaded scenario, adapter configuration.

**Outputs:** Comparable implementation result for comparison.

---

### Reference Oracle

**Purpose:** Produce **expected** verification outcomes using the reference interpreter.

**Responsibilities:**

- Build `EvaluationContext` from loaded scenario fixtures
- Invoke `veritypay-reference` per public contract: `Interpreter::evaluate(&EvaluationContext) -> VerificationResult`
- Map oracle output to the **same comparable result shape** as implementation adapters
- Bind oracle runs to the same specification pin as the scenario

**Boundaries:**

- Does **not** reimplement verification rules locally
- Does **not** modify `VerificationResult` after evaluation
- Does **not** substitute ad hoc expected outcomes when the oracle disagrees with contributor intuition
- Runs **in parallel** with the adapter path—not after it

**Inputs:** Loaded scenario, reference interpreter dependency.

**Outputs:** Comparable oracle result for comparison.

---

### Comparison

**Purpose:** Determine whether the **implementation result** matches the **oracle result**.

**Responsibilities:**

- Receive two comparable results from sibling execution paths (adapter and oracle)
- Compare normative **outcome** (`satisfied`, `not_satisfied`, `indeterminate`)
- Compare evaluated claim identity and specification binding where scenarios require
- Optionally compare trace shape or reason strings when milestones support it
- Classify mismatch types (outcome divergence, missing evidence, binding drift)

**Boundaries:**

- Comparison policy must not introduce new outcome labels
- Mismatch explanation is **reporting**—not a new verification verdict
- Does **not** silently coerce implementation results to match the oracle

**Inputs:** Implementation comparable result, oracle comparable result.

**Outputs:** Per-scenario comparison verdict (pass, fail, skip, error).

---

### Conformance Report

**Purpose:** Summarize suite results for humans and CI.

**Responsibilities:**

- Aggregate per-scenario comparison results
- Emit human-readable summary for local development
- Emit structured export for CI (format TBD—JSON or similar)
- Map results to process exit codes suitable for pipelines
- Preserve enough detail for auditors to reproduce failures

**Boundaries:**

- Reports **describe** comparison; they do not re-run verification
- Does **not** certify regulatory compliance
- Does **not** alter outcomes from implementation or oracle

**Inputs:** Comparison results for all scenarios in a run.

**Outputs:** Conformance report artifact and exit status.

---

## System context

```
┌─────────────────────────────────────────────────────────────┐
│                     veritypay-spec                          │
│  VP-CS · CONFORMANCE_MODEL · architecture docs             │
└───────────────────────────┬─────────────────────────────────┘
                            │ validated corpus
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   veritypay-tooling                         │
│  vp validate · vp-spec-model                               │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  veritypay-conformance                        │
│  loader · adapter · oracle · comparison · report           │
└───────────────┬─────────────────────────┬───────────────────┘
                │                         │
                ▼                         ▼
┌───────────────────────┐     ┌───────────────────────────────┐
│ veritypay-reference   │     │ Implementation under test      │
│ (reference oracle)    │     │ (via adapter)                  │
└───────────────────────┘     └───────────────────────────────┘
```

---

## Data ownership

| Concept | Owner |
|---------|-------|
| VP-CS scenario meaning | `veritypay-spec` |
| Expected `VerificationResult` | `veritypay-reference` |
| Implementation actual result | Implementation under test (via adapter) |
| Pass/fail comparison | `veritypay-conformance` |
| Outcome vocabulary | `veritypay-spec` (VP-TERM-011) |

---

## Non-goals

This architecture does **not** include:

- Plugin systems with runtime rule loading
- Blockchain or network adapters in the harness core
- Normative scenario authoring
- Replacing independent implementations
- Legal or regulatory certification workflows

---

## Evolution

Component APIs, crate boundaries, and serialization formats will be recorded in ADRs as milestones complete. This document remains the **conceptual map**—not the implementation spec.

When implementation begins, prefer:

- Thin orchestration in the runner
- Explicit adapter and oracle boundaries
- Fixtures aligned with VP-CS IDs from `veritypay-spec`
- Comparison logic that fails loudly on outcome mismatch

---

*Load scenarios from spec. Oracle from reference. Compare behavior. Report honestly.*
