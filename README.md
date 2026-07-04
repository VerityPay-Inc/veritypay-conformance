# veritypay-conformance

**Conformance suite for the VerityPay protocol.**

This repository is part of the **Verity Specification Platform**. It runs **VP-CS (VerityPay Conformance Scenarios)** against independent implementations and compares their outcomes to the **reference interpreter**. It does **not** define protocol meaning.

**Repository maturity:** **Conformance Platform Ready** — scenario load, reference oracle, adapter contract, runner, comparison, human and JSON reports, and `vp-conformance run` per [ROADMAP.md](ROADMAP.md); **Platform 1.3** **`normalized_text`** semantic coverage via **VP-CS-0011**–**0013**; org-wide CI automation remains incremental.

| Capability | Status |
|------------|--------|
| Scenario loading | ✓ |
| Reference oracle | ✓ |
| Adapter contract | ✓ |
| Runner | ✓ |
| Comparison | ✓ |
| Reports | ✓ |
| CLI | ✓ |
| Platform 1.3 **`normalized_text`** scenarios | ✓ (VP-CS-0011–0013) |

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
| [docs/adrs/0003-conformance-architecture.md](docs/adrs/0003-conformance-architecture.md) | ADR-0003 — Conformance pipeline |
| [docs/adrs/0004-conformance-public-contract.md](docs/adrs/0004-conformance-public-contract.md) | ADR-0004 — Public contract (`run` → `ConformanceResult`) |
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

## Repository layout

```
veritypay-conformance/
├── Cargo.toml                 ← Workspace manifest
├── Cargo.lock
├── rust-toolchain.toml        ← Pinned stable Rust
├── rustfmt.toml
├── README.md                  ← You are here
├── ARCHITECTURE.md
├── ROADMAP.md
├── CONTRIBUTING.md
├── LICENSE
├── .github/workflows/ci.yml   ← fmt, clippy, test
├── scripts/
│   └── readiness-gate.sh      ← local fmt, clippy, test, CLI smoke
├── docs/
│   └── adrs/
├── crates/
│   ├── vp-conformance-cli/      ← `vp-conformance` binary
│   ├── vp-conformance-core/     ← ScenarioContext, contracts
│   ├── vp-conformance-scenarios/← VP-CS fixture loading (Milestone B)
│   ├── vp-conformance-adapter/  ← implementation adapter boundary (Milestone C)
│   ├── vp-conformance-runner/   ← orchestration (Milestones D–E)
│   └── vp-conformance-report/   ← conformance reports (Milestone F)
├── src/lib.rs                 ← workspace root (integration tests)
└── tests/                     ← workspace integration tests
```

Build and run:

```bash
cargo build -p vp-conformance-cli
cargo run -p vp-conformance-cli --bin vp-conformance -- run \
  --scenario ../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml \
  --adapter stub \
  --adapter-outcome satisfied \
  --format human
```

### Scenario fixtures

The preferred source of VP-CS scenarios is the specification repository:

```
veritypay-spec/spec/conformance/scenarios/
```

Point `--scenario` at any TOML file in that directory (for example `VP-CS-0001.toml` through `VP-CS-0013.toml`). The harness loads spec-published field names through the same `ScenarioLoader` used for local fixtures.

**Platform release:** **[Platform 1.2](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/PLATFORM_RELEASES.md)** remains the current platform release in `veritypay-spec`. **Platform 1.3** is **in progress** — draft **`normalized_text`** scenarios are executable when sibling `veritypay-reference` implements **VP-RULE-0011**.

**Supported assertion types** (via reference oracle dispatch): **`body_equality`**, **`minimal`** (alias), **`normalized_text`**. The reference interpreter registers **2** assertion evaluators (`BodyEqualityEvaluator`, `NormalizedTextEvaluator`); **`minimal`** dispatches to `BodyEqualityEvaluator`.

**Spec-published VP-CS count:** **5** fixtures (**VP-CS-0001**, **VP-CS-0002**, **VP-CS-0011**–**0013**).

| Scenario | Rule | Assertion type | Expected oracle outcome |
|----------|------|----------------|-------------------------|
| **VP-CS-0001** | VP-RULE-0001 | `minimal` | `satisfied` |
| **VP-CS-0002** | VP-RULE-0002 | `minimal` | `indeterminate` |
| **VP-CS-0011** | VP-RULE-0011 | `normalized_text` | `satisfied` |
| **VP-CS-0012** | VP-RULE-0011 | `normalized_text` | `not_satisfied` |
| **VP-CS-0013** | VP-RULE-0011 | `normalized_text` | `indeterminate` |

Local fixtures under `crates/vp-conformance-scenarios/tests/fixtures/` remain for isolated unit tests and harness development—they are not normative scenario ownership.

Human output (matching stub outcome):

```
Conformance Report

Summary
-------
Total: 1
Passed: 1
...
✓ VP-CS-0001
```

JSON output:

```bash
cargo run -p vp-conformance-cli --bin vp-conformance -- run \
  --scenario crates/vp-conformance-scenarios/tests/fixtures/minimal.toml \
  --format json
```

Exit codes: `0` pass, `1` conformance failure, `2` user/CLI error, `3` harness error.

### Readiness gate

Before merge or local integration with sibling repositories, run the readiness gate from the repository root:

```bash
./scripts/readiness-gate.sh
```

The script runs `cargo fmt --check`, `cargo clippy`, `cargo test`, a CLI boot check, and smoke conformance runs against `../veritypay-spec/spec/conformance/scenarios/` fixtures **VP-CS-0001**, **VP-CS-0002**, and **VP-CS-0011**–**0013** when the sibling specification checkout is present. It mirrors the readiness process used in [`veritypay-reference`](https://github.com/VerityPay-Inc/veritypay-reference) and [`veritypay-tooling`](https://github.com/VerityPay-Inc/veritypay-tooling). Any failing step exits non-zero.

## Repository readiness criteria

Downstream repositories may depend on `veritypay-conformance` when all of the following hold:

| Criterion | Status |
|-----------|--------|
| Cargo workspace builds; `cargo test --workspace` passes | ✓ |
| [`scripts/readiness-gate.sh`](scripts/readiness-gate.sh) runs fmt, clippy, test, and smoke conformance | ✓ |
| `vp-conformance run` wires load → oracle → adapter → compare → report | ✓ |
| Human and JSON report renderers produce stable output | ✓ |
| Public contract declared in [ADR-0004](docs/adrs/0004-conformance-public-contract.md) | ✓ |
| Reference oracle baseline documented via [veritypay-reference ADR-0007](https://github.com/VerityPay-Inc/veritypay-reference/blob/main/docs/adrs/0007-reference-interpreter-public-contract.md) | ✓ |
| Spec-published VP-CS smoke (**VP-CS-0001**, **VP-CS-0002**, **VP-CS-0011**–**0013**) passes with matching stub adapter | ✓ |

**Deferred:** multi-scenario suite discovery, external implementation adapters, org-wide reusable CI workflows, and VP-CS registry-backed catalog discovery.

Development checks (from repository root):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

CI runs the same `fmt`, `clippy`, and `test` commands on pull requests and pushes to `main`.

**Public contract** ([ADR-0004](docs/adrs/0004-conformance-public-contract.md)): external callers depend on `ScenarioContext` → `ConformanceRunner::run` → `ConformanceResult` only—not adapter, comparison, or report internals.

---

## Capabilities

Capabilities are delivered **capability-based** per [ROADMAP.md](ROADMAP.md)—not on a fixed calendar.

| Capability | Description | Milestone |
|------------|-------------|-----------|
| Repository scaffold | Purpose, architecture, contribution rules | A ✓ |
| Scenario loading | VP-CS fixture input via `ScenarioLoader` (local + spec-published paths) | B ✓ / G.2 ✓ |
| Adapter contract | Shared result shape; plug in implementations | C ✓ |
| Reference oracle | Invoke `veritypay-reference` | D ✓ |
| Runner | Orchestrate oracle and adapter paths | D ✓ |
| Comparison | Diff adapter vs oracle results | E ✓ |
| Reports | Human and machine-readable summaries | F ✓ |
| CLI (`vp-conformance run`) | Single-scenario conformance from the shell | G ✓ |
| Readiness gate | Local fmt, clippy, test, spec scenario smoke | G ✓ |

**Deferred:** VP-CS registry-backed catalog discovery, multi-scenario suite runs, external adapters, org-wide CI workflows.

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
| Public conformance contract | [ADR-0004](docs/adrs/0004-conformance-public-contract.md) |

---

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md). You are building **comparison harnesses and reports**, not inventing protocol behavior.

---

## License

See [LICENSE](LICENSE).
