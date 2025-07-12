#!/bin/bash
# Package Vexy SVGO release artifacts
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

if [ $# -eq 0 ]; then
    log_error "Version number required"
    echo "Usage: $0 <version>"
    exit 1
fi

VERSION=$1
DIST_DIR="dist"
RELEASE_DIR="release"

# Clean and create release directory
log_info "Preparing release directory"
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Detect platform
PLATFORM=$(uname -s)
ARCH=$(uname -m)

case "$PLATFORM" in
    Darwin)
        PLATFORM_NAME="macos"
        if [ "$ARCH" = "arm64" ]; then
            ARCH_NAME="aarch64"
        else
            ARCH_NAME="x86_64"
        fi
        ;;
    Linux)
        PLATFORM_NAME="linux"
        ARCH_NAME="x86_64"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM_NAME="windows"
        ARCH_NAME="x86_64"
        ;;
    *)
        PLATFORM_NAME="unknown"
        ARCH_NAME="unknown"
        ;;
esac

BINARY_NAME="vexy-svgo-$VERSION-$PLATFORM_NAME-$ARCH_NAME"

# Package CLI binary
log_info "Packaging CLI binary"
if [ -f "$DIST_DIR/vexy-svgo" ]; then
    mkdir -p "$RELEASE_DIR/$BINARY_NAME"
    cp "$DIST_DIR/vexy-svgo" "$RELEASE_DIR/$BINARY_NAME/"
    cp README.md "$RELEASE_DIR/$BINARY_NAME/"
    cp LICENSE "$RELEASE_DIR/$BINARY_NAME/"
    cp RELEASE_CANDIDATE.md "$RELEASE_DIR/$BINARY_NAME/"
    
    # Create archive
    cd "$RELEASE_DIR"
    if [ "$PLATFORM_NAME" = "windows" ]; then
        zip -r "$BINARY_NAME.zip" "$BINARY_NAME/"
    else
        tar -czf "$BINARY_NAME.tar.gz" "$BINARY_NAME/"
    fi
    cd ..
    
    log_success "CLI binary packaged as $BINARY_NAME"
else
    log_error "CLI binary not found in $DIST_DIR"
    exit 1
fi

# Package WASM modules
log_info "Packaging WebAssembly modules"
if [ -d "$DIST_DIR/wasm" ]; then
    WASM_NAME="vexy-svgo-wasm-$VERSION"
    mkdir -p "$RELEASE_DIR/$WASM_NAME"
    cp -r "$DIST_DIR/wasm/"* "$RELEASE_DIR/$WASM_NAME/"
    cp README.md "$RELEASE_DIR/$WASM_NAME/"
    cp LICENSE "$RELEASE_DIR/$WASM_NAME/"
    
    cd "$RELEASE_DIR"
    tar -czf "$WASM_NAME.tar.gz" "$WASM_NAME/"
    cd ..
    
    log_success "WASM modules packaged as $WASM_NAME"
else
    log_error "WASM modules not found in $DIST_DIR"
    exit 1
fi

# Create checksums
log_info "Creating checksums"
cd "$RELEASE_DIR"
for file in *.tar.gz *.zip; do
    if [ -f "$file" ]; then
        sha256sum "$file" > "$file.sha256"
        log_info "Created checksum for $file"
    fi
done
cd ..

# Create release notes
log_info "Creating release notes"
cat > "$RELEASE_DIR/RELEASE_NOTES.md" << EOF
# Vexy SVGO $VERSION Release Notes

## Overview
VexySVGO $VERSION is a high-performance SVG optimizer written in Rust, designed as a modern alternative to SVGO.

## Release Artifacts

### CLI Binaries
- **$BINARY_NAME.tar.gz** - Command-line interface for $PLATFORM_NAME ($ARCH_NAME)
- **$BINARY_NAME.tar.gz.sha256** - SHA256 checksum

### WebAssembly Modules  
- **vexy-svgo-wasm-$VERSION.tar.gz** - WebAssembly modules for browser and Node.js
- **vexy-svgo-wasm-$VERSION.tar.gz.sha256** - SHA256 checksum

## Installation

### CLI Binary
```bash
# Download and extract
wget https://github.com/vexyart/vexy-svgo/releases/download/v$VERSION/$BINARY_NAME.tar.gz
tar -xzf $BINARY_NAME.tar.gz
cd $BINARY_NAME

# Install to PATH
sudo cp vexy-svgo /usr/local/bin/
```

### WebAssembly
```bash
# Download and extract
wget https://github.com/vexyart/vexy-svgo/releases/download/v$VERSION/vexy-svgo-wasm-$VERSION.tar.gz
tar -xzf vexy-svgo-wasm-$VERSION.tar.gz
```

## Verification
Verify the integrity of downloaded files:
```bash
sha256sum -c $BINARY_NAME.tar.gz.sha256
sha256sum -c vexy-svgo-wasm-$VERSION.tar.gz.sha256
```

## What's New
See [RELEASE_CANDIDATE.md](RELEASE_CANDIDATE.md) for comprehensive release notes.

## Support
- **Documentation**: https://twardoch.github.io/vexy-svgo/
- **Issues**: https://github.com/vexyart/vexy-svgo/issues
- **Discussions**: https://github.com/vexyart/vexy-svgo/discussions
EOF

log_success "Release packaging completed!"
log_info "Release artifacts available in: $RELEASE_DIR"
log_info "Files created:"
for file in "$RELEASE_DIR"/*; do
    log_info "  - $(basename "$file")"
done
