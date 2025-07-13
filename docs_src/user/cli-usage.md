---
nav_weight: 22
# this_file: docs/user/cli-usage.md
layout: default
title: CLI Usage
parent: User Guide
nav_order: 2
description: "Master the Vexy SVGO command-line interface"
---

# CLI Usage
{: .no_toc }

Master the Vexy SVGO command-line interface
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Basic Usage

Vexy SVGO provides a powerful command-line interface that's fully compatible with SVGO while adding performance and convenience features.

### Single File Optimization

```bash
# Basic optimization
vexy-svgo input.svg -o output.svg

# Using pipes (STDIN/STDOUT)
cat input.svg | vexy-svgo > output.svg

# Process string directly
vexy-svgo -s '<svg xmlns="http://www.w3.org/2000/svg"><circle r="50"/></svg>'
```

### Multiple Files

```bash
# Multiple specific files
vexy-svgo icon1.svg icon2.svg icon3.svg

# All SVG files in current directory
vexy-svgo *.svg

# Process directory
vexy-svgo -f assets/icons/

# Recursive directory processing
vexy-svgo -f assets/ -r
```

---

## Command Options

### Input/Output Options

| Option | Description | Example |
|--------|-------------|---------|
| `-i, --input` | Input file/directory/STDIN | `vexy-svgo -i input.svg` |
| `-o, --output` | Output file/directory/STDOUT | `vexy-svgo -o output.svg` |
| `-s, --string` | Process SVG string directly | `vexy-svgo -s '<svg>...</svg>'` |
| `-f, --folder` | Process entire folder | `vexy-svgo -f ./icons/` |
| `-r, --recursive` | Process folders recursively | `vexy-svgo -f ./assets/ -r` |

### Formatting Options

| Option | Description | Default |
|--------|-------------|---------|
| `--pretty` | Pretty print output | `false` |
| `--indent` | Indentation spaces | `2` |
| `--eol` | Line ending (lf/crlf) | Platform default |
| `--final-newline` | Ensure trailing newline | `false` |
| `-p, --precision` | Numeric precision | `3` |

### Plugin Options

| Option | Description | Example |
|--------|-------------|---------|
| `--config` | Custom config file | `--config .svgo.config.js` |
| `--disable` | Disable plugin | `--disable removeViewBox` |
| `--enable` | Enable plugin | `--enable cleanupIds` |
| `--show-plugins` | List available plugins | `vexy-svgo --show-plugins` |

### Output Options

| Option | Description | Example |
|--------|-------------|---------|
| `--datauri` | Output as Data URI | `--datauri base64` |
| `--multipass` | Multiple optimization passes | `--multipass` |
| `-q, --quiet` | Suppress output | `-q` |
| `--no-color` | Disable colored output | `--no-color` |

---

## Common Workflows

### Web Development

```bash
# Standard web optimization
vexy-svgo input.svg -o output.svg --pretty

# Aggressive compression for production
vexy-svgo input.svg -o output.svg --multipass -p 2

# Process all icons in a build
vexy-svgo -f src/assets/icons/ -o dist/assets/icons/ -r
```

### Build Pipeline Integration

```bash
# Process new/modified SVGs only
find . -name "*.svg" -newer .last-build | xargs vexy-svgo

# Git pre-commit hook
git diff --cached --name-only --diff-filter=A | grep '\.svg$' | xargs vexy-svgo

# npm script integration
npm run icons:optimize && vexy-svgo -f dist/icons/
```

### Batch Processing

```bash
# Process with progress (large batches)
vexy-svgo -f ./thousands-of-icons/ -r --verbose

# Custom output structure
vexy-svgo -f src/icons/ -o dist/icons/ -r --pretty

# Exclude certain files
vexy-svgo -f assets/ -r --exclude "temp|backup" --exclude ".*\.min\.svg"
```

---

## Advanced Usage

### Custom Precision

Control numeric precision for different use cases:

```bash
# High precision for print (slower, larger)
vexy-svgo input.svg -p 5

# Low precision for web (faster, smaller)  
vexy-svgo input.svg -p 1

# Balanced for most use cases
vexy-svgo input.svg -p 3
```

### Plugin Configuration

```bash
# Disable specific optimizations
vexy-svgo input.svg --disable removeViewBox --disable removeDimensions

# Enable additional plugins
vexy-svgo input.svg --enable cleanupIds --enable sortAttrs

# Show all available plugins
vexy-svgo --show-plugins
```

### Output Formats

```bash
# Generate Data URI for CSS
vexy-svgo input.svg --datauri base64 > icon.css

# Pretty formatted for development
vexy-svgo input.svg --pretty --indent 4

# Minified for production
vexy-svgo input.svg
```

---

## Configuration Files

### .svgo.config.js (SVGO Compatible)

```javascript
module.exports = {
  multipass: true,
  plugins: [
    'removeComments',
    'removeEmptyAttrs',
    {
      name: 'convertColors',
      params: {
        currentColor: true
      }
    }
  ]
};
```

### .svgo.config.json (Native Format)

```json
{
  "multipass": true,
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

### Using Config Files

```bash
# Use specific config file
vexy-svgo --config custom.config.js input.svg

# Automatic discovery (.svgo.config.js/json in current or parent dirs)
vexy-svgo input.svg

# Override config options
vexy-svgo --config base.config.js --disable removeComments input.svg
```

---

## Performance Tips

### Maximize Speed

```bash
# Use native binary (not wrapper)
vexy-svgo input.svg

# Batch processing instead of individual files
vexy-svgo *.svg  # ✅ Fast
# vs
for f in *.svg; do vexy-svgo "$f"; done  # ❌ Slow

# Enable parallel processing for large files (if feature enabled)
vexy-svgo --parallel large-complex.svg

# Disable unnecessary plugins for simple files
vexy-svgo --disable removeMetadata --disable removeTitle simple.svg
```

### Memory Optimization

```bash
# For very large files, use streaming
vexy-svgo --stream large-file.svg

# Process large batches in chunks
find . -name "*.svg" | xargs -n 50 vexy-svgo

# Configure parallel processing for optimal performance
vexy-svgo --parallel --parallel-threads 4 massive-file.svg
```

### Parallel Processing

When working with large SVG files (>1MB or >1000 elements), Vexy SVGO can automatically enable parallel processing:

```bash
# Automatic parallel activation for large files
vexy-svgo huge-map.svg  # Automatically uses parallel if file is large

# Force parallel processing
vexy-svgo --parallel icon-set.svg

# Configure parallel thresholds
vexy-svgo --parallel-size-threshold 512000 --parallel-element-threshold 500 file.svg
```

---

## Integration Examples

### Makefile

```makefile
optimize-svg:
	vexy-svgo -f src/assets/icons/ -o dist/assets/icons/ -r

watch-svg:
	watchman-make -p '**/*.svg' -t optimize-svg

.PHONY: optimize-svg watch-svg
```

### npm Scripts

```json
{
  "scripts": {
    "icons:optimize": "vexy-svgo -f src/icons/ -o dist/icons/ -r",
    "icons:watch": "chokidar 'src/**/*.svg' -c 'npm run icons:optimize'",
    "build": "npm run icons:optimize && webpack"
  }
}
```

### GitHub Actions

```yaml
- name: Optimize SVGs
  run: |
    curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-linux.tar.gz | tar -xz
    ./vexy-svgo -f assets/ -r
```

---

## Migration from SVGO

Vexy SVGO is designed as a drop-in replacement:

```bash
# Replace this:
npx svgo input.svg -o output.svg

# With this:
vexy-svgo input.svg -o output.svg
```

### Key Differences

| Feature | SVGO | Vexy SVGO |
|---------|------|-----------|
| **Speed** | Baseline | 12x faster |
| **Config** | .svgo.config.js | .svgo.config.js/.json |
| **Precision** | --precision | -p, --precision |
| **Folder** | --folder | -f, --folder |
| **String input** | Not supported | -s, --string |

### Breaking Changes

- Some plugin parameters may have different names
- Custom plugins need to be rewritten in Rust
- Output formatting may have minor differences

---

## Troubleshooting

### Common Issues

**"Command not found"**
```bash
# Check if vexy-svgo is in PATH
which vexy-svgo
echo $PATH

# Install or move binary to PATH location
```

**"Permission denied"**
```bash
# Make binary executable
chmod +x vexy-svgo
```

**"Invalid SVG"**
```bash
# Validate input SVG
xmllint --noout input.svg

# Check encoding
file input.svg
```

### Debug Mode

```bash
# Verbose output for debugging
vexy-svgo --verbose input.svg

# Time operations
time vexy-svgo input.svg
```

---

## Next Steps

- [Configuration](/user/configuration/) - Learn about configuration options
- [Plugins](/plugins/) - Understand available optimizations
- [Interactive Demo](/demo/) - Try optimizations in your browser

---

*Need help? Check our [GitHub Discussions](https://github.com/vexyart/vexy-svgo/discussions) for community support.*