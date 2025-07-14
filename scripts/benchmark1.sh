#!/bin/bash
# this_file: scripts/benchmark1.sh

# Basic benchmarking script for vexy-svgo vs svgo comparison
# Usage: ./scripts/benchmark1.sh [SVG_DIR] [ITERATIONS] [MIN_FILES]

set -euo pipefail

# Configuration
SVG_DIR="${1:-testdata}"
ITERATIONS="${2:-3}"
MIN_FILES="${3:-10}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Vexy SVGO vs SVGO Basic Benchmark ===${NC}"
echo -e "Test directory: ${SVG_DIR}"
echo -e "Iterations: ${ITERATIONS}"
echo -e "Minimum files required: ${MIN_FILES}"
echo

# Check prerequisites
if ! command -v bunx &> /dev/null; then
    echo -e "${RED}Error: bunx not found. Please install Bun.${NC}"
    exit 1
fi

if [ ! -f "./target/release/vexy_svgo" ]; then
    echo -e "${RED}Error: vexy_svgo binary not found. Run 'cargo build --release' first.${NC}"
    exit 1
fi

# Find SVG files
SVG_FILES=$(find "$SVG_DIR" -name "*.svg" -type f 2>/dev/null || true)
FILE_COUNT=$(echo "$SVG_FILES" | grep -c . || echo 0)

if [ "$FILE_COUNT" -lt "$MIN_FILES" ]; then
    echo -e "${RED}Error: Found only $FILE_COUNT SVG files, need at least $MIN_FILES${NC}"
    exit 1
fi

echo -e "${GREEN}Found $FILE_COUNT SVG files${NC}"
echo

# Function to benchmark a command
benchmark_tool() {
    local tool_name=$1
    local tool_cmd=$2
    local total_time=0
    local successful=0
    local failed=0
    
    echo -e "${YELLOW}Benchmarking $tool_name...${NC}"
    
    for i in $(seq 1 $ITERATIONS); do
        echo -n "  Iteration $i/$ITERATIONS... "
        local start_time=$(date +%s.%N)
        
        # Process all files
        local iter_success=0
        local iter_fail=0
        while IFS= read -r svg_file; do
            if [ -z "$svg_file" ]; then continue; fi
            
            if $tool_cmd "$svg_file" -o /dev/null 2>/dev/null; then
                ((iter_success++))
            else
                ((iter_fail++))
            fi
        done <<< "$SVG_FILES"
        
        local end_time=$(date +%s.%N)
        local elapsed=$(echo "$end_time - $start_time" | bc)
        total_time=$(echo "$total_time + $elapsed" | bc)
        
        # Update totals (only count from first iteration to avoid double counting)
        if [ $i -eq 1 ]; then
            successful=$iter_success
            failed=$iter_fail
        fi
        
        echo "done (${elapsed}s)"
    done
    
    local avg_time=$(echo "scale=3; $total_time / $ITERATIONS" | bc)
    local per_file_avg=$(echo "scale=4; $avg_time / $FILE_COUNT" | bc)
    
    echo -e "  ${GREEN}Average time: ${avg_time}s${NC}"
    echo -e "  ${GREEN}Per file: ${per_file_avg}s${NC}"
    echo -e "  ${GREEN}Successful: $successful, Failed: $failed${NC}"
    echo
    
    echo "$tool_name,$avg_time,$per_file_avg,$successful,$failed"
}

# Create results file
RESULTS_FILE="benchmark_results_$(date +%Y%m%d_%H%M%S).csv"
echo "Tool,Total_Time_Avg,Per_File_Avg,Successful,Failed" > "$RESULTS_FILE"

# Benchmark SVGO with Bun
SVGO_RESULT=$(benchmark_tool "SVGO (Bun)" "bunx --bun svgo")
echo "$SVGO_RESULT" >> "$RESULTS_FILE"

# Benchmark Vexy SVGO
VEXY_RESULT=$(benchmark_tool "Vexy SVGO" "./target/release/vexy_svgo")
echo "$VEXY_RESULT" >> "$RESULTS_FILE"

# Calculate speedup
SVGO_TIME=$(echo "$SVGO_RESULT" | cut -d',' -f2)
VEXY_TIME=$(echo "$VEXY_RESULT" | cut -d',' -f2)
SPEEDUP=$(echo "scale=2; $SVGO_TIME / $VEXY_TIME" | bc)

echo -e "${BLUE}=== Summary ===${NC}"
echo -e "SVGO (Bun) average time: ${SVGO_TIME}s"
echo -e "Vexy SVGO average time: ${VEXY_TIME}s"
echo -e "${GREEN}Speedup: ${SPEEDUP}x${NC}"
echo
echo -e "Results saved to: ${RESULTS_FILE}"