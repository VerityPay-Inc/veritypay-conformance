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

SPEC_FIXTURE="${ROOT}/../veritypay-spec/spec/conformance/scenarios/VP-CS-0001.toml"
if [[ -f "$SPEC_FIXTURE" ]]; then
    echo "==> vp-conformance run --scenario ${SPEC_FIXTURE} --adapter stub --adapter-outcome satisfied"
    cargo run -p vp-conformance-cli --bin vp-conformance -- run \
        --scenario "$SPEC_FIXTURE" \
        --adapter stub \
        --adapter-outcome satisfied
else
    echo "==> skipping smoke run: ${SPEC_FIXTURE} not found"
    echo "    clone veritypay-spec alongside this repository to run end-to-end spec scenario smoke"
fi

echo "readiness gate passed"
