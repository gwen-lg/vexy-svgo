#!/bin/bash
# Vexy SVGO Release Script
# Usage: ./release.sh [version]
# Example: ./release.sh 2.1.0
# If no version is provided, automatically increments from the last git tag

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

# Function to increment version number
increment_version() {
    local version=$1
    local part=${2:-patch}  # default to patch
    
    # Remove 'v' prefix if present
    version=${version#v}
    
    # Split version into parts
    IFS='.' read -ra PARTS <<< "$version"
    local major=${PARTS[0]}
    local minor=${PARTS[1]}
    local patch=${PARTS[2]}
    
    case $part in
        major)
            ((major++))
            minor=0
            patch=0
            ;;
        minor)
            ((minor++))
            patch=0
            ;;
        patch|*)
            ((patch++))
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Check if version is provided
if [ $# -eq 0 ]; then
    # Get the latest git tag
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
    
    if [ "$LATEST_TAG" = "v0.0.0" ]; then
        log_warning "No previous tags found. Starting with version 0.0.1"
        VERSION="0.0.1"
    else
        # Increment the patch version by default
        VERSION=$(increment_version "$LATEST_TAG" patch)
        log_info "Auto-incrementing version from $LATEST_TAG to $VERSION"
    fi
else
    VERSION=$1
fi

VERSION_TAG="v$VERSION"

# Validate version format (semantic versioning)
if [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format. Use semantic versioning (e.g., 2.1.0)"
    exit 1
fi

log_info "Starting Vexy SVGO release process for version $VERSION"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "crates" ]; then
    log_error "Must be run from the root of the vexy-svgo project"
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

# Update version in Cargo.toml files
log_info "Updating version in workspace Cargo.toml"
# Only update the workspace.package.version line, not rust-version or dependency versions
# Use more specific regex to avoid matching rust-version = "..."
sed -i.bak "/^\[workspace.package\]/,/^\[/ s/^version = \"[0-9.]*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update versions in individual crate Cargo.toml files
log_info "Updating version in crate Cargo.toml files"
for crate_dir in crates/*/; do
    if [ -f "$crate_dir/Cargo.toml" ]; then
        crate_name=$(basename "$crate_dir")
        log_info "  - Updating $crate_name"
        # Update version in [package] section, but only if it's not using workspace inheritance
        sed -i.bak "/^\[package\]/,/^\[/ s/^version = \"[0-9.]*\"/version = \"$VERSION\"/" "$crate_dir/Cargo.toml"
        rm "$crate_dir/Cargo.toml.bak"
    fi
done

# Update workspace dependency versions
log_info "Updating workspace dependency versions"
sed -i.bak "s/vexy-svgo-core = { path = \"\.\/crates\/core\", version = \"[0-9.]*\" }/vexy-svgo-core = { path = \".\/crates\/core\", version = \"$VERSION\" }/" Cargo.toml
sed -i.bak "s/vexy-svgo-plugin-sdk = { path = \"\.\/crates\/plugin-sdk\", version = \"[0-9.]*\" }/vexy-svgo-plugin-sdk = { path = \".\/crates\/plugin-sdk\", version = \"$VERSION\" }/" Cargo.toml
sed -i.bak "s/vexy-svgo-test-utils = { path = \"\.\/crates\/test-utils\", version = \"[0-9.]*\" }/vexy-svgo-test-utils = { path = \".\/crates\/test-utils\", version = \"$VERSION\" }/" Cargo.toml
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
git add Cargo.toml Cargo.lock crates/*/Cargo.toml
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
log_info "Monitor the release at: https://github.com/vexyart/vexy-svgo/actions"
log_info "Release will be available at: https://github.com/vexyart/vexy-svgo/releases/tag/$VERSION_TAG"