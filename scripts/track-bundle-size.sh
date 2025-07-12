#!/bin/bash
# this_file: scripts/track-bundle-size.sh
# Track WASM bundle sizes and fail if they exceed limits

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Size limits in KB
LIMIT_WEB=350
LIMIT_NODE=350
LIMIT_BUNDLER=350
LIMIT_MINIMAL=200
LIMIT_FULL=500

# Function to get file size in KB
get_size_kb() {
    local file=$1
    if [ -f "$file" ]; then
        local size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file")
        echo $((size / 1024))
    else
        echo 0
    fi
}

# Function to check size against limit
check_size() {
    local name=$1
    local file=$2
    local limit=$3
    
    local size=$(get_size_kb "$file")
    
    if [ $size -eq 0 ]; then
        echo -e "${YELLOW}⚠️  $name: File not found${NC}"
        return 1
    elif [ $size -gt $limit ]; then
        echo -e "${RED}❌ $name: ${size}KB (limit: ${limit}KB) - EXCEEDS LIMIT${NC}"
        return 1
    else
        echo -e "${GREEN}✅ $name: ${size}KB (limit: ${limit}KB)${NC}"
        return 0
    fi
}

# Track bundle sizes
echo -e "${GREEN}WASM Bundle Size Report${NC}"
echo "========================"
date

echo -e "\nChecking bundle sizes...\n"

# Track if any check fails
failed=0

# Check each bundle
#!/usr/bin/env bash
set -euo pipefail

# This script tracks the bundle size of vexy-svgo WASM modules.

# --- Configuration ---
LIMIT_WEB=200
LIMIT_NODE=200
LIMIT_BUNDLER=200
LIMIT_MINIMAL=150
LIMIT_FULL=250

# --- Helper Functions ---
get_size_kb() {
    stat -f%z "$1" | awk '{print int($1/1024)}'
}

check_size() {
    local target_name=$1
    local file_path=$2
    local limit_kb=$3
    local size_kb=$(get_size_kb "$file_path")

    if [ $size_kb -gt $limit_kb ]; then
        echo -e "[\033[31mFAIL\033[0m] $target_name size ($size_kb KB) exceeds limit ($limit_kb KB)"
        return 1
    else
        echo -e "[\033[32mPASS\033[0m] $target_name size ($size_kb KB) is within limit ($limit_kb KB)"
        return 0
    fi
}

# --- Main Logic ---
failed=0

# Check each bundle
check_size "Web target" "pkg-web/vexy-svgo_bg.wasm" $LIMIT_WEB || failed=1
check_size "Node.js target" "pkg-node/vexy-svgo_bg.wasm" $LIMIT_NODE || failed=1
check_size "Bundler target" "pkg-bundler/vexy-svgo_bg.wasm" $LIMIT_BUNDLER || failed=1
check_size "Minimal build" "pkg-minimal/vexy-svgo_bg.wasm" $LIMIT_MINIMAL || failed=1
check_size "Full build" "pkg-full/vexy-svgo_bg.wasm" $LIMIT_FULL || failed=1

exit $failed


echo -e "\nGzipped sizes:"
for pkg in pkg-web pkg-node pkg-bundler pkg-minimal pkg-full; do
    if [ -f "$pkg/vexy_svgo_wasm_bg.wasm" ]; then
        gzip -c "$pkg/vexy_svgo_wasm_bg.wasm" > "$pkg/vexy_svgo_wasm_bg.wasm.gz"
        gzip_size=$(get_size_kb "$pkg/vexy_svgo_wasm_bg.wasm.gz")
        echo "  $pkg: ${gzip_size}KB (gzipped)"
        rm "$pkg/vexy_svgo_wasm_bg.wasm.gz"
    fi
done

# Generate markdown report
cat > wasm-bundle-size-report.md << EOF
# WASM Bundle Size Report

Generated: $(date)

## Bundle Sizes

| Target | Size (KB) | Limit (KB) | Status |
|--------|-----------|------------|--------|
| Web | $(get_size_kb "pkg-web/vexy_svgo_wasm_bg.wasm") | $LIMIT_WEB | $([ $(get_size_kb "pkg-web/vexy_svgo_wasm_bg.wasm") -le $LIMIT_WEB ] && echo "✅" || echo "❌") |
| Node.js | $(get_size_kb "pkg-node/vexy_svgo_wasm_bg.wasm") | $LIMIT_NODE | $([ $(get_size_kb "pkg-node/vexy_svgo_wasm_bg.wasm") -le $LIMIT_NODE ] && echo "✅" || echo "❌") |
| Bundler | $(get_size_kb "pkg-bundler/vexy_svgo_wasm_bg.wasm") | $LIMIT_BUNDLER | $([ $(get_size_kb "pkg-bundler/vexy_svgo_wasm_bg.wasm") -le $LIMIT_BUNDLER ] && echo "✅" || echo "❌") |
| Minimal | $(get_size_kb "pkg-minimal/vexy_svgo_wasm_bg.wasm") | $LIMIT_MINIMAL | $([ $(get_size_kb "pkg-minimal/vexy_svgo_wasm_bg.wasm") -le $LIMIT_MINIMAL ] && echo "✅" || echo "❌") |
| Full | $(get_size_kb "pkg-full/vexy_svgo_wasm_bg.wasm") | $LIMIT_FULL | $([ $(get_size_kb "pkg-full/vexy_svgo_wasm_bg.wasm") -le $LIMIT_FULL ] && echo "✅" || echo "❌") |

## Size History

Add this report to your CI artifacts to track size changes over time.
EOF

echo -e "\nBundle size report saved to wasm-bundle-size-report.md"

# Exit with error if any size limit was exceeded
if [ $failed -eq 1 ]; then
    echo -e "\n${RED}ERROR: One or more bundles exceed size limits!${NC}"
    echo "To fix this:"
    echo "  1. Review recent changes that might have increased bundle size"
    echo "  2. Consider moving features behind feature flags"
    echo "  3. Check for unnecessary dependencies"
    echo "  4. Update size limits in this script if the increase is justified"
    exit 1
else
    echo -e "\n${GREEN}All bundle sizes are within limits!${NC}"
fi