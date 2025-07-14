#!/bin/bash
set -euo pipefail

echo "--- Verifying clean release build ---"
cargo clean
cargo build --release

echo "--- Running tests against release build ---"
cargo test --release

echo "--- Build verification successful ---"
