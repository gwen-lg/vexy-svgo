# CLI Usage Examples

This document provides common usage examples for the Vexy SVGO command-line interface.

## Basic Usage

### Optimize a single file

```bash
# Basic optimization with default settings
vexy_svgo input.svg -o output.svg

# Using stdin/stdout
cat input.svg | vexy_svgo > output.svg

# Optimize in place (overwrites the original)
vexy_svgo input.svg
```

### Optimize multiple files

```bash
# Optimize multiple files
vexy_svgo file1.svg file2.svg file3.svg

# With custom output names
vexy_svgo input1.svg -o output1.svg input2.svg -o output2.svg

# Using glob patterns (handled by shell)
vexy_svgo *.svg

# Optimize all SVG files in a directory
vexy_svgo -f ./images/
```

## Configuration Options

### Pretty printing

```bash
# Pretty print with indentation
vexy_svgo input.svg -o output.svg --pretty

# Custom indentation (4 spaces)
vexy_svgo input.svg -o output.svg --pretty --indent 4
```

### Multipass optimization

```bash
# Run optimization passes until no more improvements
vexy_svgo input.svg -o output.svg --multipass
```

### Precision control

```bash
# Set decimal precision for numbers (default: 3)
vexy_svgo input.svg -o output.svg --precision 2
```

## Plugin Management

### Disable specific plugins

```bash
# Disable removeComments plugin
vexy_svgo input.svg -o output.svg --disable removeComments

# Disable multiple plugins
vexy_svgo input.svg -o output.svg --disable removeComments,removeEmptyAttrs
```

### Enable only specific plugins

```bash
# Use only specific plugins
vexy_svgo input.svg -o output.svg --enable removeComments,collapseGroups
```

### Configure plugin parameters

```bash
# Configure convertColors plugin
vexy_svgo input.svg -o output.svg --config '{"plugins":[{"name":"convertColors","params":{"currentColor":true}}]}'
```

## Using Configuration Files

### YAML configuration

Create `vexy_svgo.config.yml`:
```yaml
multipass: true
js2svg:
  pretty: true
  indent: 2
plugins:
  - removeComments
  - removeEmptyAttrs
  - convertColors:
      currentColor: true
      names2hex: true
  - cleanupIds:
      minify: true
```

Use it:
```bash
vexy_svgo input.svg -o output.svg --config vexy_svgo.config.yml
```

### JSON configuration

Create `vexy_svgo.config.json`:
```json
{
  "multipass": true,
  "js2svg": {
    "pretty": true,
    "indent": 2
  },
  "plugins": [
    "removeComments",
    "removeEmptyAttrs",
    {
      "name": "convertColors",
      "params": {
        "currentColor": true
      }
    }
  ]
}
```

Use it:
```bash
vexy_svgo input.svg -o output.svg --config vexy_svgo.config.json
```

## Output Options

### Data URI output

```bash
# Output as data URI (base64)
vexy_svgo input.svg --datauri base64

# Output as URL-encoded data URI
vexy_svgo input.svg --datauri enc

# Output as unencoded data URI
vexy_svgo input.svg --datauri unenc
```

### Show optimization info

```bash
# Display optimization statistics
vexy_svgo input.svg -o output.svg --show-info

# Example output:
# Original: 2,451 bytes
# Optimized: 1,234 bytes (49.6% reduction)
# Plugins applied: 12
# Optimization passes: 2
```

## Advanced Usage

### Process SVG from string

```bash
# Optimize SVG string directly
vexy_svgo -s '<svg><rect x="0" y="0" width="100" height="100"/></svg>'
```

### Batch processing with custom settings

```bash
# Create a script for batch processing
for file in images/*.svg; do
  vexy_svgo "$file" -o "optimized/$(basename "$file")" \
    --multipass \
    --precision 2 \
    --config production.yml
done
```

### Integration with other tools

```bash
# Combine with imagemin
imagemin images/*.svg --plugin=vexy_svgo > optimized/

# Use with find
find . -name "*.svg" -exec vexy_svgo {} \;

# Process and compress
vexy_svgo input.svg | gzip > output.svgz
```

## Feature Flags (if enabled in build)

```bash
# Enable parallel processing
vexy_svgo input.svg -o output.svg --features parallel

# Enable experimental plugins
vexy_svgo input.svg -o output.svg --features experimental

# List available features
vexy_svgo features list

# Show current feature configuration
vexy_svgo features show
```

## Common Optimization Scenarios

### Web production build

```bash
# Aggressive optimization for web
vexy_svgo input.svg -o output.svg \
  --multipass \
  --precision 2 \
  --disable removeViewBox,removeTitle
```

### Icon optimization

```bash
# Optimize SVG icons
vexy_svgo icon.svg -o icon-optimized.svg \
  --enable removeComments,removeEmptyAttrs,convertColors,cleanupIds \
  --config '{"plugins":[{"name":"cleanupIds","params":{"minify":true}}]}'
```

### Preserve editability

```bash
vexy-svgo input.svg -o output.svg \
  --pretty \
  --disable convertPathData,mergePaths \
  --config keep-editable.yml
```

## Error Handling

```bash
# Verbose output for debugging
vexy-svgo input.svg -o output.svg --verbose

# Quiet mode (suppress non-error output)
vexy-svgo input.svg -o output.svg --quiet

# Continue on errors when processing multiple files
vexy-svgo *.svg --continue-on-error
```

## Advanced Usage

### Memory and Performance Options

```bash
# Enable parallel processing with 8 threads
vexy_svgo large-file.svg --parallel 8

# Set memory limit (useful for very large files)
vexy_svgo huge-file.svg --memory-limit 500MB

# Enable streaming mode for huge files
vexy_svgo massive-file.svg --streaming

# Batch processing with custom batch size
vexy_svgo -f ./svg-folder --batch-size 10
```

### Output Formats

```bash
# Generate data URI (base64)
vexy_svgo icon.svg --datauri base64

# Generate data URI (URL encoded)
vexy_svgo icon.svg --datauri enc

# Generate data URI (unencoded)
vexy_svgo icon.svg --datauri unenc

# JSON output with optimization stats
vexy_svgo input.svg --output-format json

# Output to stdout
vexy_svgo input.svg -o -
```

### Benchmarking and Analysis

```bash
# Show optimization statistics
vexy_svgo input.svg --stats

# Benchmark mode (shows timing information)
vexy_svgo input.svg --benchmark

# Show detailed timing for each plugin
vexy_svgo input.svg --timing

# Show progress indicator for batch processing
vexy_svgo -f ./large-folder --progress
```

### Integration Examples

```bash
# Process SVG from URL (via curl)
curl https://example.com/image.svg | vexy_svgo > optimized.svg

# Optimize all SVG files and save with .min.svg suffix
for file in *.svg; do
    vexy_svgo "$file" -o "${file%.svg}.min.svg"
done

# Find and optimize all SVG files recursively
find . -name "*.svg" -exec vexy_svgo {} \;

# Optimize and copy to different directory
vexy_svgo src/*.svg --output-dir dist/

# Watch mode (requires external tool like entr)
ls *.svg | entr -c vexy_svgo /_
```

## Real-World Scenarios

### Web Development Workflow

```bash
# Optimize all icons in a project
vexy_svgo src/assets/icons/*.svg --output-dir public/icons/

# Optimize with specific settings for web
vexy_svgo logo.svg \
  --enable removeViewBox=false \
  --enable removeTitle=false \
  --precision 2 \
  -o public/logo.svg

# Generate optimized sprites
cat icons/*.svg | vexy_svgo --multipass > sprite.svg
```

### Build System Integration

```bash
# npm scripts in package.json
{
  "scripts": {
    "optimize:svg": "vexy_svgo src/**/*.svg --quiet",
    "build:icons": "vexy_svgo src/icons/*.svg --output-dir dist/icons/ --stats",
    "watch:svg": "chokidar 'src/**/*.svg' -c 'npm run optimize:svg'"
  }
}

# Makefile integration
optimize-svg:
	@echo "Optimizing SVG files..."
	@vexy_svgo assets/*.svg --quiet --stats

# GitHub Actions
- name: Optimize SVG files
  run: |
    vexy_svgo **/*.svg --stats --continue-on-error
```

### Debugging and Troubleshooting

```bash
# Debug plugin execution order
vexy_svgo input.svg --show-plugins

# Test with specific plugins only
vexy_svgo input.svg \
  --disable '*' \
  --enable removeComments \
  --enable removeMetadata \
  --verbose

# Compare before and after
vexy_svgo input.svg -o output.svg --stats
diff -u input.svg output.svg

# Validate SVG after optimization
vexy_svgo input.svg | xmllint --noout -
```

### Batch Processing Patterns

```bash
# Process files in parallel with progress
vexy_svgo -f ./images --parallel 4 --progress

# Process with error handling and logging
vexy_svgo *.svg \
  --continue-on-error \
  --stats \
  2> errors.log \
  | tee optimization.log

# Conditional optimization based on file size
for file in *.svg; do
    size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file")
    if [ $size -gt 10000 ]; then
        echo "Optimizing large file: $file"
        vexy_svgo "$file" --multipass --parallel
    else
        vexy_svgo "$file"
    fi
done
```

## Platform-Specific Examples

### macOS

```bash
# Using with Homebrew
brew install vexy-svgo
vexy_svgo ~/Desktop/*.svg

# Quick Look integration
vexy_svgo input.svg -o - | qlmanage -p -
```

### Windows

```batch
REM Batch file for Windows
@echo off
for %%f in (*.svg) do (
    vexy_svgo "%%f" -o "optimized_%%f"
)

REM PowerShell
Get-ChildItem -Filter *.svg | ForEach-Object {
    vexy_svgo $_.FullName -o "optimized_$($_.Name)"
}
```

### Linux

```bash
# System-wide installation
sudo cp vexy_svgo /usr/local/bin/
sudo chmod +x /usr/local/bin/vexy_svgo

# Desktop file association
xdg-mime default vexy-svgo.desktop image/svg+xml
```