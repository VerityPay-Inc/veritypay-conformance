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

run_spec_smoke() {
    local fixture_id="$1"
    local adapter_outcome="$2"
    local fixture="${ROOT}/../veritypay-spec/spec/conformance/scenarios/${fixture_id}.toml"

    if [[ -f "$fixture" ]]; then
        echo "==> vp-conformance run --scenario ${fixture} --adapter stub --adapter-outcome ${adapter_outcome}"
        cargo run -p vp-conformance-cli --bin vp-conformance -- run \
            --scenario "$fixture" \
            --adapter stub \
            --adapter-outcome "$adapter_outcome"
    else
        echo "==> skipping smoke run: ${fixture} not found"
        echo "    clone veritypay-spec alongside this repository to run ${fixture_id} smoke"
    fi
}

run_spec_smoke "VP-CS-0001" "satisfied"
run_spec_smoke "VP-CS-0002" "indeterminate"
run_spec_smoke "VP-CS-0011" "satisfied"
run_spec_smoke "VP-CS-0012" "not_satisfied"
run_spec_smoke "VP-CS-0013" "indeterminate"

echo "readiness gate passed"
