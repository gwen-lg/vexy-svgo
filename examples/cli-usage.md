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
# Keep SVG editable in vector editors
vexy_svgo input.svg -o output.svg \
  --pretty \
  --disable convertPathData,mergePaths \
  --config keep-editable.yml
```

## Error Handling

```bash
# Verbose output for debugging
vexy_svgo input.svg -o output.svg --verbose

# Quiet mode (suppress non-error output)
vexy_svgo input.svg -o output.svg --quiet

# Continue on errors when processing multiple files
vexy_svgo *.svg --continue-on-error
```