#!/usr/bin/env bash
# The gate: formatting, lints, tests. CI runs this script too.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

# Spotlight skips a directory carrying this marker. `cargo clean` deletes it
# with target/, so every run puts it back.
mkdir -p target
touch target/.metadata_never_index

cargo fmt --check
cargo clippy --all-targets
cargo test
