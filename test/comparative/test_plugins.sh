#!/bin/bash
# Plugin-specific test runner for SVGO vs Vexy SVGO comparison
# Tests individual plugins against specific test cases

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
COMPARE_SCRIPT="$SCRIPT_DIR/compare_outputs.sh"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Common SVGO plugins that should be tested
COMMON_PLUGINS=(
    "cleanupAttrs"
    "removeDoctype"
    "removeXMLProcInst"
    "removeComments"
    "removeMetadata"
    "removeTitle"
    "removeDesc"
    "removeUselessDefs"
    "removeEditorsNSData"
    "removeEmptyAttrs"
    "removeHiddenElems"
    "removeEmptyText"
    "removeEmptyContainers"
    "removeViewBox"
    "cleanupEnableBackground"
    "convertStyleToAttrs"
    "convertColors"
    "convertPathData"
    "convertTransform"
    "removeUnknownsAndDefaults"
    "removeNonInheritableGroupAttrs"
    "removeUselessStrokeAndFill"
    "removeUnusedNS"
    "cleanupIDs"
    "cleanupNumericValues"
    "moveElemsAttrsToGroup"
    "moveGroupAttrsToElems"
    "collapseGroups"
    "removeRasterImages"
    "mergePaths"
    "convertShapeToPath"
    "sortAttrs"
    "removeDimensions"
    "removeStyleElement"
    "removeScriptElement"
)

log() {
    echo -e "${BLUE}[$(date '+%H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

info() {
    echo -e "${PURPLE}[INFO]${NC} $1"
}

usage() {
    echo "Usage: $0 [OPTIONS] [PLUGIN]"
    echo ""
    echo "Test individual SVGO/Vexy SVGO plugins"
    echo ""
    echo "OPTIONS:"
    echo "  -h, --help          Show this help message"
    echo "  -l, --list          List available plugins"
    echo "  -a, --all           Test all common plugins"
    echo "  -f, --file FILE     Test specific file (default: use test samples)"
    echo "  -v, --verbose       Verbose output"
    echo "  --build             Build Vexy SVGO before running tests"
    echo "  --quick             Quick test (first 5 plugins only)"
    echo ""
    echo "EXAMPLES:"
    echo "  $0 cleanupAttrs"
    echo "  $0 --all --quick"
    echo "  $0 --file mytest.svg removeComments"
    echo "  $0 --list"
}

# List available plugins
list_plugins() {
    echo "Available plugins for testing:"
    echo "================================"
    for plugin in "${COMMON_PLUGINS[@]}"; do
        echo "  - $plugin"
    done
    echo ""
    echo "Usage: $0 [OPTIONS] PLUGIN_NAME"
}

# Get test files for a plugin
get_test_files() {
    local plugin="$1"
    local test_files=()
    
    # Look for plugin-specific test files
    local plugin_test_dir="$ROOT_DIR/testdata/plugins/$plugin"
    if [[ -d "$plugin_test_dir" ]]; then
        while IFS= read -r -d '' file; do
            test_files+=("$file")
        done < <(find "$plugin_test_dir" -name "*.svg" -print0)
    fi
    
    # If no plugin-specific files, use general test files
    if [[ ${#test_files[@]} -eq 0 ]]; then
        local general_test_dirs=(
            "$ROOT_DIR/testdata"
            "$ROOT_DIR/test/samples"
        )
        
        for dir in "${general_test_dirs[@]}"; do
            if [[ -d "$dir" ]]; then
                while IFS= read -r -d '' file; do
                    test_files+=("$file")
                done < <(find "$dir" -name "*.svg" -print0 | head -10)
                break
            fi
        done
    fi
    
    # Fallback to any SVG files in the root
    if [[ ${#test_files[@]} -eq 0 ]]; then
        while IFS= read -r -d '' file; do
            test_files+=("$file")
        done < <(find "$ROOT_DIR" -maxdepth 1 -name "*.svg" -print0)
    fi
    
    printf '%s\n' "${test_files[@]}"
}

# Test a single plugin
test_plugin() {
    local plugin="$1"
    local test_file="$2"
    local verbose="$3"
    
    log "Testing plugin: $plugin"
    
    if [[ -n "$test_file" ]]; then
        local test_files=("$test_file")
    else
        local test_files
        readarray -t test_files < <(get_test_files "$plugin")
    fi
    
    if [[ ${#test_files[@]} -eq 0 ]]; then
        warning "No test files found for plugin $plugin"
        return 1
    fi
    
    local plugin_results_dir="$ROOT_DIR/test/results/plugins/$plugin"
    mkdir -p "$plugin_results_dir"
    
    local passed=0
    local failed=0
    local identical=0
    local different=0
    
    # Test each file with this plugin
    for file in "${test_files[@]}"; do
        if [[ ! -f "$file" ]]; then
            continue
        fi
        
        local basename
        basename=$(basename "$file" .svg)
        local test_output="$plugin_results_dir/${basename}_test.txt"
        
        info "  Testing: $basename"
        
        # Run comparison
        local cmd="$COMPARE_SCRIPT --plugin '$plugin' --stats"
        if [[ "$verbose" == "true" ]]; then
            cmd="$cmd --verbose"
        fi
        cmd="$cmd '$file'"
        
        if eval "$cmd" > "$test_output" 2>&1; then
            success "    ✓ Identical outputs"
            ((passed++))
            ((identical++))
        else
            local exit_code=$?
            if [[ $exit_code -eq 1 ]]; then
                warning "    ≠ Outputs differ"
                ((passed++))
                ((different++))
                
                # Check if Vexy SVGO is better
                if grep -q "Vexy SVGO output is.*bytes smaller" "$test_output"; then
                    info "      → Vexy SVGO improvement detected"
                fi
            else
                error "    ✗ Test failed"
                ((failed++))
            fi
        fi
    done
    
    # Generate plugin report
    local report_file="$plugin_results_dir/plugin_report.md"
    cat > "$report_file" << EOF
# Plugin Test Report: $plugin

**Generated:** $(date)  
**Test Files:** ${#test_files[@]}

## Summary

| Metric | Count | Percentage |
|--------|--------|------------|
| Total Tests | $((passed + failed)) | 100% |
| Passed Tests | $passed | $(echo "scale=1; $passed * 100 / (passed + failed)" | bc -l)% |
| Failed Tests | $failed | $(echo "scale=1; $failed * 100 / (passed + failed)" | bc -l)% |
| Identical Outputs | $identical | $(echo "scale=1; $identical * 100 / (passed + failed)" | bc -l)% |
| Different Outputs | $different | $(echo "scale=1; $different * 100 / (passed + failed)" | bc -l)% |

## Test Files

EOF
    
    for file in "${test_files[@]}"; do
        if [[ -f "$file" ]]; then
            echo "- \`$file\`" >> "$report_file"
        fi
    done
    
    echo ""
    echo "Plugin: $plugin"
    echo "  Tests: $((passed + failed))"
    echo "  Passed: $passed"
    echo "  Failed: $failed"
    echo "  Identical: $identical"
    echo "  Different: $different"
    echo "  Report: $report_file"
    
    return $((failed > 0 ? 1 : 0))
}

# Test all plugins
test_all_plugins() {
    local test_file="$1"
    local verbose="$2"
    local quick="$3"
    
    log "Testing all common plugins..."
    
    local plugins_to_test=("${COMMON_PLUGINS[@]}")
    if [[ "$quick" == "true" ]]; then
        plugins_to_test=("${COMMON_PLUGINS[@]:0:5}")
        warning "Quick mode: testing first 5 plugins only"
    fi
    
    local total_plugins=${#plugins_to_test[@]}
    local passed_plugins=0
    local failed_plugins=0
    
    # Create summary results directory
    local summary_dir="$ROOT_DIR/test/results/plugin_summary_$(date '+%Y%m%d_%H%M%S')"
    mkdir -p "$summary_dir"
    
    echo "# Plugin Test Summary" > "$summary_dir/summary.md"
    echo "" >> "$summary_dir/summary.md"
    echo "**Generated:** $(date)" >> "$summary_dir/summary.md"
    echo "**Plugins Tested:** $total_plugins" >> "$summary_dir/summary.md"
    echo "" >> "$summary_dir/summary.md"
    echo "## Results" >> "$summary_dir/summary.md"
    echo "" >> "$summary_dir/summary.md"
    
    # Test each plugin
    for plugin in "${plugins_to_test[@]}"; do
        echo "----------------------------------------"
        if test_plugin "$plugin" "$test_file" "$verbose"; then
            ((passed_plugins++))
            echo "- ✅ **$plugin** - All tests passed" >> "$summary_dir/summary.md"
        else
            ((failed_plugins++))
            echo "- ❌ **$plugin** - Some tests failed" >> "$summary_dir/summary.md"
        fi
    done
    
    # Final summary
    echo "========================================"
    echo "         PLUGIN TEST SUMMARY"
    echo "========================================"
    echo "Total Plugins: $total_plugins"
    echo "Passed: $passed_plugins"
    echo "Failed: $failed_plugins"
    echo "Success Rate: $(echo "scale=1; $passed_plugins * 100 / $total_plugins" | bc -l)%"
    echo "Summary: $summary_dir/summary.md"
    echo "========================================"
    
    return $((failed_plugins > 0 ? 1 : 0))
}

# Main execution
main() {
    local plugin=""
    local test_file=""
    local verbose=false
    local build_vexy_svgo=false
    local quick=false
    local list_plugins_flag=false
    local test_all=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                usage
                exit 0
                ;;
            -l|--list)
                list_plugins_flag=true
                shift
                ;;
            -a|--all)
                test_all=true
                shift
                ;;
            -f|--file)
                test_file="$2"
                shift 2
                ;;
            -v|--verbose)
                verbose=true
                shift
                ;;
            --build)
                build_vexy_svgo=true
                shift
                ;;
            --quick)
                quick=true
                shift
                ;;
            -*)
                error "Unknown option: $1"
                usage
                exit 1
                ;;
            *)
                if [[ -z "$plugin" ]]; then
                    plugin="$1"
                else
                    error "Multiple plugins specified"
                    usage
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # Handle list plugins request
    if [[ "$list_plugins_flag" == "true" ]]; then
        list_plugins
        exit 0
    fi
    
    # Validate arguments
    if [[ "$test_all" == "false" && -z "$plugin" ]]; then
        error "No plugin specified. Use --all to test all plugins or specify a plugin name."
        usage
        exit 1
    fi
    
    # Check test file exists
    if [[ -n "$test_file" && ! -f "$test_file" ]]; then
        error "Test file not found: $test_file"
        exit 1
    fi
    
    # Build Vexy SVGO if requested
    if [[ "$build_vexy_svgo" == "true" ]]; then
        log "Building Vexy SVGO..."
        cd "$ROOT_DIR"
        cargo build --release
        cd - > /dev/null
    fi
    
    # Create results directory
    mkdir -p "$ROOT_DIR/test/results/plugins"
    
    # Run tests
    if [[ "$test_all" == "true" ]]; then
        test_all_plugins "$test_file" "$verbose" "$quick"
    else
        test_plugin "$plugin" "$test_file" "$verbose"
    fi
}

# Run main function
main "$@"
