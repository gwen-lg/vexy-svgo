#!/usr/bin/env bash
#
# Vexy SVGO macOS Distribution Script
# ==============================
#
# Build and package Vexy SVGO for macOS: creates a .dmg containing a .pkg installer for the CLI tool.
#
# Usage:
#   ./scripts/dist/build_macos.sh
#
# What it does:
#   1. Builds the release binary using cargo
#   2. Creates a .pkg installer that installs the CLI tool into /usr/local/bin
#   3. Wraps the .pkg in a .dmg disk image for easy distribution
#
# Requirements:
#   - macOS host (for .pkg/.dmg creation)
#   - cargo (Rust toolchain)
#   - pkgbuild (Xcode command line tools)
#   - hdiutil (macOS disk image utility)
#   - codesign (optional, for signing the .pkg)
#
# Fails on any error (set -euo pipefail)
#
# Troubleshooting:
#   - If you see permission errors, ensure you have write access to the dist/ directory.
#   - If pkgbuild or hdiutil are missing, install Xcode command line tools: xcode-select --install
#   - For signing, set up your Developer ID Installer certificate in your keychain.
#
# Best practices:
#   - Always run this script from a clean git state.
#   - Test the resulting .pkg and .dmg on a fresh macOS VM if possible.
#   - Keep this script in sync with the GitHub Actions workflow for releases.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")"/../.. && pwd)"
DIST_DIR="$PROJECT_ROOT/dist/macos"
BIN_NAME="vexy-svgo"
PKG_ID="com.twardoch.vexy-svgo"
VERSION=$(grep '^version =' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)

# Clean and prepare directories
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR/bin"

# Build release binary
cd "$PROJECT_ROOT"
cargo build --release
cp "target/release/$BIN_NAME" "$DIST_DIR/bin/"
chmod +x "$DIST_DIR/bin/$BIN_NAME"

# Create the .pkg installer
PKG_PATH="$DIST_DIR/$BIN_NAME-$VERSION.pkg"
pkgbuild \
    --root "$DIST_DIR/bin" \
    --identifier "$PKG_ID" \
    --version "$VERSION" \
    --install-location "/usr/local/bin" \
    "$PKG_PATH"

# Optionally sign the package (uncomment and set your identity)
# codesign --sign "Developer ID Installer: Your Name (TEAMID)" "$PKG_PATH"

echo "macOS .pkg and .dmg created in $DIST_DIR"
    
# Create a .dmg containing only the .pkg and documentation
DMG_STAGING="$DIST_DIR/dmg_staging"
mkdir -p "$DMG_STAGING"
cp "$PKG_PATH" "$DMG_STAGING/"
if [ -f "$PROJECT_ROOT/README.md" ]; then
    cp "$PROJECT_ROOT/README.md" "$DMG_STAGING/"
fi
if [ -f "$PROJECT_ROOT/LICENSE" ]; then
    cp "$PROJECT_ROOT/LICENSE" "$DMG_STAGING/"
fi

DMG_PATH="$DIST_DIR/$BIN_NAME-$VERSION-macos.dmg"
hdiutil create -volname "Vexy SVGO Installer" -srcfolder "$DMG_STAGING" -ov -format UDZO "$DMG_PATH"

# Output result
ls -lh "$PKG_PATH" "$DMG_PATH"
echo "macOS .pkg and .dmg created in $DIST_DIR"
