#!/usr/bin/env bash
#
# Vexy SVGO Linux Distribution Script
# =================================
#
# Build and package Vexy SVGO for Linux: creates a .tar.gz archive containing the CLI executable.
#
# Usage:
#   ./scripts/dist/build_linux.sh
#
# What it does:
#   1. Builds the release binary for Linux using cargo
#   2. Creates a .tar.gz archive for distribution
#
# Requirements:
#   - Linux host
#   - cargo (Rust toolchain)
#   - tar (utility to create tar.gz archives)
#
# Fails on any error (set -euo pipefail)
#
# Troubleshooting:
#   - Ensure you have the necessary build tools installed on your Linux system.
#
# Best practices:
#   - Always run this script from a clean git state.
#   - Test the resulting .tar.gz on a fresh Linux VM if possible.
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
DIST_DIR="$PROJECT_ROOT/dist/linux"
BIN_NAME="vexy-svgo"
VERSION=$(grep '^version =' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)

# Clean and prepare directories
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build release binary for Linux
log_info "Building release binary for Linux (x86_64-unknown-linux-gnu)"
cd "$PROJECT_ROOT"
cargo build --release --target x86_64-unknown-linux-gnu

# Copy binary and create tar.gz archive
log_info "Creating .tar.gz archive for Linux"
mkdir -p "$DIST_DIR/$BIN_NAME-$VERSION-linux"
cp "target/x86_64-unknown-linux-gnu/release/$BIN_NAME" "$DIST_DIR/$BIN_NAME-$VERSION-linux/"
cp "$PROJECT_ROOT/README.md" "$DIST_DIR/$BIN_NAME-$VERSION-linux/"
cp "$PROJECT_ROOT/LICENSE" "$DIST_DIR/$BIN_NAME-$VERSION-linux/"

cd "$DIST_DIR"
tar -czf "$BIN_NAME-$VERSION-linux.tar.gz" "$BIN_NAME-$VERSION-linux"
rm -rf "$BIN_NAME-$VERSION-linux" # Clean up staging directory

# Output result
ls -lh "$BIN_NAME-$VERSION-linux.tar.gz"
log_info "Linux .tar.gz created in $DIST_DIR"