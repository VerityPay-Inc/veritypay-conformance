# Contributing to veritypay-conformance

**Handbook for contributors building the VerityPay conformance suite.**

You are not defining the VerityPay protocol here. You are building **comparison harnesses**—runners that load VP-CS scenarios, invoke the reference oracle, execute implementation adapters, and report whether outcomes match.

Read this before opening a pull request.

---

## Specification boundary

These statements govern every contribution to this repository:

1. **Contributors do not define protocol behavior here.** VP-CS scenario **meaning** belongs in [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec). When harness behavior and scenario prose disagree on meaning, the specification wins.

2. **Expected outcomes come from `veritypay-reference`.** The reference interpreter is the default oracle per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md). Do not hard-code expected outcomes that bypass the oracle unless an accepted governance process defines an alternate baseline.

3. **Conformance compares behavior; it does not invent behavior.** This repository detects whether an implementation matches the reference oracle on authored scenarios. It does not assign new verification outcomes, claim types, or scenario semantics.

4. **Protocol changes belong in `veritypay-spec` through RFCs.** Normative changes flow through [VP-RFC-0000](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0000-rfc-process.md). Implement resulting harness behavior here only after scenarios and outcomes are accepted upstream.

---

## Welcome

Contributing to `veritypay-conformance` means strengthening **reproducible conformance checking** for the VerityPay ecosystem.

This repository exists because:

- **Implementers** need a standard way to test against VP-CS
- **CI** needs pass/fail signals grounded in the reference oracle
- **Auditors** need evidence that comparisons use the same scenarios and expected outcomes
- **`veritypay-reference`** needs a consumer for its public evaluation contract

We welcome engineers, conformance authors, and specification readers. You do not need permission to read, propose issues, or draft ADRs. You **do** need to respect the boundary: **the suite compares behavior defined elsewhere; it never defines the protocol.**

---

## Before you start

| Order | Document | Why |
|-------|----------|-----|
| 1 | [README.md](README.md) | Purpose, boundaries, maturity |
| 2 | [ARCHITECTURE.md](ARCHITECTURE.md) | Conformance pipeline and components |
| 3 | [ROADMAP.md](ROADMAP.md) | Current milestone and success criteria |
| 4 | [veritypay-spec — CONFORMANCE_MODEL](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) | Verification outcomes and VP-CS philosophy |
| 5 | [veritypay-reference — ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) | Oracle public contract |
| 6 | [veritypay-spec — Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) | Where conformance sits in the platform |

For protocol context, read [veritypay-spec — CONTRIBUTING](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/CONTRIBUTING.md) when your work touches semantics defined there.

---

## The golden rule

> **VP-CS scenario meaning belongs in `veritypay-spec`.**  
> **Expected outcomes come from `veritypay-reference`.**  
> **This repository compares behavior—it does not invent behavior.**

| If you want to… | Do this |
|-----------------|---------|
| Change what a VP-CS scenario **means** | Author or amend scenario in `veritypay-spec` |
| Change expected verification outcome for a scenario | Update spec scenario + ensure reference interpreter matches |
| Add harness loading, comparison, or reporting | Pull request in **this** repository |
| Validate registry YAML or document links | Pull request in **`veritypay-tooling`** |
| Implement verification rules | Pull request in **`veritypay-reference`** |
| Define new outcome labels | RFC in `veritypay-spec` |

When unsure, **default to spec governance first.**

---

## What belongs in this repository

| In scope | Examples |
|----------|----------|
| Scenario loading | VP-CS fixture ingestion from validated spec tree |
| Reference oracle wiring | Invoke `Interpreter::evaluate` |
| Adapter contract | Plug in implementations under test |
| Outcome comparison | Oracle vs implementation diff |
| Conformance reports | Human and CI-oriented summaries |
| ADRs | Runner layout, adapter API, report format |
| Tests | Fixture runs with known pass/fail |

| Out of scope | Examples |
|--------------|----------|
| Normative VP-CS authoring | Scenario prose and meaning |
| Verification rule implementation | Oracle logic (belongs in reference) |
| Corpus validation | Registry validators |
| Production implementation code | Wallets, payroll, merchant stacks |
| SDKs and product APIs | Integrator libraries |
| Certifying legal compliance | Governance outside this repo |

---

## Implementation values

When code milestones begin, prefer:

| Value | Practice |
|-------|----------|
| **Honest comparison** | Fail loudly on outcome mismatch |
| **Oracle fidelity** | Use `veritypay-reference`; do not shadow the interpreter |
| **Adapter isolation** | Keep implementation-specific code in adapters |
| **Reproducibility** | Same fixtures → same comparison results |
| **Minimal scope** | Smallest milestone that proves one scenario end-to-end |

Avoid:

- Hard-coding expected outcomes that contradict the reference oracle
- Authoring scenario meaning in harness code or comments
- Smuggling normative protocol changes through comparison policy
- Treating pass/fail as product certification

---

## Pull request expectations

Before requesting review:

1. Confirm the change aligns with the **current ROADMAP milestone**
2. Confirm no normative spec or scenario change is smuggled in without upstream acceptance
3. Add or update fixtures demonstrating expected comparison behavior
4. Cite VP-CS IDs, spec sections, or reference ADRs where helpful
5. If the change affects public adapter or report contracts, propose or reference an ADR

---

## Related repositories

| Repository | Relationship |
|------------|--------------|
| [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec) | Source of truth — VP-CS scenario meaning |
| [`veritypay-reference`](https://github.com/VerityPay-Inc/veritypay-reference) | Reference oracle — expected outcomes |
| [`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling) | Validates corpus; may supply typed spec structures |

---

*Load scenarios from spec. Oracle from reference. Compare honestly. Report clearly.*
