#!/bin/bash
# this_file: build.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Function to generate llms.txt
build_llms() {
    log_info "Generating code snapshot in ./llms.txt..."
    llms . "llms*.txt,*.d,*.json,*.html,*.svg,.specstory,ref,testdata,*.lock,*.svg,*.css,*.txt"
}

# Function to clean build artifacts
build_clean() {
    log_info "Cleaning build artifacts..."
    cargo clean
    rm -rf dist/
    rm -rf pkg*
    rm -f build.log.txt
    log_info "Clean complete"
}

# Function to build release version
build_release() {
    log_info "Building release version..."
    {
        # Call the consolidated build script
        ./scripts/build.sh

        echo "Running tests..."
        cargo test --release

        echo "Running linter (clippy)..."
        cargo clippy -- -D warnings

        echo "Checking code formatting..."
        cargo fmt --check

        echo "Build and verification complete."
        echo "To run the optimized binary, use: ./target/release/vexy-svgo"
    }

    log_info "Release build complete. Log saved to build.log.txt"
}

# Function to build debug version
build_debug() {
    log_info "Building debug version..."
    cargo build
    cargo test
    log_info "Debug build complete"
}

# Function to install CLI to /usr/local/bin
build_install() {
    if [[ "$OSTYPE" != "darwin"* ]]; then
        log_warn "Install command is currently only supported on macOS"
        exit 1
    fi

    log_info "Installing vexy-svgo to /usr/local/bin..."

    # First build release if not already built
    if [ ! -f "./target/release/vexy-svgo" ]; then
        log_info "Building release version first..."
        cargo build --release
    fi

    # Create /usr/local/bin if it doesn't exist
    sudo mkdir -p /usr/local/bin

    # Copy binary
    sudo cp ./target/release/vexy-svgo /usr/local/bin/
    sudo chmod +x /usr/local/bin/vexy-svgo

    log_info "Installation complete. You can now use 'vexy-svgo' from anywhere."

    # Verify installation
    if command -v vexy-svgo &>/dev/null; then
        log_info "Verification: vexy-svgo is available in PATH"
        vexy-svgo --version
    else
        log_warn "vexy-svgo was installed but may not be in your PATH"
    fi
}

# Function to build WASM
build_wasm() {
    log_info "Building WebAssembly modules..."

    # Check if wasm-pack is installed
    if ! command -v wasm-pack &>/dev/null; then
        log_error "wasm-pack is not installed. Please install it first:
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    fi

    # Run the WASM build script
    if [ -f "./build-wasm.sh" ]; then
        ./build-wasm.sh
    else
        log_error "build-wasm.sh not found"
    fi
}

# Function to build platform deliverables
build_deliverables() {
    log_info "Building platform deliverables..."

    # Make scripts executable
    chmod +x ./scripts/build-all-platforms.sh
    chmod +x ./scripts/package-deliverables.sh

    # Run the build script
    ./scripts/build-all-platforms.sh
}

# Function to run all builds
build_all() {
    log_info "Running complete build process..."

    # Generate llms.txt
    build_llms

    # Clean first
    build_clean

    # Build release
    build_release

    # Build WASM if wasm-pack is available
    if command -v wasm-pack &>/dev/null; then
        build_wasm
    else
        log_warn "Skipping WASM build (wasm-pack not installed)"
    fi

    log_info "Complete build process finished"
}

# Show usage
show_usage() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  llms         Generate code snapshot in llms.txt"
    echo "  clean        Clean all build artifacts"
    echo "  release      Build optimized release version"
    echo "  debug        Build debug version"
    echo "  install      Install CLI to /usr/local/bin (macOS only)"
    echo "  wasm         Build WebAssembly modules"
    echo "  deliverables Build platform-specific packages (dmg, zip, tar.gz)"
    echo "  help         Show this help message"
    echo ""
    echo "If no command is specified, runs the complete build process."
}

# Main script logic
case "${1:-all}" in
llms)
    build_llms
    ;;
clean)
    build_clean
    ;;
release)
    build_release
    ;;
debug)
    build_debug
    ;;
install)
    build_install
    ;;
wasm)
    build_wasm
    ;;
deliverables)
    build_deliverables
    ;;
all)
    build_all
    ;;
help | --help | -h)
    show_usage
    ;;
*)
    log_error "Unknown command: $1"
    show_usage
    exit 1
    ;;
esac
