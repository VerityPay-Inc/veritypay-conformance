---
id: ADR-0001
title: Implementation Language for veritypay-conformance
status: accepted
version: 1.0.0
authors:
  - VerityPay Core Team
reviewers: []
related_docs:
  - README.md
  - ARCHITECTURE.md
  - ROADMAP.md
  - CONTRIBUTING.md
decision_date: 2026-07-06
superseded_by: null
---

# ADR-0001 — Implementation Language for veritypay-conformance

**Status:** Accepted · **Version:** 1.0.0 · **Date:** 2026-07-06

**Related:** [README.md](../../README.md) · [ARCHITECTURE.md](../../ARCHITECTURE.md) · [ROADMAP.md](../../ROADMAP.md) · [veritypay-reference — ADR-0001](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0001-reference-implementation-language.md) · [veritypay-tooling — ADR-0001](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0001-tooling-implementation-language.md) · [veritypay-spec — ADR Guide](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/ADR_GUIDE.md)

---

## Purpose

Choose the **implementation language** for the VerityPay conformance suite before any runtime code exists.

---

## Context

[Milestone A](../../ROADMAP.md) established `veritypay-conformance` as a **documentation scaffold**: purpose, architecture, roadmap, and contribution boundaries.

[Milestone B](../../ROADMAP.md) will load VP-CS scenario fixtures. Later milestones wire **implementation adapters**, invoke **`veritypay-reference`** as the oracle, **compare** results, and emit **conformance reports**.

Before implementation begins, the project must choose an implementation language. This is an **engineering decision**—not a protocol decision.

> **This ADR does not define protocol behavior.**  
> **It does not define VP-CS semantics.**  
> Those remain in [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec).

`veritypay-conformance` must:

- Load scenario fixtures from validated specification input
- Invoke the reference interpreter as oracle per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)
- Compare implementation and oracle outcomes reproducibly
- Produce reports suitable for local development and CI

The language must model scenarios, adapters, comparable results, and reports **explicitly**—without becoming a second specification.

---

## Decision

**`veritypay-conformance` will be implemented in Rust.**

The primary reason is **not** raw performance. The conformance suite is not a throughput benchmark; **honest comparison** and **platform consistency** matter more than microseconds.

The primary reasons are:

- **Same ecosystem as `veritypay-tooling` and `veritypay-reference`** — shared engineering conventions, CI patterns, and contributor context
- **Native consumption of `vp-spec-model`** — load specification-bound scenario metadata without duplicate types
- **Native consumption of `veritypay-reference` as the oracle** — direct library use of `Interpreter::evaluate` and `VerificationResult`
- **Strong typing** for VP-CS scenarios, adapters, comparable results, and conformance reports
- **Excellent testing and CI support** — `cargo test`, reproducible builds, pinned toolchains
- **Platform consistency over language diversity** — one toolchain across spec platform infrastructure repos

---

## Alternatives considered

Each option was evaluated for **conformance harness fit**, not general language popularity.

### Rust

**Advantages**

- Direct `vp-spec-model` and `veritypay-reference` dependencies (path, git, or workspace)
- Strong static types for scenario records, adapter contracts, and comparison results
- Mature CLI, test, and fixture tooling for Milestone G CI integration
- Same language as tooling and reference—minimal context switching for platform contributors

**Tradeoffs**

- Steeper learning curve for contributors unfamiliar with ownership and lifetimes
- Workspace bootstrap takes longer than a single-script harness

**Assessment:** Best match for typed integration, oracle fidelity, and long-lived conformance infrastructure.

---

### Go

**Advantages**

- Simple language with fast compilation
- Approachable for many backend engineers
- Good standard library for CLI and subprocess adapters
- Single-binary deployment story

**Tradeoffs**

- No native `vp-spec-model` or `veritypay-reference` consumption—FFI, regeneration, or duplicate types
- Weaker compile-time modeling for scenario and report domains as surface area grows
- Second toolchain in a Rust-first specification platform

**Assessment:** Viable for a thin CLI wrapper; weaker fit for native oracle integration and shared specification types.

---

### TypeScript

**Advantages**

- Very large contributor ecosystem
- Rapid iteration for early prototyping
- Familiar to many application developers

**Tradeoffs**

- Requires Node.js (or bundled runtime) at execution time
- No native `vp-spec-model` or `veritypay-reference` bindings—duplicate types or fragile subprocess glue
- Weaker story for a pinned conformance runner in CI
- Runtime type drift unless carefully disciplined

**Assessment:** Strong for examples and docs tooling; weaker as the canonical conformance harness runtime.

---

### Python

**Advantages**

- Enormous ecosystem and parsing libraries
- Rapid development for experiments and one-off analysis

**Tradeoffs**

- Packaging and dependency friction for reproducible CI
- No native `vp-spec-model` or `veritypay-reference` consumption
- Harder to ship one portable, pinned conformance artifact
- Interpreter version drift across contributors

**Assessment:** Excellent for ad hoc analysis; poor fit as the long-lived conformance suite core.

---

## Rationale

The conformance suite exists so implementations can be **compared honestly** to the reference oracle on authored VP-CS scenarios—not to maximize language diversity across the platform.

Rust provides:

1. **Direct reuse of `vp-spec-model`** — scenario loading aligned with validated specification representation
2. **Direct reuse of `veritypay-reference`** — oracle path without reimplementing verification rules
3. **Explicit harness modeling** — scenarios, adapters, comparable results, and reports as types reviewers can follow
4. **Platform coherence** — same toolchain as `veritypay-tooling` and `veritypay-reference`, reducing institutional overhead

Go, TypeScript, and Python remain reasonable for **adjacent** work (product SDKs, documentation sites, one-off scripts). They are not rejected globally—only for **this** repository's core mission.

---

## Consequences

### Positive

- **Shared specification types** — scenario loading uses the same model layer validators already use
- **Faithful oracle integration** — conformance invokes the reference interpreter, not a reimplementation
- **Explicit comparison surface** — strong types for pass/fail records and report aggregation
- **Reliable CI** — reproducible builds for conformance gates (Milestone G)
- **Platform alignment** — contributors move between tooling, reference, and conformance in one ecosystem

### Negative

- **Rust learning curve** — some contributors need onboarding before first merged harness PR
- **Coupling to Rust platform crates** — non-Rust implementations under test integrate via adapters/subprocess, not embedded model types (acceptable)
- **Slower initial velocity** — workspace bootstrap before first scenario load milestone

**Why these tradeoffs are acceptable**

The conformance suite is **institutional infrastructure**—a comparison harness auditors and implementers must trust. Slower start with explicit types and native oracle integration beats fast iteration that drifts from the reference interpreter.

---

## Future reconsideration

This decision should **only** be revisited if:

- Rust becomes a **blocker to ecosystem sustainability** (e.g. no maintainers able to steward the codebase over a documented period)
- **Native integration** with `vp-spec-model` or `veritypay-reference` cannot reasonably be achieved in Rust without disproportionate cost
- A **future ADR supersedes** this one with new evidence

Changing the implementation language **requires a new ADR**. Partial rewrites without ADR are not acceptable for the core harness.

---

## Related decisions

| Document | Relationship |
|----------|--------------|
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | Conformance pipeline; language chosen by this ADR |
| [ROADMAP.md](../../ROADMAP.md) | Milestone B proceeds after workspace bootstrap |
| [CONTRIBUTING.md](../../CONTRIBUTING.md) | Compare behavior; do not invent protocol semantics |
| [veritypay-reference — ADR-0001](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0001-reference-implementation-language.md) | Reference interpreter language (Rust) |
| [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) | Oracle public contract |
| [veritypay-tooling — ADR-0001](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0001-tooling-implementation-language.md) | Tooling language (same ecosystem) |
| [veritypay-tooling — ADR-0007](https://github.com/VerityPay-Inc/veritypay-tooling/blob/main/docs/adrs/0007-specification-model-stability.md) | Stable `vp-spec-model` for consumers |

---

## Follow-up

- [ ] Bootstrap `cargo` workspace (Milestone B implementation—separate PR)
- [ ] Add `vp-spec-model` and `veritypay-reference` dependency paths when workspace exists
- [ ] Document local build and readiness gate in README when code lands

---

## Conclusion

The conformance suite values **platform consistency, typed harness modeling, and faithful oracle integration** over language diversity. **Rust** best supports native consumption of **`vp-spec-model`** and **`veritypay-reference`**, reproducible comparison, and long-term maintainability as VP-CS coverage grows.

This ADR records an engineering decision only. It does **not** define protocol behavior or VP-CS semantics.
