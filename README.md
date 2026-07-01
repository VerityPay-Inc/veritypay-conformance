# veritypay-conformance

**Conformance suite for the VerityPay protocol.**

This repository is part of the **Verity Specification Platform**. It runs **VP-CS (VerityPay Conformance Scenarios)** against independent implementations and compares their outcomes to the **reference interpreter**. It does **not** define protocol meaning.

**Repository maturity:** **Scaffold** — purpose, architecture, roadmap, and contribution boundaries documented; implementation language ([ADR-0001](docs/adrs/0001-implementation-language.md)) and workspace layout ([ADR-0002](docs/adrs/0002-cargo-workspace-architecture.md)) chosen; no runner code yet.

---

## Documentation

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Purpose, boundaries, and ecosystem links *(this file)* |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Long-term conformance pipeline—conceptual, not executable code |
| [ROADMAP.md](ROADMAP.md) | Capability milestones A–G with success criteria |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute to the conformance suite |
| [docs/adrs/README.md](docs/adrs/README.md) | Architecture Decision Records |
| [docs/adrs/0001-implementation-language.md](docs/adrs/0001-implementation-language.md) | ADR-0001 — Implementation language (Rust) |
| [docs/adrs/0002-cargo-workspace-architecture.md](docs/adrs/0002-cargo-workspace-architecture.md) | ADR-0002 — Cargo workspace (`vp-conformance-*`) |
| [LICENSE](LICENSE) | License terms for this repository |

---

## What is the conformance suite?

The **conformance suite** is the **executable harness** that asks whether an implementation produces the same verification outcomes as the reference interpreter under authored VP-CS scenarios.

It is the bridge between:

- **Normative scenario text** in [`veritypay-spec`](https://github.com/VerityPay-Inc/veritypay-spec)
- **Expected outcomes** from [`veritypay-reference`](https://github.com/VerityPay-Inc/veritypay-reference)
- **Actual outcomes** from an implementation under test

`veritypay-conformance` is **not**:

- a production implementation
- a reference interpreter
- an SDK or integrator library
- the owner of protocol truth
- a substitute for reading the specification

It **compares behavior**. It does **not invent** behavior.

---

## Why does it exist?

Phase I established *what VerityPay is* and how conformance is philosophically described in [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md).

Phase II requires conformance to become **runnable**—so that independent stacks can ask *"Do I match the reference oracle on VP-CS?"* and receive a reproducible answer.

Manual comparison does not scale for:

- **Implementers** validating behavior before release
- **CI pipelines** that must fail when outcomes diverge
- **Grant and audit audiences** who need evidence that implementations were tested against the same scenarios
- **Educators** demonstrating how specification scenarios map to outcomes

`veritypay-conformance` exists so that:

- VP-CS scenarios can be **loaded and executed** in a consistent harness
- The **reference interpreter** supplies expected outcomes per [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)
- **Implementation adapters** can plug in without fork-specific glue
- **Conformance reports** summarize pass, fail, and mismatch with explainable detail

See [Phase II Platform Plan](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) in `veritypay-spec`.

---

## Relationship to veritypay-spec

```
veritypay-spec          ← source of truth (VP-CS scenario meaning, normative text)
       ↓ scenarios authored here
veritypay-conformance   ← runs scenarios (this repo)
       ↓ compares against
veritypay-reference     ← reference oracle (expected outcomes)
```

| Responsibility | `veritypay-spec` | `veritypay-conformance` |
|----------------|------------------|-------------------------|
| VP-CS scenario **meaning** | Yes — authoritative prose | No — loads and runs scenarios |
| Verification outcome vocabulary | Yes — `satisfied`, `not_satisfied`, `indeterminate` | No — compares outcomes; does not define labels |
| Scenario IDs and normative fixtures | Yes | Consumes; does not author |
| Pass/fail for an implementation | No | Yes — reports comparison results |

When scenario meaning and the conformance harness disagree, **the specification wins**. The harness is updated—not the protocol.

Normative sources include [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md), [DATA_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/01-architecture/DATA_MODEL.md), and accepted VP-CS documents in `veritypay-spec`.

---

## Relationship to veritypay-tooling

[`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling) validates that the specification corpus is **internally consistent** before scenarios are executed at scale.

| Layer | Role for this repository |
|-------|--------------------------|
| **`veritypay-tooling`** | Confirms registries, documents, and references are well-formed |
| **`vp-spec-model`** | May supply typed structures for loading spec-bound scenario metadata |
| **`veritypay-conformance`** | Assumes validated upstream inputs; does not reimplement validators |

Tooling answers *"Is the spec coherent?"* The conformance suite answers *"Does this implementation match the reference oracle on VP-CS?"*

Validation rules and diagnostic policy remain in tooling. Scenario orchestration and outcome comparison remain here.

---

## Relationship to veritypay-reference

[`veritypay-reference`](https://github.com/VerityPay-Inc/veritypay-reference) is the **default reference interpreter**—the oracle for expected verification outcomes.

| Layer | Role |
|-------|------|
| **`veritypay-reference`** | Produces `VerificationResult` via `Interpreter::evaluate(&EvaluationContext)` |
| **`veritypay-conformance`** | Invokes the reference oracle; compares implementation output to that result |

The conformance suite **depends on** the reference interpreter public contract ([ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md)). It does **not** embed verification rules or redefine outcomes.

Independent implementations may conform without shipping the reference interpreter—but **this suite** uses it as the comparison baseline unless a future governance process defines an alternate oracle.

---

## System context

```
┌─────────────────────────────────────────────────────────────┐
│                     veritypay-spec                          │
│  VP-CS scenarios · CONFORMANCE_MODEL · architecture docs   │
└───────────────────────────┬─────────────────────────────────┘
                            │ validated corpus (via tooling)
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   veritypay-tooling                         │
│  vp validate · vp-spec-model                               │
└───────────────────────────┬─────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┐
         ▼                  ▼                  ▼
┌─────────────────┐ ┌───────────────┐ ┌─────────────────────┐
│veritypay-       │ │ Implementation│ │ veritypay-          │
│reference        │ │ under test    │ │ conformance         │
│(reference       │ │ (adapter)     │ │ (this repo)         │
│ oracle)         │ │               │ │                     │
└────────┬────────┘ └───────┬───────┘ └──────────┬──────────┘
         │                  │                     │
         └──────────────────┴─────────────────────┘
                            ▼
                    Conformance Report
```

---

## What this repository intentionally does NOT do

| Out of scope | Where it belongs |
|--------------|------------------|
| Normative VP-CS scenario authoring | `veritypay-spec` |
| Protocol semantics and outcome vocabulary | `veritypay-spec` |
| Registry and link validation | `veritypay-tooling` |
| Verification rule execution (oracle) | `veritypay-reference` |
| Production implementation | Product repositories |
| SDKs, wallets, payroll UI | Product or SDK repos |
| Defining new outcomes or claim types | RFC in `veritypay-spec` |
| Certifying legal or regulatory compliance | Governance outside this repo |

If a change alters **what the protocol means**, it belongs in an RFC—not in this repository.

---

## Planned capabilities

Capabilities are delivered **capability-based** per [ROADMAP.md](ROADMAP.md)—not on a fixed calendar.

| Capability | Description | Milestone |
|------------|-------------|-----------|
| Repository scaffold | Purpose, architecture, contribution rules | A |
| Load scenario fixtures | VP-CS fixture input | B |
| Adapter contract | Shared result shape; plug in implementations | C |
| Run reference oracle | Invoke `veritypay-reference` | D |
| Compare implementation output | Diff adapter vs oracle results | E |
| Produce conformance report | Human and machine-readable results | F |
| CI integration | Runnable in pipelines | G |

Long-term structure: [ARCHITECTURE.md](ARCHITECTURE.md). Workspace crates: [ADR-0002](docs/adrs/0002-cargo-workspace-architecture.md).

---

## Links to sibling repositories

| Resource | Location |
|----------|----------|
| Specification home | [veritypay-spec](https://github.com/VerityPay-Inc/veritypay-spec) |
| Conformance model | [CONFORMANCE_MODEL.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/03-development/CONFORMANCE_MODEL.md) |
| Phase II platform plan | [PHASE_II_PLATFORM_PLAN.md](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/docs/05-governance/PHASE_II_PLATFORM_PLAN.md) |
| Reference interpreter | [veritypay-reference](https://github.com/VerityPay-Inc/veritypay-reference) |
| Specification tooling | [veritypay-tooling](https://github.com/VerityPay-Inc/veritypay-tooling) |
| Public interpreter contract | [ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) |

---

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md). You are building **comparison harnesses and reports**, not inventing protocol behavior.

---

## License

See [LICENSE](LICENSE).
