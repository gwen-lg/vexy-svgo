#!/bin/bash
# Build all VEXYSVGO targets
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get the target directory
TARGET_DIR="target/release"
DIST_DIR="dist"

# Clean previous builds
log_info "Cleaning previous builds"
cargo clean
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build CLI for current platform
log_info "Building CLI for current platform"
if ! cargo build --release --package vexy_svgo-cli; then
    log_error "CLI build failed"
    exit 1
fi

# Build WASM modules
log_info "Building WebAssembly modules"
if ! ./build-wasm.sh; then
    log_error "WASM build failed"
    exit 1
fi

# Build optimized WASM
log_info "Building optimized WebAssembly modules"
if ! ./build-wasm-optimized.sh; then
    log_error "Optimized WASM build failed"
    exit 1
fi

# Copy CLI binary to dist
log_info "Copying CLI binary to dist"
cp "$TARGET_DIR/vexy_svgo" "$DIST_DIR/"

# Copy WASM modules to dist
log_info "Copying WASM modules to dist"
mkdir -p "$DIST_DIR/wasm"
if [ -d "crates/wasm/pkg-web" ]; then
    cp -r crates/wasm/pkg-web "$DIST_DIR/wasm/"
fi
if [ -d "crates/wasm/pkg-node" ]; then
    cp -r crates/wasm/pkg-node "$DIST_DIR/wasm/"
fi
if [ -d "crates/wasm/pkg-bundler" ]; then
    cp -r crates/wasm/pkg-bundler "$DIST_DIR/wasm/"
fi

# Build docs
log_info "Building documentation"
if ! cargo doc --no-deps --workspace; then
    log_warning "Documentation build failed, continuing anyway"
fi

log_success "All builds completed successfully!"
log_info "Binaries available in: $DIST_DIR"
log_info "Documentation available in: target/doc"