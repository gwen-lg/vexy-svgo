#!/bin/bash
# VEXYSVGO Release Script
# Usage: ./release.sh [version]
# Example: ./release.sh 2.1.0

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

# Check if version is provided
if [ $# -eq 0 ]; then
    log_error "Version number required"
    echo "Usage: $0 <version>"
    echo "Example: $0 2.1.0"
    exit 1
fi

VERSION=$1
VERSION_TAG="v$VERSION"

# Validate version format (semantic versioning)
if [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format. Use semantic versioning (e.g., 2.1.0)"
    exit 1
fi

log_info "Starting VEXYSVGO release process for version $VERSION"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "crates" ]; then
    log_error "Must be run from the root of the vexy_svgo project"
    exit 1
fi

# Check if git is clean
if [ -n "$(git status --porcelain)" ]; then
    log_error "Working directory is not clean. Please commit all changes first."
    exit 1
fi

# Check if tag already exists
if git rev-parse "$VERSION_TAG" >/dev/null 2>&1; then
    log_error "Tag $VERSION_TAG already exists"
    exit 1
fi

# Update version in Cargo.toml
log_info "Updating version in Cargo.toml"
# Only update the workspace.package.version, not dependency versions
sed -i.bak "/^\[workspace.package\]/,/^\[/ s/version = \"[^\"]*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Run tests to ensure everything is working
log_info "Running tests"
if ! cargo test --workspace --release; then
    log_error "Tests failed"
    exit 1
fi

# Build all targets
log_info "Building all targets"
if ! ./scripts/build-all.sh; then
    log_error "Build failed"
    exit 1
fi

# Create release artifacts
log_info "Creating release artifacts"
if ! ./scripts/package-release.sh "$VERSION"; then
    log_error "Packaging failed"
    exit 1
fi

# Commit version change
log_info "Committing version change"
git add Cargo.toml Cargo.lock
git commit -m "Release version $VERSION"

# Create and push tag
log_info "Creating git tag $VERSION_TAG"
git tag -a "$VERSION_TAG" -m "Release version $VERSION"

# Push changes and tag
log_info "Pushing changes to remote"
git push origin main
git push origin "$VERSION_TAG"

log_success "Release $VERSION initiated successfully!"
log_info "GitHub Actions will automatically:"
log_info "  - Build binaries for all platforms"
log_info "  - Create release artifacts"
log_info "  - Upload to GitHub Releases"
log_info "  - Publish to package managers"

echo
log_info "Monitor the release at: https://github.com/twardoch/vexy_svgo/actions"
log_info "Release will be available at: https://github.com/twardoch/vexy_svgo/releases/tag/$VERSION_TAG"