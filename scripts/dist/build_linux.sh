#!/usr/bin/env bash
# Build and package VEXYSVGO for Linux: creates a .tar.gz containing the CLI binary
# Usage: ./scripts/dist/build_linux.sh
#
# This script builds the release binary for Linux, tars and gzips it, and places it in dist/linux.
# Requirements: cargo, tar, gzip
#
# Fails on any error.
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist/linux"
BIN_NAME="vexy_svgo"
VERSION=$(grep '^version =' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)

# Clean and prepare directories
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build release binary
cd "$PROJECT_ROOT"
cargo build --release
cp "target/release/$BIN_NAME" "$DIST_DIR/"
chmod +x "$DIST_DIR/$BIN_NAME"

# Tar and gzip the binary
cd "$DIST_DIR"
TAR_NAME="vexy_svgo-$VERSION-linux.tar.gz"
tar -czvf "$TAR_NAME" "$BIN_NAME"

# Output result
ls -lh "$TAR_NAME"
echo "Linux .tar.gz created in $DIST_DIR"
