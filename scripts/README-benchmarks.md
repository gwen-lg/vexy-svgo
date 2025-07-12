# Vexy SVGO Benchmarking Tools

This directory contains comprehensive benchmarking tools for comparing Vexy SVGO performance against various SVGO implementations.

## Available Scripts

### 1. `benchmark1.sh` (Legacy)
Basic benchmarking script comparing `bunx --bun svgo` vs `./target/release/vexy_svgo`.

**Usage:**
```bash
./scripts/benchmark1.sh [SVG_DIR] [ITERATIONS] [MIN_FILES]
```

### 2. `benchmark-comprehensive.sh` (Recommended)
Advanced benchmarking tool with multi-tool comparison, memory monitoring, and Jekyll integration.

**Usage:**
```bash
./scripts/benchmark-comprehensive.sh [SVG_DIR] [ITERATIONS] [MIN_FILES] [OUTPUT_FORMAT] [ENABLE_MEMORY] [ENABLE_SIZE_ANALYSIS]
```

**Parameters:**
- `SVG_DIR`: Directory containing SVG test files (default: `testdata`)
- `ITERATIONS`: Number of benchmark runs for averaging (default: `3`)
- `MIN_FILES`: Minimum number of SVG files required (default: `10`)
- `OUTPUT_FORMAT`: Output format - `csv`, `json`, or `both` (default: `both`)
- `ENABLE_MEMORY`: Enable memory usage monitoring (default: `true`)
- `ENABLE_SIZE_ANALYSIS`: Enable file size analysis (default: `true`)

## Tool Comparison

The comprehensive benchmark compares Vexy SVGO against multiple SVGO implementations:

1. **`$HOME/.bun/bin/bun --bun $(which svgo)`** - Bun runtime with locally installed SVGO (reference)
2. **`bunx --bun svgo`** - Bun package runner
3. **`npx svgo`** - Node.js package runner  
4. **`./target/release/vexy_svgo`** - Our Rust implementation

## Metrics Collected

### Performance Metrics
- **Execution Time**: Total processing time and per-file average
- **Memory Usage**: Peak memory consumption during processing (requires `time` command)
- **Success Rate**: Percentage of files processed without errors

### Quality Metrics  
- **File Size Reduction**: Original vs optimized file sizes
- **Compression Ratio**: Percentage reduction in file size
- **Output Compatibility**: Success/failure analysis

### Statistical Analysis
- **Multiple Iterations**: Averages across multiple runs for reliability
- **Speedup Calculations**: Performance comparisons relative to reference tool
- **Error Analysis**: Detailed failure tracking and reporting

## Output Formats

### CSV Output
Traditional comma-separated values format compatible with spreadsheet applications.

**Example:**
```csv
Tool,Total_Time_Avg,Per_File_Avg,Successful_Files,Failed_Files,Memory_KB_Avg,Original_Size_Total,Optimized_Size_Total,Compression_Ratio
svgo_bun_home,2.456,0.0491,147,3,45672,1048576,574259,45.2
vexy_svgo,0.892,0.0178,149,1,12845,1048576,555894,47.0
```

### JSON Output
Structured JSON format with comprehensive metadata and detailed results.

**Features:**
- System information (OS, CPU, memory)
- Tool versions and availability status
- Test configuration parameters
- Summary and detailed per-file results
- Jekyll-compatible data structure

### Documentation Integration
Automatically generates `docs_src/data/benchmarks.json` for integration with MkDocs documentation site.

**Generated Pages:**
- Interactive charts using Chart.js
- Responsive comparison tables
- Performance analysis summaries
- Historical trend tracking

## Prerequisites

### Required Tools
- **Rust toolchain**: For building Vexy SVGO (`cargo build --release`)
- **Node.js/npm**: For SVGO installation and npx runner
- **Bun runtime**: For bun-based SVGO execution
- **bc**: Basic calculator for mathematical operations
- **time**: For memory usage monitoring (optional)

### Installation Commands

**macOS:**
```bash
# Install Bun
curl -fsSL https://bun.sh/install | bash

# Install SVGO globally
npm install -g svgo

# Install bc (if not available)
brew install bc

# Build Vexy SVGO
cargo build --release
```

**Linux (Ubuntu/Debian):**
```bash
# Install Bun
curl -fsSL https://bun.sh/install | bash

# Install Node.js and npm
sudo apt update
sudo apt install nodejs npm bc

# Install SVGO globally
npm install -g svgo

# Build Vexy SVGO
cargo build --release
```

## Test Data Requirements

### Directory Structure
```
testdata/
├── simple/
│   ├── circle.svg
│   ├── rect.svg
│   └── path.svg
├── complex/
│   ├── illustration.svg
│   ├── logo.svg
│   └── icon-set.svg
└── large/
    ├── detailed-map.svg
    └── complex-diagram.svg
```

### Recommended Test Files
- **Variety**: Include simple shapes, complex illustrations, and large detailed graphics
- **Sources**: Real-world SVG files from different sources and tools
- **Sizes**: Range from small icons (1-5KB) to large graphics (100KB+)
- **Complexity**: Mix of path-heavy, text-heavy, and mixed content SVGs

### Minimum Requirements
- At least 10 SVG files (configurable with `MIN_FILES` parameter)
- Files should be valid SVG format
- Avoid extremely large files (>10MB) for reasonable benchmark times

## Interpreting Results

### Performance Analysis
- **Speedup Factor**: Higher is better (e.g., 2.5x means 2.5 times faster)
- **Memory Usage**: Lower is better (measured in KB)
- **Success Rate**: Higher is better (percentage of files processed successfully)

### Quality Analysis
- **Compression Ratio**: Higher is generally better, but consider output quality
- **File Size Reduction**: Absolute bytes saved across all test files
- **Consistency**: Look for consistent performance across different file types

### Statistical Significance
- Multiple iterations provide confidence in results
- Look for consistent trends across different test datasets
- Consider system load and other factors that might affect timing

## Troubleshooting

### Common Issues

**Tool Not Found:**
```bash
# Check if tools are installed and accessible
which svgo
which bunx
ls -la ./target/release/vexy_svgo
```

**Permission Errors:**
```bash
# Make scripts executable
chmod +x ./scripts/benchmark-comprehensive.sh
chmod +x ./target/release/vexy_svgo
```

**Memory Monitoring Disabled:**
- Ensure `/usr/bin/time` is available
- On some systems, use `/usr/bin/time` instead of `time`
- Memory monitoring is automatically disabled if `time` command is not found

**Insufficient Test Files:**
```bash
# Check test directory contents
find testdata -name "*.svg" | wc -l

# Adjust minimum file requirement
./scripts/benchmark-comprehensive.sh testdata 3 5  # Require only 5 files
```

### Performance Considerations

**System Load:**
- Run benchmarks on idle system for consistent results
- Close unnecessary applications during benchmarking
- Consider system thermal throttling on laptops

**File System:**
- Use SSD storage for better I/O performance
- Ensure sufficient disk space for output files
- Consider tmpfs for temporary files on Linux

## Contributing

### Adding New Tools
Modify the `TOOLS` array in `benchmark-comprehensive.sh`:
```bash
declare -A TOOLS=(
    ["your_tool"]="your_command -i"
    # ... existing tools
)
```

### Extending Metrics
- Add new measurement code in the `benchmark_tool` function
- Update CSV header and JSON structure accordingly
- Ensure backward compatibility with existing data

### Improving Analysis
- Enhance statistical calculations
- Add new visualization types to Jekyll page
- Implement regression detection algorithms

### Test Data Contributions
- Submit diverse SVG test files
- Ensure files are appropriately licensed
- Document file sources and characteristics

## Integration with CI/CD

### GitHub Actions Example
```yaml
name: Performance Benchmarks
on:
  schedule:
    - cron: '0 2 * * 0'  # Weekly benchmarks
  
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Bun
        run: curl -fsSL https://bun.sh/install | bash
      - name: Install SVGO
        run: npm install -g svgo
      - name: Build Vexy SVGO
        run: cargo build --release
      - name: Run Benchmarks
        run: ./scripts/benchmark-comprehensive.sh testdata 5 20 json true true
      - name: Upload Results
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: benchmark_results_*.json
```

### Automated Reporting
- Generate benchmark reports automatically
- Track performance regressions over time
- Send notifications for significant changes
- Update documentation with latest results

## License

These benchmarking tools are part of the Vexy SVGO project and are available under the same license terms as the main project.
