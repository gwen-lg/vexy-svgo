# SVGO vs Vexy SVGO Comparative Testing Tools

This directory contains comprehensive testing tools for comparing outputs between SVGO and Vexy SVGO optimizers. These tools ensure compatibility and help identify differences in optimization behavior.

## Overview

The comparative testing suite provides multiple levels of testing:

1. **Single File Comparison** - Compare one SVG file between SVGO and Vexy SVGO
2. **Batch Testing** - Test multiple files at once with detailed reporting
3. **Plugin Testing** - Test individual plugins across multiple files
4. **Regression Testing** - Track changes over time

## Tools

### 1. `compare_outputs.sh` - Single File Comparison

Compare a single SVG file between SVGO and Vexy SVGO.

```bash
# Basic comparison
./compare_outputs.sh input.svg

# Test specific plugin
./compare_outputs.sh --plugin cleanupAttrs input.svg

# Show detailed diff and statistics
./compare_outputs.sh --diff --stats input.svg

# Build Vexy SVGO before testing
./compare_outputs.sh --build input.svg
```

**Features:**
- Side-by-side optimization comparison
- File size analysis
- Detailed diff output
- Plugin-specific testing
- Automatic result archiving

### 2. `batch_test.sh` - Batch Testing

Test multiple SVG files and generate comprehensive reports.

```bash
# Test all files in a directory
./batch_test.sh --dir testdata/

# Test specific files
./batch_test.sh --files "file1.svg,file2.svg,file3.svg"

# Test with specific plugin
./batch_test.sh --plugin removeComments --dir testdata/

# Quick test (first 10 files only)
./batch_test.sh --quick --dir testdata/

# Verbose output with detailed logs
./batch_test.sh --verbose --dir testdata/
```

**Features:**
- Batch processing of multiple files
- Comprehensive markdown reports
- Statistics and compatibility analysis
- Individual test result archiving
- Performance comparison metrics

### 3. `test_plugins.sh` - Plugin Testing

Test individual SVGO/Vexy SVGO plugins for compatibility.

```bash
# Test a specific plugin
./test_plugins.sh cleanupAttrs

# Test all common plugins
./test_plugins.sh --all

# Quick test (first 5 plugins)
./test_plugins.sh --all --quick

# Test plugin with specific file
./test_plugins.sh --file mytest.svg removeComments

# List available plugins
./test_plugins.sh --list
```

**Features:**
- Plugin-specific testing
- Tests 30+ common SVGO plugins
- Per-plugin compatibility reports
- Automatic test file discovery
- Success/failure tracking

## Output Structure

All test results are saved in organized directory structures:

```
test/results/
├── batch_20240115_143022/          # Batch test results
│   ├── batch_report.md             # Main report
│   └── tests/                      # Individual test results
│       ├── test1/
│       │   ├── comparison_output.txt
│       │   └── metadata.json
│       └── test2/
├── plugins/                        # Plugin test results
│   ├── cleanupAttrs/
│   │   ├── plugin_report.md
│   │   └── test1_test.txt
│   └── removeComments/
└── 20240115_143022_mytest/         # Single comparison
    ├── original.svg
    ├── svgo.svg
    ├── vexy_svgo.svg
    └── report.txt
```

## Common Workflows

### Complete Compatibility Check

Test all aspects of SVGO/Vexy SVGO compatibility:

```bash
# 1. Test all plugins quickly
./test_plugins.sh --all --quick

# 2. Run comprehensive batch test
./batch_test.sh --dir testdata/ --verbose

# 3. Test specific problematic files
./compare_outputs.sh --diff --stats problematic.svg
```

### Plugin Development

When developing a new plugin or fixing compatibility:

```bash
# 1. Test the specific plugin
./test_plugins.sh --file mytest.svg myPlugin

# 2. Compare detailed outputs
./compare_outputs.sh --plugin myPlugin --diff mytest.svg

# 3. Run batch test to ensure no regressions
./batch_test.sh --plugin myPlugin --dir testdata/
```

### Regression Testing

Track changes over development:

```bash
# Regular comprehensive testing
./batch_test.sh --dir testdata/ > results.log

# Compare with previous results
diff previous_results.log results.log
```

## Configuration

### Environment Variables

- `SVGO_CMD` - Path to SVGO binary (default: `npx svgo`)
- `Vexy SVGO_CMD` - Path to Vexy SVGO binary (default: `target/release/vexy_svgo`)
- `TEMP_DIR` - Temporary directory for comparisons (default: `/tmp/vexy_svgo_comparison`)

### Test Data Organization

Organize test files for optimal testing:

```
testdata/
├── simple/              # Basic SVG files
├── complex/             # Complex SVG files
├── plugins/             # Plugin-specific test cases
│   ├── cleanupAttrs/
│   ├── removeComments/
│   └── convertColors/
└── samples/             # General test samples
```

## Interpreting Results

### Test Outcomes

1. **✅ Identical** - SVGO and Vexy SVGO produce identical output
2. **⚠️ Different** - Outputs differ but both tools succeeded
3. **❌ Failed** - One or both tools failed to process the file

### Performance Indicators

- **Vexy SVGO Improvements** - Cases where Vexy SVGO produces smaller output
- **Vexy SVGO Degradations** - Cases where Vexy SVGO produces larger output
- **Compatibility Rate** - Percentage of identical outputs

### Report Analysis

Batch test reports include:
- Summary statistics
- Individual test results
- Performance analysis
- Compatibility recommendations

## Troubleshooting

### Common Issues

1. **Tools Not Found**
   ```bash
   # Install SVGO
   npm install -g svgo
   
   # Build Vexy SVGO
   cargo build --release
   ```

2. **Permission Denied**
   ```bash
   chmod +x *.sh
   ```

3. **No Test Files Found**
   ```bash
   # Create test data
   mkdir -p testdata/samples
   cp *.svg testdata/samples/
   ```

4. **BC Calculator Missing**
   ```bash
   # macOS
   brew install bc
   
   # Ubuntu/Debian
   sudo apt install bc
   ```

### Debug Mode

Enable verbose output for troubleshooting:

```bash
./compare_outputs.sh --verbose input.svg
./batch_test.sh --verbose --dir testdata/
./test_plugins.sh --verbose pluginName
```

## Integration

### CI/CD Integration

Add to your GitHub Actions workflow:

```yaml
- name: Run SVGO/Vexy SVGO Compatibility Tests
  run: |
    chmod +x test/comparative/*.sh
    ./test/comparative/batch_test.sh --dir testdata/ --quick
```

### Development Workflow

Integrate into development process:

```bash
# Pre-commit hook
./test/comparative/test_plugins.sh --all --quick

# Pre-release testing
./test/comparative/batch_test.sh --dir testdata/
```

## Contributing

When adding new test tools:

1. Follow the existing naming convention
2. Include usage documentation
3. Add error handling and validation
4. Generate structured output/reports
5. Make scripts executable (`chmod +x`)

## Requirements

- **bash** 3.2+ (macOS/Linux)
- **Node.js** with npm (for SVGO)
- **Rust/Cargo** (for Vexy SVGO)
- **bc** calculator (for percentage calculations)
- **diff** utility (standard on Unix systems)

## License

Same as main Vexy SVGO project.
