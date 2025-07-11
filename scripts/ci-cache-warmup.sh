#!/bin/bash
# this_file: scripts/ci-cache-warmup.sh

# Script to warm up CI caches by pre-building dependencies
set -e

echo "Warming up CI caches..."

# Download and compile dependencies only (no project code)
echo "Downloading dependencies..."
cargo fetch

# Build dependencies in release mode
echo "Building dependencies..."
cargo build --release --workspace --dependencies-only 2>/dev/null || \
  cargo build --release --workspace

# Pre-build common test dependencies
echo "Building test dependencies..."
cargo test --release --workspace --no-run

# Pre-build benchmark dependencies
echo "Building benchmark dependencies..."
cargo bench --workspace --no-run

# Generate Cargo.lock if it doesn't exist
if [ ! -f Cargo.lock ]; then
    echo "Generating Cargo.lock..."
    cargo generate-lockfile
fi

echo "Cache warmup complete!"