#!/usr/bin/env bash
# Build and package Vexy SVGO for all supported platforms (macOS, Windows, Linux)
# Usage: ./scripts/dist/release_all.sh
#
# This script runs all platform-specific build scripts and collects deliverables in dist/
# Fails on any error.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Build for each platform
bash "$SCRIPT_DIR/build_macos.sh"
bash "$SCRIPT_DIR/build_windows.sh"
bash "$SCRIPT_DIR/build_linux.sh"

echo "All platform deliverables are in $PROJECT_ROOT/dist/"
ls -lh "$PROJECT_ROOT/dist"/*/*
