#!/bin/bash
# this_file: scripts/package-deliverables.sh
# Script to package platform-specific deliverables with installers

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Check if dist directory exists
if [ ! -d "$DIST_DIR" ]; then
    log_error "Distribution directory not found. Run build.sh first."
fi

log_info "Packaging deliverables for version $VERSION"

# Create platform-specific directories
mkdir -p "$DIST_DIR/macos"
mkdir -p "$DIST_DIR/windows"
mkdir -p "$DIST_DIR/linux"

# Function to create macOS .pkg installer
create_macos_pkg() {
    log_info "Creating macOS .pkg installer..."
    
    local PKG_ROOT="$DIST_DIR/macos/pkg-root"
    local SCRIPTS_DIR="$DIST_DIR/macos/scripts"
    
    # Create package structure
    mkdir -p "$PKG_ROOT/usr/local/bin"
    mkdir -p "$SCRIPTS_DIR"
    
    # Copy binary
    cp "$DIST_DIR/vexy-svgo-macos-universal" "$PKG_ROOT/usr/local/bin/vexy-svgo"
    chmod 755 "$PKG_ROOT/usr/local/bin/vexy-svgo"
    
    # Create postinstall script
    cat > "$SCRIPTS_DIR/postinstall" << 'EOF'
#!/bin/bash
# Ensure /usr/local/bin is in PATH
if ! echo "$PATH" | grep -q "/usr/local/bin"; then
    echo "Note: Please ensure /usr/local/bin is in your PATH"
fi
exit 0
EOF
    chmod 755 "$SCRIPTS_DIR/postinstall"
    
    # Build the package
    pkgbuild \
        --root "$PKG_ROOT" \
        --scripts "$SCRIPTS_DIR" \
        --identifier "com.vexyart.vexy-svgo" \
        --version "$VERSION" \
        --ownership recommended \
        "$DIST_DIR/macos/vexy-svgo-$VERSION.pkg"
    
    # Clean up temporary files
    rm -rf "$PKG_ROOT" "$SCRIPTS_DIR"
    
    log_info "Created macOS .pkg installer"
}

# Function to create macOS .dmg
create_macos_dmg() {
    log_info "Creating macOS .dmg..."
    
    local DMG_DIR="$DIST_DIR/macos/dmg-contents"
    mkdir -p "$DMG_DIR"
    
    # Copy .pkg to DMG contents
    cp "$DIST_DIR/macos/vexy-svgo-$VERSION.pkg" "$DMG_DIR/"
    
    # Create a simple README
    cat > "$DMG_DIR/README.txt" << EOF
Vexy SVGO v$VERSION
==================

To install:
1. Double-click on vexy-svgo-$VERSION.pkg
2. Follow the installation wizard
3. The 'vexy-svgo' command will be available in Terminal

The binary will be installed to: /usr/local/bin/vexy-svgo

For more information, visit: https://github.com/vexyart/vexy-svgo
EOF
    
    # Create DMG
    hdiutil create -volname "Vexy SVGO $VERSION" \
        -srcfolder "$DMG_DIR" \
        -ov \
        -format UDZO \
        "$DIST_DIR/macos/vexy-svgo-$VERSION.dmg"
    
    # Clean up
    rm -rf "$DMG_DIR"
    
    log_info "Created macOS .dmg"
}

# Function to package Windows deliverables
package_windows() {
    log_info "Packaging Windows deliverables..."
    
    # Find Windows executable
    local WIN_EXE=$(find "$DIST_DIR" -name "vexy-svgo-windows-*.exe" | head -1)
    if [ -z "$WIN_EXE" ]; then
        log_warn "No Windows executable found, skipping Windows packaging"
        return
    fi
    
    local WIN_DIR="$DIST_DIR/windows/vexy-svgo-$VERSION"
    mkdir -p "$WIN_DIR"
    
    # Copy executable
    cp "$WIN_EXE" "$WIN_DIR/vexy-svgo.exe"
    
    # Create README
    cat > "$WIN_DIR/README.txt" << EOF
Vexy SVGO v$VERSION
==================

To use:
1. Add this directory to your PATH, or
2. Copy vexy-svgo.exe to a directory already in your PATH

Usage: vexy-svgo [OPTIONS] <INPUT>

For more information, visit: https://github.com/vexyart/vexy-svgo
EOF
    
    # Create ZIP
    cd "$DIST_DIR/windows"
    zip -r "vexy-svgo-$VERSION-windows.zip" "vexy-svgo-$VERSION"
    rm -rf "vexy-svgo-$VERSION"
    cd "$PROJECT_ROOT"
    
    log_info "Created Windows .zip package"
}

# Function to package Linux deliverables
package_linux() {
    log_info "Packaging Linux deliverables..."
    
    # Find Linux executable
    local LINUX_EXE=$(find "$DIST_DIR" -name "vexy-svgo-linux-*" -type f | head -1)
    if [ -z "$LINUX_EXE" ]; then
        log_warn "No Linux executable found, skipping Linux packaging"
        return
    fi
    
    local LINUX_DIR="$DIST_DIR/linux/vexy-svgo-$VERSION"
    mkdir -p "$LINUX_DIR/bin"
    
    # Copy executable
    cp "$LINUX_EXE" "$LINUX_DIR/bin/vexy-svgo"
    chmod 755 "$LINUX_DIR/bin/vexy-svgo"
    
    # Create install script
    cat > "$LINUX_DIR/install.sh" << 'EOF'
#!/bin/bash
# Vexy SVGO installer script

INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

echo "Installing vexy-svgo to $INSTALL_DIR"
echo "You may need to use sudo if installing to a system directory."

cp bin/vexy-svgo "$INSTALL_DIR/"
chmod 755 "$INSTALL_DIR/vexy-svgo"

echo "Installation complete!"
echo "Make sure $INSTALL_DIR is in your PATH."
EOF
    chmod 755 "$LINUX_DIR/install.sh"
    
    # Create README
    cat > "$LINUX_DIR/README.txt" << EOF
Vexy SVGO v$VERSION
==================

To install:
1. Run: ./install.sh
   Or manually copy bin/vexy-svgo to a directory in your PATH

Usage: vexy-svgo [OPTIONS] <INPUT>

For more information, visit: https://github.com/vexyart/vexy-svgo
EOF
    
    # Create tar.gz
    cd "$DIST_DIR/linux"
    tar -czf "vexy-svgo-$VERSION-linux.tar.gz" "vexy-svgo-$VERSION"
    rm -rf "vexy-svgo-$VERSION"
    cd "$PROJECT_ROOT"
    
    log_info "Created Linux .tar.gz package"
}

# Main packaging process
main() {
    log_info "Starting packaging process..."
    
    # Detect current platform and package accordingly
    case "$(uname -s)" in
        Darwin*)
            if [ -f "$DIST_DIR/vexy-svgo-macos-universal" ]; then
                create_macos_pkg
                create_macos_dmg
            else
                log_warn "macOS binary not found, skipping macOS packaging"
            fi
            ;;
        Linux*)
            package_linux
            ;;
        CYGWIN*|MINGW32*|MSYS*|MINGW64*)
            package_windows
            ;;
    esac
    
    # Always try to package other platforms if their binaries exist
    if [[ "$(uname -s)" != "Darwin"* ]] && [ -f "$DIST_DIR/vexy-svgo-macos-universal" ]; then
        log_warn "Cannot create macOS .pkg/.dmg on non-macOS system"
    fi
    
    if [[ "$(uname -s)" != "Linux"* ]] && ls "$DIST_DIR"/vexy-svgo-linux-* >/dev/null 2>&1; then
        package_linux
    fi
    
    if [[ "$(uname -s)" != CYGWIN* && "$(uname -s)" != MINGW* ]] && ls "$DIST_DIR"/vexy-svgo-windows-*.exe >/dev/null 2>&1; then
        package_windows
    fi
    
    log_info "Packaging complete!"
    log_info "Platform-specific packages:"
    [ -d "$DIST_DIR/macos" ] && ls -la "$DIST_DIR/macos/"
    [ -d "$DIST_DIR/windows" ] && ls -la "$DIST_DIR/windows/"
    [ -d "$DIST_DIR/linux" ] && ls -la "$DIST_DIR/linux/"
}

# Run main function
main