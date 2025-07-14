#!/bin/bash
# this_file: scripts/profile-memory.sh

# Memory profiling script for vexy-svgo
# Usage: ./scripts/profile-memory.sh [SVG_FILE] [OUTPUT_DIR]

set -euo pipefail

# Configuration
SVG_FILE="${1:-}"
OUTPUT_DIR="${2:-./memory-profile}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print usage
usage() {
    echo "Usage: $0 <svg_file> [output_dir]"
    echo ""
    echo "Profile memory usage of vexy-svgo when processing large SVG files"
    echo ""
    echo "Arguments:"
    echo "  svg_file    Path to SVG file to process"
    echo "  output_dir  Directory for profiling output (default: ./memory-profile)"
    echo ""
    echo "Examples:"
    echo "  $0 large.svg"
    echo "  $0 huge.svg ./profiles"
    exit 1
}

# Check arguments
if [ -z "$SVG_FILE" ] || [ ! -f "$SVG_FILE" ]; then
    echo -e "${RED}Error: SVG file not found or not specified${NC}"
    usage
fi

# Check prerequisites
if ! command -v /usr/bin/time &> /dev/null; then
    echo -e "${RED}Error: 'time' command not found${NC}"
    echo "On macOS, the built-in time command should be available"
    echo "On Linux, install with: sudo apt-get install time"
    exit 1
fi

if [ ! -f "./target/release/vexy_svgo" ]; then
    echo -e "${RED}Error: vexy_svgo binary not found${NC}"
    echo "Run 'cargo build --release' first"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}=== Vexy SVGO Memory Profiling ===${NC}"
echo -e "SVG file: ${SVG_FILE}"
echo -e "File size: $(ls -lh "$SVG_FILE" | awk '{print $5}')"
echo -e "Output directory: ${OUTPUT_DIR}"
echo ""

# Generate test files of different sizes if needed
generate_test_files() {
    local base_svg=$1
    local output_dir=$2
    
    echo -e "${CYAN}Generating test files of different sizes...${NC}"
    
    # Read the base SVG
    local svg_content=$(cat "$base_svg")
    local svg_start=$(echo "$svg_content" | grep -n "<svg" | head -1 | cut -d: -f1)
    local svg_end=$(echo "$svg_content" | grep -n "</svg>" | tail -1 | cut -d: -f1)
    
    # Extract header and footer
    local header=$(echo "$svg_content" | head -n $svg_start)
    local footer=$(echo "$svg_content" | tail -n +$svg_end)
    local body=$(echo "$svg_content" | sed -n "$((svg_start+1)),$((svg_end-1))p")
    
    # Generate files with repeated content
    for multiplier in 1 10 50 100; do
        local output_file="$output_dir/test_${multiplier}x.svg"
        echo "$header" > "$output_file"
        
        for i in $(seq 1 $multiplier); do
            echo "$body" >> "$output_file"
        done
        
        echo "$footer" >> "$output_file"
        
        echo -e "  Generated: $(basename "$output_file") - $(ls -lh "$output_file" | awk '{print $5}')"
    done
}

# Profile memory usage with time command
profile_with_time() {
    local svg_file=$1
    local label=$2
    local output_prefix="$OUTPUT_DIR/${label}_${TIMESTAMP}"
    
    echo -e "\n${YELLOW}Profiling: $label${NC}"
    echo -e "Input: $(basename "$svg_file") ($(ls -lh "$svg_file" | awk '{print $5}'))"
    
    # Run with time command (macOS format)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        /usr/bin/time -l ./target/release/vexy_svgo "$svg_file" -o "${output_prefix}_output.svg" \
            2> "${output_prefix}_time.txt" \
            > "${output_prefix}_stdout.txt"
        
        # Extract metrics from macOS time output
        local peak_memory=$(grep "maximum resident set size" "${output_prefix}_time.txt" | awk '{print $1}')
        local peak_memory_mb=$(echo "scale=2; $peak_memory / 1024 / 1024" | bc)
        local user_time=$(grep "user" "${output_prefix}_time.txt" | awk '{print $1}')
        local sys_time=$(grep "sys" "${output_prefix}_time.txt" | awk '{print $1}')
        
    else
        # Linux format
        /usr/bin/time -v ./target/release/vexy_svgo "$svg_file" -o "${output_prefix}_output.svg" \
            2> "${output_prefix}_time.txt" \
            > "${output_prefix}_stdout.txt"
        
        # Extract metrics from Linux time output
        local peak_memory=$(grep "Maximum resident set size" "${output_prefix}_time.txt" | awk '{print $6}')
        local peak_memory_mb=$(echo "scale=2; $peak_memory / 1024" | bc)
        local user_time=$(grep "User time" "${output_prefix}_time.txt" | awk '{print $4}')
        local sys_time=$(grep "System time" "${output_prefix}_time.txt" | awk '{print $4}')
    fi
    
    # Get file sizes
    local input_size=$(stat -f%z "$svg_file" 2>/dev/null || stat -c%s "$svg_file")
    local output_size=$(stat -f%z "${output_prefix}_output.svg" 2>/dev/null || stat -c%s "${output_prefix}_output.svg" || echo 0)
    
    # Calculate metrics
    local compression_ratio=0
    if [ $output_size -gt 0 ]; then
        compression_ratio=$(echo "scale=1; 100 - ($output_size * 100 / $input_size)" | bc)
    fi
    
    # Display results
    echo -e "  ${GREEN}Peak memory: ${peak_memory_mb} MB${NC}"
    echo -e "  ${GREEN}User time: ${user_time}s${NC}"
    echo -e "  ${GREEN}System time: ${sys_time}s${NC}"
    echo -e "  ${GREEN}Compression: ${compression_ratio}%${NC}"
    
    # Save summary
    cat > "${output_prefix}_summary.json" << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "input_file": "$svg_file",
  "input_size_bytes": $input_size,
  "output_size_bytes": $output_size,
  "compression_ratio": $compression_ratio,
  "peak_memory_mb": $peak_memory_mb,
  "user_time_seconds": $user_time,
  "system_time_seconds": $sys_time
}
EOF
}

# Profile with Rust's built-in allocator tracking
profile_with_allocator() {
    local svg_file=$1
    local label=$2
    local output_prefix="$OUTPUT_DIR/${label}_${TIMESTAMP}"
    
    echo -e "\n${YELLOW}Profiling with allocator tracking: $label${NC}"
    
    # Run with RUST_BACKTRACE for any panics
    RUST_BACKTRACE=1 ./target/release/vexy_svgo "$svg_file" \
        -o "${output_prefix}_alloc_output.svg" \
        --verbose \
        2> "${output_prefix}_alloc_stderr.txt" \
        > "${output_prefix}_alloc_stdout.txt"
}

# Generate memory usage graph (requires gnuplot)
generate_graph() {
    if ! command -v gnuplot &> /dev/null; then
        echo -e "${YELLOW}Skipping graph generation (gnuplot not found)${NC}"
        return
    fi
    
    echo -e "\n${CYAN}Generating memory usage graph...${NC}"
    
    # Create gnuplot script
    cat > "$OUTPUT_DIR/plot_memory.gnu" << 'EOF'
set terminal png size 800,600
set output "memory_usage.png"
set title "Vexy SVGO Memory Usage vs File Size"
set xlabel "Input File Size (MB)"
set ylabel "Peak Memory Usage (MB)"
set grid
plot "memory_data.txt" using 1:2 with linespoints title "Peak Memory"
EOF
    
    # Generate graph if we have data
    if [ -f "$OUTPUT_DIR/memory_data.txt" ]; then
        cd "$OUTPUT_DIR"
        gnuplot plot_memory.gnu
        echo -e "  ${GREEN}Graph saved to: $OUTPUT_DIR/memory_usage.png${NC}"
        cd - > /dev/null
    fi
}

# Main profiling workflow
main() {
    # Profile the provided file
    profile_with_time "$SVG_FILE" "single_file"
    
    # Generate test files if requested
    if [ "${GENERATE_TESTS:-false}" = "true" ]; then
        generate_test_files "$SVG_FILE" "$OUTPUT_DIR"
        
        # Profile each generated file
        for test_file in "$OUTPUT_DIR"/test_*.svg; do
            if [ -f "$test_file" ]; then
                label=$(basename "$test_file" .svg)
                profile_with_time "$test_file" "$label"
            fi
        done
    fi
    
    # Run with different configurations
    echo -e "\n${CYAN}Testing different configurations...${NC}"
    
    # Test with multipass
    echo -e "\n${YELLOW}With multipass enabled:${NC}"
    /usr/bin/time -l ./target/release/vexy_svgo "$SVG_FILE" \
        --multipass \
        -o "$OUTPUT_DIR/multipass_output.svg" \
        2> "$OUTPUT_DIR/multipass_time.txt" || true
    
    # Test with parallel processing
    if [[ $(stat -f%z "$SVG_FILE" 2>/dev/null || stat -c%s "$SVG_FILE") -gt 1048576 ]]; then
        echo -e "\n${YELLOW}With parallel processing:${NC}"
        /usr/bin/time -l ./target/release/vexy_svgo "$SVG_FILE" \
            --parallel 4 \
            -o "$OUTPUT_DIR/parallel_output.svg" \
            2> "$OUTPUT_DIR/parallel_time.txt" || true
    fi
    
    # Generate summary report
    echo -e "\n${CYAN}Generating summary report...${NC}"
    cat > "$OUTPUT_DIR/report.md" << EOF
# Memory Profiling Report

Generated: $(date)

## Test File
- Path: $SVG_FILE
- Size: $(ls -lh "$SVG_FILE" | awk '{print $5}')

## Results Summary

| Configuration | Peak Memory | Time | Compression |
|--------------|-------------|------|-------------|
EOF
    
    # Add results to report
    for summary in "$OUTPUT_DIR"/*_summary.json; do
        if [ -f "$summary" ]; then
            config=$(basename "$summary" | cut -d_ -f1)
            peak_mem=$(jq -r .peak_memory_mb "$summary")
            user_time=$(jq -r .user_time_seconds "$summary")
            compression=$(jq -r .compression_ratio "$summary")
            echo "| $config | ${peak_mem} MB | ${user_time}s | ${compression}% |" >> "$OUTPUT_DIR/report.md"
        fi
    done
    
    echo -e "\n${GREEN}✅ Profiling complete!${NC}"
    echo -e "Results saved to: ${OUTPUT_DIR}"
    echo -e "Report: ${OUTPUT_DIR}/report.md"
    
    # Generate graph
    generate_graph
}

# Run main function
main