#!/bin/bash
# this_file: scripts/build-all-platforms.sh
# Build script to create deliverables for all platforms

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
CARGO_DIR="$PROJECT_ROOT"

# Get version from workspace Cargo.toml
VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d '"' -f 2)

log_info "Building all platform deliverables for version $VERSION"

# Clean and create dist directory
log_info "Cleaning distribution directory..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    log_error "cargo not found. Please install Rust."
fi

# Function to build for a specific target
build_target() {
    local TARGET=$1
    local OUTPUT_NAME=$2
    local IS_WINDOWS=$3
    
    log_info "Building for target: $TARGET"
    
    # Add the target if not already added
    rustup target add "$TARGET" 2>/dev/null || true
    
    # Build
    cd "$CARGO_DIR"
    if cargo build --release --target "$TARGET" -p vexy-svgo-cli; then
        # Copy the built binary
        if [ "$IS_WINDOWS" = "true" ]; then
            cp "target/$TARGET/release/vexy-svgo.exe" "$DIST_DIR/$OUTPUT_NAME.exe"
        else
            cp "target/$TARGET/release/vexy-svgo" "$DIST_DIR/$OUTPUT_NAME"
            chmod +x "$DIST_DIR/$OUTPUT_NAME"
        fi
        log_info "Successfully built $OUTPUT_NAME"
        return 0
    else
        log_warn "Failed to build for $TARGET"
        return 1
    fi
}

# Build for all platforms based on current OS
case "$(uname -s)" in
    Darwin*)
        log_info "Building on macOS - creating universal binary and attempting cross-compilation"
        
        # Build macOS universal binary
        log_info "Building macOS universal binary..."
        build_target "x86_64-apple-darwin" "vexy-svgo-macos-x86_64" false
        build_target "aarch64-apple-darwin" "vexy-svgo-macos-aarch64" false
        
        # Create universal binary
        if [ -f "$DIST_DIR/vexy-svgo-macos-x86_64" ] && [ -f "$DIST_DIR/vexy-svgo-macos-aarch64" ]; then
            log_info "Creating universal binary..."
            lipo -create \
                "$DIST_DIR/vexy-svgo-macos-x86_64" \
                "$DIST_DIR/vexy-svgo-macos-aarch64" \
                -output "$DIST_DIR/vexy-svgo-macos-universal"
            chmod +x "$DIST_DIR/vexy-svgo-macos-universal"
            
            # Clean up individual arch binaries
            rm "$DIST_DIR/vexy-svgo-macos-x86_64" "$DIST_DIR/vexy-svgo-macos-aarch64"
            
            log_info "Universal binary created successfully"
        fi
        
        # Try cross-compilation for Linux and Windows
        log_info "Attempting cross-compilation for other platforms..."
        build_target "x86_64-unknown-linux-gnu" "vexy-svgo-linux-x86_64" false || true
        build_target "x86_64-pc-windows-gnu" "vexy-svgo-windows-x86_64" true || true
        ;;
        
    Linux*)
        log_info "Building on Linux"
        
        # Build for Linux
        ARCH=$(uname -m)
        case "$ARCH" in
            x86_64) 
                build_target "x86_64-unknown-linux-gnu" "vexy-svgo-linux-x86_64" false
                ;;
            aarch64) 
                build_target "aarch64-unknown-linux-gnu" "vexy-svgo-linux-aarch64" false
                ;;
            armv7l) 
                build_target "armv7-unknown-linux-gnueabihf" "vexy-svgo-linux-armv7" false
                ;;
            *) 
                log_error "Unsupported Linux architecture: $ARCH"
                ;;
        esac
        
        # Try cross-compilation for other platforms
        log_info "Attempting cross-compilation for other platforms..."
        build_target "x86_64-pc-windows-gnu" "vexy-svgo-windows-x86_64" true || true
        ;;
        
    CYGWIN*|MINGW32*|MSYS*|MINGW64*)
        log_info "Building on Windows"
        
        # Build for Windows
        build_target "x86_64-pc-windows-msvc" "vexy-svgo-windows-x86_64" true
        
        # Cross-compilation from Windows is typically more difficult
        log_warn "Cross-compilation from Windows not implemented"
        ;;
        
    *)
        log_error "Unsupported operating system: $(uname -s)"
        ;;
esac

# Run packaging script
log_info "Running packaging script..."
if [ -x "$SCRIPT_DIR/package-deliverables.sh" ]; then
    "$SCRIPT_DIR/package-deliverables.sh" "$VERSION"
else
    log_warn "Package script not found or not executable"
fi

log_info "Build process completed!"
log_info "Deliverables created in: $DIST_DIR"

# List all created files
echo
log_info "Created files:"
find "$DIST_DIR" -type f -name "*.dmg" -o -name "*.pkg" -o -name "*.zip" -o -name "*.tar.gz" | sort