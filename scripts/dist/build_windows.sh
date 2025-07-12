#!/usr/bin/env bash
#
# Vexy SVGO Windows Distribution Script
# ===================================
#
# Build and package Vexy SVGO for Windows: creates a .zip archive containing the CLI executable.
#
# Usage:
#   ./scripts/dist/build_windows.sh
#
# What it does:
#   1. Builds the release binary for Windows using cargo (cross-compilation)
#   2. Creates a .zip archive for distribution
#
# Requirements:
#   - Linux host (for cross-compilation) or Windows host
#   - cargo (Rust toolchain)
#   - mingw-w64 (for x86_64-pc-windows-gnu target on Linux)
#   - zip (utility to create zip archives)
#
# Fails on any error (set -euo pipefail)
#
# Troubleshooting:
#   - If cross-compiling on Linux, ensure you have the x86_64-pc-windows-gnu target installed:
#     rustup target add x86_64-pc-windows-gnu
#   - Install mingw-w64: sudo apt-get install mingw-w64
#
# Best practices:
#   - Always run this script from a clean git state.
#   - Test the resulting .zip on a fresh Windows VM if possible.
#   - Keep this script in sync with the GitHub Actions workflow for releases.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../.. && pwd)"
DIST_DIR="$PROJECT_ROOT/dist/windows"
BIN_NAME="vexy-svgo"
VERSION=$(grep '^version =' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)

# Clean and prepare directories
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build release binary for Windows
log_info "Building release binary for Windows (x86_64-pc-windows-gnu)"
cd "$PROJECT_ROOT"
cargo build --release --target x86_64-pc-windows-gnu

# Copy binary and create zip archive
log_info "Creating .zip archive for Windows"
mkdir -p "$DIST_DIR/$BIN_NAME-$VERSION-windows"
cp "target/x86_64-pc-windows-gnu/release/$BIN_NAME.exe" "$DIST_DIR/$BIN_NAME-$VERSION-windows/"
cp "$PROJECT_ROOT/README.md" "$DIST_DIR/$BIN_NAME-$VERSION-windows/"
cp "$PROJECT_ROOT/LICENSE" "$DIST_DIR/$BIN_NAME-$VERSION-windows/"

cd "$DIST_DIR"
zip -r "$BIN_NAME-$VERSION-windows.zip" "$BIN_NAME-$VERSION-windows"
rm -rf "$BIN_NAME-$VERSION-windows" # Clean up staging directory

# Output result
ls -lh "$BIN_NAME-$VERSION-windows.zip"
log_info "Windows .zip created in $DIST_DIR"