#!/bin/bash
# this_file: scripts/package-current-platform.sh
# Simplified script to package deliverables for the current platform only

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
VERSION="${1:-$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d '"' -f 2)}"

log_info "Packaging deliverables for current platform (version $VERSION)"

# First, build using the existing build script
log_info "Building release..."
"$SCRIPT_DIR/build.sh"

# Check if dist directory was created
if [ ! -d "$DIST_DIR" ]; then
    log_error "Build failed - no dist directory found"
fi

# Package based on current platform
case "$(uname -s)" in
    Darwin*)
        log_info "Packaging for macOS..."
        
        # Check if binary exists
        if [ ! -f "$DIST_DIR/vexy-svgo-macos-universal" ]; then
            log_error "macOS binary not found"
        fi
        
        # Create platform directory
        mkdir -p "$DIST_DIR/macos"
        
        # Create simple installer script
        cat > "$DIST_DIR/macos/install.sh" << 'EOF'
#!/bin/bash
# Vexy SVGO macOS installer

echo "Installing vexy-svgo to /usr/local/bin..."
sudo mkdir -p /usr/local/bin
sudo cp vexy-svgo /usr/local/bin/
sudo chmod 755 /usr/local/bin/vexy-svgo

echo "Installation complete!"
echo "You can now use 'vexy-svgo' from Terminal."
EOF
        chmod 755 "$DIST_DIR/macos/install.sh"
        
        # Create package directory
        PACKAGE_DIR="$DIST_DIR/macos/vexy-svgo-$VERSION-macos"
        mkdir -p "$PACKAGE_DIR"
        
        # Copy files
        cp "$DIST_DIR/vexy-svgo-macos-universal" "$PACKAGE_DIR/vexy-svgo"
        chmod 755 "$PACKAGE_DIR/vexy-svgo"
        cp "$DIST_DIR/macos/install.sh" "$PACKAGE_DIR/"
        
        # Create README
        cat > "$PACKAGE_DIR/README.txt" << EOF
Vexy SVGO v$VERSION for macOS
=============================

Quick Install:
  ./install.sh

Manual Install:
  Copy vexy-svgo to a directory in your PATH

Usage:
  vexy-svgo [OPTIONS] <INPUT>

For more information:
  https://github.com/vexyart/vexy-svgo
EOF
        
        # Create DMG using hdiutil
        log_info "Creating DMG..."
        cd "$DIST_DIR/macos"
        hdiutil create -volname "Vexy SVGO $VERSION" \
            -srcfolder "vexy-svgo-$VERSION-macos" \
            -ov \
            -format UDZO \
            "vexy-svgo-$VERSION-macos.dmg"
        
        # Also create a simple tar.gz
        tar -czf "vexy-svgo-$VERSION-macos.tar.gz" "vexy-svgo-$VERSION-macos"
        
        # Clean up
        rm -rf "vexy-svgo-$VERSION-macos"
        rm install.sh
        
        log_info "Created macOS packages:"
        log_info "  - $DIST_DIR/macos/vexy-svgo-$VERSION-macos.dmg"
        log_info "  - $DIST_DIR/macos/vexy-svgo-$VERSION-macos.tar.gz"
        ;;
        
    Linux*)
        log_info "Packaging for Linux..."
        
        # Find the Linux binary
        LINUX_BINARY=$(find "$DIST_DIR" -name "vexy-svgo-linux-*" -type f | head -1)
        if [ -z "$LINUX_BINARY" ]; then
            log_error "Linux binary not found"
        fi
        
        # Create platform directory
        mkdir -p "$DIST_DIR/linux"
        
        # Create package directory
        PACKAGE_DIR="$DIST_DIR/linux/vexy-svgo-$VERSION-linux"
        mkdir -p "$PACKAGE_DIR/bin"
        
        # Copy binary
        cp "$LINUX_BINARY" "$PACKAGE_DIR/bin/vexy-svgo"
        chmod 755 "$PACKAGE_DIR/bin/vexy-svgo"
        
        # Create install script
        cat > "$PACKAGE_DIR/install.sh" << 'EOF'
#!/bin/bash
# Vexy SVGO Linux installer

INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

echo "Installing vexy-svgo to $INSTALL_DIR"
echo "You may need to use sudo if installing to a system directory."

mkdir -p "$INSTALL_DIR"
cp bin/vexy-svgo "$INSTALL_DIR/"
chmod 755 "$INSTALL_DIR/vexy-svgo"

echo "Installation complete!"
echo "Make sure $INSTALL_DIR is in your PATH."
EOF
        chmod 755 "$PACKAGE_DIR/install.sh"
        
        # Create README
        cat > "$PACKAGE_DIR/README.txt" << EOF
Vexy SVGO v$VERSION for Linux
=============================

Quick Install:
  ./install.sh

Manual Install:
  Copy bin/vexy-svgo to a directory in your PATH

Usage:
  vexy-svgo [OPTIONS] <INPUT>

For more information:
  https://github.com/vexyart/vexy-svgo
EOF
        
        # Create tar.gz
        cd "$DIST_DIR/linux"
        tar -czf "vexy-svgo-$VERSION-linux.tar.gz" "vexy-svgo-$VERSION-linux"
        rm -rf "vexy-svgo-$VERSION-linux"
        
        log_info "Created Linux package:"
        log_info "  - $DIST_DIR/linux/vexy-svgo-$VERSION-linux.tar.gz"
        ;;
        
    CYGWIN*|MINGW*|MSYS*)
        log_info "Packaging for Windows..."
        
        # Find Windows exe
        WIN_EXE=$(find "$DIST_DIR" -name "*.exe" | head -1)
        if [ -z "$WIN_EXE" ]; then
            log_error "Windows executable not found"
        fi
        
        # Create platform directory
        mkdir -p "$DIST_DIR/windows"
        
        # Create package directory
        PACKAGE_DIR="$DIST_DIR/windows/vexy-svgo-$VERSION-windows"
        mkdir -p "$PACKAGE_DIR"
        
        # Copy executable
        cp "$WIN_EXE" "$PACKAGE_DIR/vexy-svgo.exe"
        
        # Create README
        cat > "$PACKAGE_DIR/README.txt" << EOF
Vexy SVGO v$VERSION for Windows
===============================

Installation:
  1. Copy vexy-svgo.exe to a directory in your PATH, or
  2. Add this directory to your PATH environment variable

Usage:
  vexy-svgo [OPTIONS] <INPUT>

For more information:
  https://github.com/vexyart/vexy-svgo
EOF
        
        # Create batch file for adding to PATH
        cat > "$PACKAGE_DIR/add-to-path.bat" << 'EOF'
@echo off
echo Adding %CD% to PATH...
setx PATH "%PATH%;%CD%"
echo Done! Please restart your command prompt for changes to take effect.
pause
EOF
        
        # Create ZIP
        cd "$DIST_DIR/windows"
        zip -r "vexy-svgo-$VERSION-windows.zip" "vexy-svgo-$VERSION-windows"
        rm -rf "vexy-svgo-$VERSION-windows"
        
        log_info "Created Windows package:"
        log_info "  - $DIST_DIR/windows/vexy-svgo-$VERSION-windows.zip"
        ;;
        
    *)
        log_error "Unsupported platform: $(uname -s)"
        ;;
esac

log_info "Packaging complete!"