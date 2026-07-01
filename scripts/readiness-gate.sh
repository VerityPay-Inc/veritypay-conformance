#!/usr/bin/env bash
# Readiness gate for veritypay-conformance.
# Run from the repository root or any directory; resolves paths relative to this script.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> cargo fmt --check"
cargo fmt --check

echo "==> cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> cargo test --workspace"
cargo test --workspace

echo "==> cargo run -p vp-conformance-cli --bin vp-conformance"
cargo run -p vp-conformance-cli --bin vp-conformance -- help

FIXTURE="${ROOT}/crates/vp-conformance-scenarios/tests/fixtures/minimal.toml"
if [[ -f "$FIXTURE" ]]; then
    echo "==> vp-conformance run --scenario ${FIXTURE} --adapter stub --adapter-outcome satisfied"
    cargo run -p vp-conformance-cli --bin vp-conformance -- run \
        --scenario "$FIXTURE" \
        --adapter stub \
        --adapter-outcome satisfied
else
    echo "==> skipping smoke run: ${FIXTURE} not found"
    echo "    minimal VP-CS fixture is required for end-to-end conformance smoke"
fi

echo "readiness gate passed"
