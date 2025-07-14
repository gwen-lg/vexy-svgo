#!/bin/bash
# this_file: scripts/benchmark-comprehensive.sh

# Comprehensive benchmarking script for vexy-svgo with multi-tool comparison
# Usage: ./scripts/benchmark-comprehensive.sh [SVG_DIR] [ITERATIONS] [MIN_FILES] [OUTPUT_FORMAT] [ENABLE_MEMORY] [ENABLE_SIZE_ANALYSIS]

set -euo pipefail

# Configuration
SVG_DIR="${1:-testdata}"
ITERATIONS="${2:-3}"
MIN_FILES="${3:-10}"
OUTPUT_FORMAT="${4:-both}" # csv, json, or both
ENABLE_MEMORY="${5:-true}"
ENABLE_SIZE_ANALYSIS="${6:-true}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Tool configurations
declare -A TOOLS=(
    ["svgo_bun_home"]="$HOME/.bun/bin/bun --bun $(which svgo 2>/dev/null || echo 'svgo') -i"
    ["svgo_bunx"]="bunx --bun svgo -i"
    ["svgo_npx"]="npx svgo -i"
    ["vexy_svgo"]="./target/release/vexy_svgo"
)

# Tool display names
declare -A TOOL_NAMES=(
    ["svgo_bun_home"]="SVGO (Bun Home)"
    ["svgo_bunx"]="SVGO (bunx)"
    ["svgo_npx"]="SVGO (npx)"
    ["vexy_svgo"]="Vexy SVGO"
)

echo -e "${BLUE}=== Vexy SVGO Comprehensive Benchmark ===${NC}"
echo -e "Test directory: ${SVG_DIR}"
echo -e "Iterations: ${ITERATIONS}"
echo -e "Minimum files required: ${MIN_FILES}"
echo -e "Output format: ${OUTPUT_FORMAT}"
echo -e "Memory monitoring: ${ENABLE_MEMORY}"
echo -e "Size analysis: ${ENABLE_SIZE_ANALYSIS}"
echo

# Check if memory monitoring is available
HAS_TIME_CMD=false
if [ "$ENABLE_MEMORY" = "true" ] && command -v /usr/bin/time &> /dev/null; then
    HAS_TIME_CMD=true
    echo -e "${GREEN}Memory monitoring enabled${NC}"
else
    ENABLE_MEMORY=false
    echo -e "${YELLOW}Memory monitoring disabled (time command not found)${NC}"
fi

# Check prerequisites
echo -e "\n${CYAN}Checking prerequisites...${NC}"

for tool in "${!TOOLS[@]}"; do
    cmd="${TOOLS[$tool]}"
    name="${TOOL_NAMES[$tool]}"
    
    # Extract the main command
    main_cmd=$(echo "$cmd" | awk '{print $1}')
    
    if [ "$tool" = "vexy_svgo" ]; then
        if [ -f "$main_cmd" ]; then
            echo -e "  ✓ $name found"
        else
            echo -e "  ${RED}✗ $name not found (run 'cargo build --release')${NC}"
            unset TOOLS[$tool]
        fi
    elif command -v "$main_cmd" &> /dev/null; then
        echo -e "  ✓ $name available"
    else
        echo -e "  ${YELLOW}⚠ $name not available${NC}"
        unset TOOLS[$tool]
    fi
done

if [ ${#TOOLS[@]} -eq 0 ]; then
    echo -e "${RED}Error: No tools available for benchmarking${NC}"
    exit 1
fi

# Find SVG files
echo -e "\n${CYAN}Scanning for SVG files...${NC}"
SVG_FILES=$(find "$SVG_DIR" -name "*.svg" -type f 2>/dev/null | sort || true)
FILE_COUNT=$(echo "$SVG_FILES" | grep -c . || echo 0)

if [ "$FILE_COUNT" -lt "$MIN_FILES" ]; then
    echo -e "${RED}Error: Found only $FILE_COUNT SVG files, need at least $MIN_FILES${NC}"
    exit 1
fi

echo -e "${GREEN}Found $FILE_COUNT SVG files${NC}"

# Calculate total size of test files
if [ "$ENABLE_SIZE_ANALYSIS" = "true" ]; then
    TOTAL_ORIGINAL_SIZE=0
    while IFS= read -r svg_file; do
        if [ -z "$svg_file" ] || [ ! -f "$svg_file" ]; then continue; fi
        size=$(stat -f%z "$svg_file" 2>/dev/null || stat -c%s "$svg_file" 2>/dev/null || echo 0)
        TOTAL_ORIGINAL_SIZE=$((TOTAL_ORIGINAL_SIZE + size))
    done <<< "$SVG_FILES"
    echo -e "Total test data size: $(echo "scale=2; $TOTAL_ORIGINAL_SIZE / 1024" | bc)KB"
fi

# Initialize results storage
declare -A RESULTS
declare -A MEMORY_USAGE
declare -A SIZE_REDUCTION

# Function to measure memory usage
measure_memory() {
    local cmd=$1
    local output_file=$2
    
    if [ "$HAS_TIME_CMD" = "true" ]; then
        # Use time command to measure memory
        /usr/bin/time -l -o "$output_file" $cmd 2>&1 >/dev/null || true
        
        # Extract peak memory usage (macOS format)
        local mem_kb=$(grep "maximum resident set size" "$output_file" 2>/dev/null | awk '{print int($1/1024)}' || echo 0)
        
        # If macOS format didn't work, try Linux format
        if [ "$mem_kb" -eq 0 ]; then
            mem_kb=$(grep "Maximum resident set size" "$output_file" 2>/dev/null | awk '{print $6}' || echo 0)
        fi
        
        echo "$mem_kb"
    else
        echo "0"
    fi
}

# Function to benchmark a tool
benchmark_tool() {
    local tool_key=$1
    local tool_cmd="${TOOLS[$tool_key]}"
    local tool_name="${TOOL_NAMES[$tool_key]}"
    
    echo -e "\n${YELLOW}Benchmarking $tool_name...${NC}"
    
    local total_time=0
    local total_memory=0
    local successful=0
    local failed=0
    local total_optimized_size=0
    local temp_dir=$(mktemp -d)
    
    for i in $(seq 1 $ITERATIONS); do
        echo -n "  Iteration $i/$ITERATIONS... "
        local start_time=$(date +%s.%N 2>/dev/null || date +%s)
        local iter_memory=0
        local iter_success=0
        local iter_fail=0
        
        # Process all files
        while IFS= read -r svg_file; do
            if [ -z "$svg_file" ] || [ ! -f "$svg_file" ]; then continue; fi
            
            local output_file="$temp_dir/$(basename "$svg_file")"
            local time_file="$temp_dir/time.txt"
            
            # Run the tool with memory measurement
            if [ "$ENABLE_MEMORY" = "true" ] && [ "$HAS_TIME_CMD" = "true" ]; then
                if /usr/bin/time -l -o "$time_file" $tool_cmd "$svg_file" -o "$output_file" 2>/dev/null; then
                    ((iter_success++))
                    
                    # Get memory usage
                    local mem_kb=$(grep "maximum resident set size" "$time_file" 2>/dev/null | awk '{print int($1/1024)}' || echo 0)
                    if [ "$mem_kb" -eq 0 ]; then
                        mem_kb=$(grep "Maximum resident set size" "$time_file" 2>/dev/null | awk '{print $6}' || echo 0)
                    fi
                    iter_memory=$((iter_memory + mem_kb))
                    
                    # Get output size
                    if [ "$ENABLE_SIZE_ANALYSIS" = "true" ] && [ -f "$output_file" ]; then
                        local out_size=$(stat -f%z "$output_file" 2>/dev/null || stat -c%s "$output_file" 2>/dev/null || echo 0)
                        if [ $i -eq 1 ]; then  # Only count size on first iteration
                            total_optimized_size=$((total_optimized_size + out_size))
                        fi
                    fi
                else
                    ((iter_fail++))
                fi
            else
                if $tool_cmd "$svg_file" -o "$output_file" 2>/dev/null; then
                    ((iter_success++))
                    
                    # Get output size
                    if [ "$ENABLE_SIZE_ANALYSIS" = "true" ] && [ -f "$output_file" ] && [ $i -eq 1 ]; then
                        local out_size=$(stat -f%z "$output_file" 2>/dev/null || stat -c%s "$output_file" 2>/dev/null || echo 0)
                        total_optimized_size=$((total_optimized_size + out_size))
                    fi
                else
                    ((iter_fail++))
                fi
            fi
            
            rm -f "$output_file" "$time_file"
        done <<< "$SVG_FILES"
        
        local end_time=$(date +%s.%N 2>/dev/null || date +%s)
        local elapsed=$(echo "$end_time - $start_time" | bc 2>/dev/null || echo "0")
        total_time=$(echo "$total_time + $elapsed" | bc)
        
        if [ "$ENABLE_MEMORY" = "true" ]; then
            total_memory=$((total_memory + iter_memory))
        fi
        
        # Update totals (only count from first iteration)
        if [ $i -eq 1 ]; then
            successful=$iter_success
            failed=$iter_fail
        fi
        
        echo "done (${elapsed}s)"
    done
    
    rm -rf "$temp_dir"
    
    # Calculate averages
    local avg_time=$(echo "scale=3; $total_time / $ITERATIONS" | bc)
    local per_file_avg=$(echo "scale=4; $avg_time / $FILE_COUNT" | bc)
    local avg_memory=0
    if [ "$ENABLE_MEMORY" = "true" ] && [ $total_memory -gt 0 ]; then
        avg_memory=$(echo "scale=0; $total_memory / $ITERATIONS / $successful" | bc)
    fi
    
    # Store results
    RESULTS["${tool_key}_time"]=$avg_time
    RESULTS["${tool_key}_per_file"]=$per_file_avg
    RESULTS["${tool_key}_success"]=$successful
    RESULTS["${tool_key}_fail"]=$failed
    MEMORY_USAGE[$tool_key]=$avg_memory
    SIZE_REDUCTION[$tool_key]=$total_optimized_size
    
    echo -e "  ${GREEN}Average time: ${avg_time}s${NC}"
    echo -e "  ${GREEN}Per file: ${per_file_avg}s${NC}"
    echo -e "  ${GREEN}Successful: $successful, Failed: $failed${NC}"
    if [ "$ENABLE_MEMORY" = "true" ] && [ $avg_memory -gt 0 ]; then
        echo -e "  ${GREEN}Avg memory: ${avg_memory}KB${NC}"
    fi
    if [ "$ENABLE_SIZE_ANALYSIS" = "true" ] && [ $total_optimized_size -gt 0 ]; then
        local reduction_pct=$(echo "scale=1; 100 - ($total_optimized_size * 100 / $TOTAL_ORIGINAL_SIZE)" | bc)
        echo -e "  ${GREEN}Size reduction: ${reduction_pct}%${NC}"
    fi
}

# Run benchmarks
echo -e "\n${CYAN}Running benchmarks...${NC}"
for tool in "${!TOOLS[@]}"; do
    benchmark_tool "$tool"
done

# Generate output files
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# CSV output
if [ "$OUTPUT_FORMAT" = "csv" ] || [ "$OUTPUT_FORMAT" = "both" ]; then
    CSV_FILE="benchmark_results_${TIMESTAMP}.csv"
    echo "Tool,Total_Time_Avg,Per_File_Avg,Successful_Files,Failed_Files,Memory_KB_Avg,Original_Size_Total,Optimized_Size_Total,Compression_Ratio" > "$CSV_FILE"
    
    for tool in "${!TOOLS[@]}"; do
        local compression_ratio=0
        if [ "$ENABLE_SIZE_ANALYSIS" = "true" ] && [ "${SIZE_REDUCTION[$tool]}" -gt 0 ]; then
            compression_ratio=$(echo "scale=1; 100 - (${SIZE_REDUCTION[$tool]} * 100 / $TOTAL_ORIGINAL_SIZE)" | bc)
        fi
        
        echo "${TOOL_NAMES[$tool]},${RESULTS[${tool}_time]},${RESULTS[${tool}_per_file]},${RESULTS[${tool}_success]},${RESULTS[${tool}_fail]},${MEMORY_USAGE[$tool]},$TOTAL_ORIGINAL_SIZE,${SIZE_REDUCTION[$tool]},$compression_ratio" >> "$CSV_FILE"
    done
    
    echo -e "\n${GREEN}CSV results saved to: $CSV_FILE${NC}"
fi

# JSON output
if [ "$OUTPUT_FORMAT" = "json" ] || [ "$OUTPUT_FORMAT" = "both" ]; then
    JSON_FILE="benchmark_results_${TIMESTAMP}.json"
    
    # Start JSON structure
    cat > "$JSON_FILE" << EOF
{
  "metadata": {
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "system": {
      "os": "$(uname -s)",
      "arch": "$(uname -m)",
      "cpu_count": $(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 1),
      "memory_mb": $(echo "$(sysctl -n hw.memsize 2>/dev/null || free -b | grep Mem | awk '{print $2}' || echo 0) / 1024 / 1024" | bc)
    },
    "test_config": {
      "svg_directory": "$SVG_DIR",
      "file_count": $FILE_COUNT,
      "iterations": $ITERATIONS,
      "memory_monitoring": $ENABLE_MEMORY,
      "size_analysis": $ENABLE_SIZE_ANALYSIS,
      "total_original_size": ${TOTAL_ORIGINAL_SIZE:-0}
    }
  },
  "tools": {
EOF
    
    # Add tool results
    first=true
    for tool in "${!TOOLS[@]}"; do
        if [ "$first" = true ]; then
            first=false
        else
            echo "," >> "$JSON_FILE"
        fi
        
        local compression_ratio=0
        if [ "$ENABLE_SIZE_ANALYSIS" = "true" ] && [ "${SIZE_REDUCTION[$tool]}" -gt 0 ]; then
            compression_ratio=$(echo "scale=1; 100 - (${SIZE_REDUCTION[$tool]} * 100 / $TOTAL_ORIGINAL_SIZE)" | bc)
        fi
        
        cat >> "$JSON_FILE" << EOF
    "$tool": {
      "name": "${TOOL_NAMES[$tool]}",
      "command": "${TOOLS[$tool]}",
      "results": {
        "total_time_avg": ${RESULTS[${tool}_time]},
        "per_file_avg": ${RESULTS[${tool}_per_file]},
        "successful_files": ${RESULTS[${tool}_success]},
        "failed_files": ${RESULTS[${tool}_fail]},
        "memory_kb_avg": ${MEMORY_USAGE[$tool]},
        "optimized_size_total": ${SIZE_REDUCTION[$tool]:-0},
        "compression_ratio": $compression_ratio
      }
    }
EOF
    done
    
    # Close JSON structure
    echo -e "\n  }\n}" >> "$JSON_FILE"
    
    echo -e "${GREEN}JSON results saved to: $JSON_FILE${NC}"
    
    # Copy to docs data directory for Jekyll integration
    if [ -d "docs_src/data" ]; then
        cp "$JSON_FILE" "docs_src/data/benchmarks.json"
        echo -e "${GREEN}Results copied to docs_src/data/benchmarks.json${NC}"
    fi
fi

# Display summary
echo -e "\n${BLUE}=== Benchmark Summary ===${NC}"

# Find reference tool (first SVGO variant)
REF_TOOL=""
for tool in "svgo_bun_home" "svgo_bunx" "svgo_npx"; do
    if [ -n "${RESULTS[${tool}_time]:-}" ]; then
        REF_TOOL=$tool
        break
    fi
done

if [ -n "$REF_TOOL" ]; then
    echo -e "\nPerformance comparison (vs ${TOOL_NAMES[$REF_TOOL]}):"
    REF_TIME=${RESULTS[${REF_TOOL}_time]}
    
    for tool in "${!TOOLS[@]}"; do
        if [ "$tool" != "$REF_TOOL" ]; then
            local speedup=$(echo "scale=2; $REF_TIME / ${RESULTS[${tool}_time]}" | bc)
            echo -e "  ${TOOL_NAMES[$tool]}: ${speedup}x speedup"
        fi
    done
fi

# Show best performer
BEST_TIME=999999
BEST_TOOL=""
for tool in "${!TOOLS[@]}"; do
    if (( $(echo "${RESULTS[${tool}_time]} < $BEST_TIME" | bc -l) )); then
        BEST_TIME=${RESULTS[${tool}_time]}
        BEST_TOOL=$tool
    fi
done

echo -e "\n${GREEN}Best performer: ${TOOL_NAMES[$BEST_TOOL]} (${BEST_TIME}s average)${NC}"

# Memory comparison
if [ "$ENABLE_MEMORY" = "true" ]; then
    echo -e "\nMemory usage comparison:"
    for tool in "${!TOOLS[@]}"; do
        if [ "${MEMORY_USAGE[$tool]}" -gt 0 ]; then
            echo -e "  ${TOOL_NAMES[$tool]}: ${MEMORY_USAGE[$tool]}KB average"
        fi
    done
fi

# Size reduction comparison
if [ "$ENABLE_SIZE_ANALYSIS" = "true" ]; then
    echo -e "\nSize reduction comparison:"
    for tool in "${!TOOLS[@]}"; do
        if [ "${SIZE_REDUCTION[$tool]:-0}" -gt 0 ]; then
            local reduction_pct=$(echo "scale=1; 100 - (${SIZE_REDUCTION[$tool]} * 100 / $TOTAL_ORIGINAL_SIZE)" | bc)
            echo -e "  ${TOOL_NAMES[$tool]}: ${reduction_pct}% reduction"
        fi
    done
fi

echo -e "\n${CYAN}Benchmark complete!${NC}"