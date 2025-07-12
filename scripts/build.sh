#!/bin/bash
# this_file: scripts/build.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    exit 1
}

# Setup paths
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
DIST_DIR="$PROJECT_ROOT/dist"
CARGO_DIR="$PROJECT_ROOT" # Building from workspace root

# Clean and create dist directory
log_info "Cleaning and creating distribution directory..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    log_error "cargo not found. Please install Rust."
fi

# Determine OS and architecture
OS_TYPE=$(uname -s)
ARCH_TYPE=$(uname -m)

log_info "Detected OS: $OS_TYPE, Architecture: $ARCH_TYPE"

case "$OS_TYPE" in
    Linux*)
        TARGET=""
        case "$ARCH_TYPE" in
            x86_64) TARGET="x86_64-unknown-linux-gnu";;
            aarch64) TARGET="aarch64-unknown-linux-gnu";;
            armv7l) TARGET="armv7-unknown-linux-gnueabihf";;
            *) log_error "Unsupported Linux architecture: $ARCH_TYPE";;
        esac
        log_info "Building for Linux ($TARGET)..."
        rustup target add "$TARGET" || true # Add target if not already added
        cd "$CARGO_DIR"
        cargo build --release --target "$TARGET" -p vexy_svgo-cli
        cp "target/$TARGET/release/vexy-svgo" "$DIST_DIR/vexy-svgo-linux-$ARCH_TYPE"
        chmod +x "$DIST_DIR/vexy-svgo-linux-$ARCH_TYPE"
        log_info "Stripping binary..."
        strip "$DIST_DIR/vexy-svgo-linux-$ARCH_TYPE" || log_warn "strip command not found, skipping binary stripping."
        ;;
    Darwin*)
        log_info "Building for macOS (universal binary)..."
        rustup target add x86_64-apple-darwin || true
        rustup target add aarch64-apple-darwin || true
        cd "$CARGO_DIR"
        cargo build --release --target x86_64-apple-darwin -p vexy_svgo-cli
        cargo build --release --target aarch64-apple-darwin -p vexy_svgo-cli
        log_info "Creating universal binary..."
        lipo -create \
            "$PROJECT_ROOT/target/x86_64-apple-darwin/release/vexy-svgo" \
            "$PROJECT_ROOT/target/aarch64-apple-darwin/release/vexy-svgo" \
            -output "$DIST_DIR/vexy-svgo-macos-universal"
        chmod +x "$DIST_DIR/vexy-svgo-macos-universal"
        log_info "Verifying universal binary..."
        file "$DIST_DIR/vexy-svgo-macos-universal"
        lipo -info "$DIST_DIR/vexy-svgo-macos-universal"
        ;;
    CYGWIN*|MINGW32*|MSYS*|MINGW64*)
        log_info "Building for Windows..."
        TARGET=""
        case "$ARCH_TYPE" in
            x86_64) TARGET="x86_64-pc-windows-msvc";;
            i686) TARGET="i686-pc-windows-msvc";;
            aarch64) TARGET="aarch64-pc-windows-msvc";;
            *) log_error "Unsupported Windows architecture: $ARCH_TYPE";;
        esac
        rustup target add "$TARGET" || true
        cd "$CARGO_DIR"
        cargo build --release --target "$TARGET" -p vexy_svgo-cli
        cp "target/$TARGET/release/vexy-svgo.exe" "$DIST_DIR/vexy-svgo-windows-$ARCH_TYPE.exe"
        ;;
    *)
        log_error "Unsupported operating system: $OS_TYPE"
        ;;
esac

# Get version from workspace Cargo.toml
VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d '"' -f 2)
log_info "Built version: $VERSION"

# Create archives
log_info "Creating archives..."
cd "$DIST_DIR"
if [[ "$OS_TYPE" == "Linux"* ]]; then
    tar -czf "vexy-svgo-$VERSION-linux-$ARCH_TYPE.tar.gz" "vexy-svgo-linux-$ARCH_TYPE"
elif [[ "$OS_TYPE" == "Darwin"* ]]; then
    tar -czf "vexy-svgo-$VERSION-macos-universal.tar.gz" "vexy-svgo-macos-universal"
elif [[ "$OS_TYPE" == "CYGWIN"* || "$OS_TYPE" == "MINGW32"* || "$OS_TYPE" == "MSYS"* || "$OS_TYPE" == "MINGW64"* ]]; then
    zip "vexy-svgo-$VERSION-windows-$ARCH_TYPE.zip" "vexy-svgo-windows-$ARCH_TYPE.exe"
fi
cd "$PROJECT_ROOT"

log_info "Build process completed successfully."
log_info "Artifacts are in: $DIST_DIR"

# Build WASM if script exists and wasm-pack is available
if [ -f "$PROJECT_ROOT/build-wasm-v2.sh" ] && command -v wasm-pack &> /dev/null; then
    log_info "Building WebAssembly modules..."
    cd "$PROJECT_ROOT"
    ./build-wasm-v2.sh
    log_info "WASM build completed."
elif [ -f "$PROJECT_ROOT/build-wasm.sh" ] && command -v wasm-pack &> /dev/null; then
    log_info "Building WebAssembly modules (legacy)..."
    cd "$PROJECT_ROOT"
    ./build-wasm.sh
    log_info "WASM build completed."
else
    log_warn "wasm-pack not found or WASM build script missing, skipping WASM build"
fi
