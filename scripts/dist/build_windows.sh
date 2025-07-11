#!/usr/bin/env bash
# Build and package VEXYSVGO for Windows: creates a .zip containing the CLI .exe
# Usage: ./scripts/dist/build_windows.sh
#
# This script cross-compiles the release binary for Windows, zips it, and places it in dist/windows.
# Requirements: cargo, zip, x86_64-pc-windows-gnu toolchain (install with rustup)
#
# Fails on any error.
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist/windows"
BIN_NAME="vexy_svgo.exe"
VERSION=$(grep '^version =' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)

# Clean and prepare directories
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build release binary for Windows (x86_64)
cd "$PROJECT_ROOT"
cargo build --release --target x86_64-pc-windows-gnu
cp "target/x86_64-pc-windows-gnu/release/vexy_svgo.exe" "$DIST_DIR/"

# Zip the binary
cd "$DIST_DIR"
ZIP_NAME="vexy_svgo-$VERSION-windows.zip"
zip "$ZIP_NAME" vexy_svgo.exe

# Output result
ls -lh "$ZIP_NAME"
echo "Windows .zip created in $DIST_DIR"
