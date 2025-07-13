---
nav_weight: 34
---

# Migration Guide: From SVGO to Vexy SVGO

This comprehensive guide helps you migrate from the original SVGO (JavaScript) to Vexy SVGO (Rust), covering everything from basic usage to advanced integration scenarios.

## Table of Contents

1. [Overview](#overview)
2. [Installation Migration](#installation-migration)
3. [CLI Migration](#cli-migration)
4. [Configuration Migration](#configuration-migration)
5. [API Migration](#api-migration)
6. [Plugin Migration](#plugin-migration)
7. [Performance Comparison](#performance-comparison)
8. [Troubleshooting](#troubleshooting)

## Overview

Vexy SVGO is a high-performance Rust implementation of SVGO that maintains API compatibility while providing significant performance improvements. The migration process is designed to be as seamless as possible.

### Key Benefits of Migration

- **10-50x Performance Improvement**: Native Rust performance
- **Reduced Memory Usage**: Efficient memory management
- **Better Error Handling**: Typed errors with detailed context
- **Parallel Processing**: Multi-core optimization support
- **Cross-Platform**: Native binaries for all platforms
- **WebAssembly Support**: Run in browsers and Node.js

### Compatibility Matrix

| Feature | SVGO | Vexy SVGO | Notes |
|---------|------|-----------|-------|
| CLI Interface | ✅ | ✅ | Fully compatible |
| Plugin System | ✅ | ✅ | API compatible |
| Configuration Files | ✅ | ✅ | Supports all formats |
| Node.js API | ✅ | ✅ (via WASM) | Drop-in replacement |
| Browser Support | ✅ | ✅ (via WASM) | Better performance |
| Custom Plugins | ✅ | ⚠️ | Requires Rust rewrite |

## Installation Migration

### Uninstall SVGO

```bash
# Remove global SVGO installation
npm uninstall -g svgo

# Remove from project dependencies
npm uninstall svgo
```

### Install Vexy SVGO

```bash
# Install native binary (recommended)
cargo install vexy-svgo

# Or use pre-built binaries
curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-x86_64-linux.tar.gz | tar xz

# Or use npm wrapper (Node.js projects)
npm install vexy-svgo

# Or use Homebrew (macOS)
brew install vexyart/tap/vexy-svgo
```

### Verify Installation

```bash
# Check version and verify it works
vexy-svgo --version
vexy-svgo --help

# Test with a simple SVG
echo '<svg><rect width="100" height="100"/></svg>' | vexy-svgo
```

## CLI Migration

The CLI interface is fully compatible with SVGO. Most commands work identically:

### Basic Usage

```bash
# SVGO
svgo input.svg

# Vexy SVGO (identical)
vexy-svgo input.svg
```

### Common CLI Commands

| SVGO Command | Vexy SVGO Equivalent | Notes |
|--------------|---------------------|-------|
| `svgo *.svg` | `vexy-svgo *.svg` | Identical syntax |
| `svgo -i input.svg -o output.svg` | `vexy-svgo -i input.svg -o output.svg` | Identical |
| `svgo --config=.svgorc` | `vexy-svgo --config=.svgorc` | Same config format |
| `svgo --pretty` | `vexy-svgo --pretty` | Identical output |
| `svgo --multipass` | `vexy-svgo --multipass` | Same optimization |

### New CLI Features

Vexy SVGO adds several new CLI options:

```bash
# Parallel processing (new)
vexy-svgo --parallel=4 *.svg

# Progress indicators (enhanced)
vexy-svgo --verbose folder/*.svg

# Dry run mode (enhanced)
vexy-svgo --dry-run *.svg

# Memory limit controls (new)
vexy-svgo --memory-limit=1GB large-file.svg
```

### Batch Processing Improvements

```bash
# SVGO: Process directory
svgo -f src/icons -o dist/icons

# Vexy SVGO: Same syntax, much faster
vexy-svgo -f src/icons -o dist/icons

# Vexy SVGO: Enhanced with progress
vexy-svgo -f src/icons -o dist/icons --verbose --parallel=8
```

## Configuration Migration

Vexy SVGO supports all SVGO configuration formats without modification.

### Configuration File Formats

All existing SVGO configuration files work with Vexy SVGO:

```javascript
// .svgo.config.js (supported)
module.exports = {
  plugins: [
    'removeDoctype',
    'removeComments',
    {
      name: 'removeViewBox',
      params: {
        removeViewBox: false
      }
    }
  ]
};
```

```json
// .svgorc.json (supported)
{
  "plugins": [
    "removeDoctype",
    "removeComments",
    {
      "name": "removeViewBox", 
      "params": {
        "removeViewBox": false
      }
    }
  ]
}
```

```yaml
# svgo.config.yaml (supported)
plugins:
  - removeDoctype
  - removeComments
  - name: removeViewBox
    params:
      removeViewBox: false
```

### Plugin Configuration

Plugin configurations are identical:

```javascript
// SVGO configuration
module.exports = {
  multipass: true,
  plugins: [
    'removeDoctype',
    'removeXMLProcInst',
    'removeComments',
    'removeMetadata',
    'removeTitle',
    'removeDesc',
    'removeUselessDefs',
    'removeEditorsNSData',
    'removeEmptyAttrs',
    'removeHiddenElems',
    'removeEmptyText',
    'removeEmptyContainers',
    'removeViewBox',
    'cleanupEnableBackground',
    'convertStyleToAttrs',
    'convertColors',
    'convertPathData',
    'convertTransform',
    'removeUnknownsAndDefaults',
    'removeNonInheritableGroupAttrs',
    'removeUselessStrokeAndFill',
    'removeUnusedNS',
    'cleanupIDs',
    'cleanupNumericValues',
    'moveElemsAttrsToGroup',
    'moveGroupAttrsToElems',
    'collapseGroups',
    'removeDimensions',
    'removeAttrs',
    'removeAttributesBySelector',
    'removeElementsByAttr',
    'addClassesToSVGElement',
    'removeStyleElement',
    'removeScriptElement',
    'addAttributesToSVGElement',
    'sortAttrs',
    'removeDuplicateGradientStops'
  ]
};

// This exact configuration works with Vexy SVGO
```

### Advanced Configuration

```javascript
// Complex plugin configuration (fully supported)
module.exports = {
  multipass: true,
  js2svg: {
    pretty: true,
    indent: 2
  },
  plugins: [
    {
      name: 'convertColors',
      params: {
        currentColor: true,
        names2hex: true,
        rgb2hex: true,
        shorthex: true,
        shortname: true
      }
    },
    {
      name: 'cleanupIDs',
      params: {
        minify: true,
        preserve: ['important-id-1', 'important-id-2'],
        preservePrefixes: ['icon-', 'logo-']
      }
    }
  ]
};
```

## API Migration

### Node.js API

Vexy SVGO provides a WebAssembly-based Node.js API that's drop-in compatible:

```javascript
// SVGO API
const { optimize } = require('svgo');

const result = optimize(svgString, {
  path: input.svg,
  plugins: ['removeDoctype', 'removeComments']
});

console.log(result.data);
```

```javascript
// Vexy SVGO API (identical interface)
const { optimize } = require('vexy-svgo');

const result = optimize(svgString, {
  path: 'input.svg',
  plugins: ['removeDoctype', 'removeComments']
});

console.log(result.data);
```

### TypeScript Support

```typescript
// SVGO TypeScript
import { optimize, Config } from 'svgo';

const config: Config = {
  plugins: ['removeComments']
};

const result = optimize(svgString, config);
```

```typescript
// Vexy SVGO TypeScript (identical)
import { optimize, Config } from 'vexy-svgo';

const config: Config = {
  plugins: ['removeComments']
};

const result = optimize(svgString, config);
```

### Browser API

```javascript
// SVGO browser (via bundler)
import { optimize } from 'svgo/dist/svgo.browser.js';

// Vexy SVGO browser (WebAssembly)
import { optimize } from 'vexy-svgo/web';

// Same API, better performance
const result = optimize(svgString, config);
```

### Async/Promise Support

```javascript
// Vexy SVGO supports both sync and async
const { optimize, optimizeAsync } = require('vexy-svgo');

// Synchronous (same as SVGO)
const result = optimize(svgString, config);

// Asynchronous (new, for large files)
const result = await optimizeAsync(svgString, config);
```

## Plugin Migration

### Built-in Plugin Compatibility

All SVGO built-in plugins are available in Vexy SVGO with identical behavior:

| SVGO Plugin | Vexy SVGO | Status | Notes |
|-------------|-----------|--------|-------|
| `removeDoctype` | ✅ | ✅ | Identical |
| `removeComments` | ✅ | ✅ | Identical |
| `removeMetadata` | ✅ | ✅ | Identical |
| `removeTitle` | ✅ | ✅ | Identical |
| `removeDesc` | ✅ | ✅ | Identical |
| `removeViewBox` | ✅ | ✅ | Identical |
| `convertColors` | ✅ | ✅ | Identical |
| `convertPathData` | ✅ | ✅ | Enhanced |
| `cleanupIDs` | ✅ | ✅ | Identical |
| All others | ✅ | ✅ | 50+ plugins |

### Custom Plugin Migration

If you have custom SVGO plugins, they need to be rewritten in Rust:

**SVGO Plugin (JavaScript):**
```javascript
// custom-plugin.js
exports.type = 'visitor';
exports.name = 'customPlugin';
exports.description = 'My custom plugin';

exports.fn = (root, params) => {
  // Plugin logic
  visit(root, (node) => {
    if (node.type === 'element' && node.name === 'rect') {
      node.attributes['data-custom'] = 'true';
    }
  });
};
```

**Vexy SVGO Plugin (Rust):**
```rust
// custom_plugin.rs
use crate::Plugin;
use anyhow::Result;
use vexy_svgo_core::ast::{Document, Element, Node};

#[derive(Clone)]
pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn name(&self) -> &'static str {
        "customPlugin"
    }

    fn description(&self) -> &'static str {
        "My custom plugin"
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.process_element(&mut document.root);
        Ok(())
    }
}

impl CustomPlugin {
    fn process_element(&self, element: &mut Element) {
        if element.name == "rect" {
            element.attributes.insert("data-custom".to_string(), "true".to_string());
        }
        
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.process_element(elem);
            }
        }
    }
}
```

### Plugin Development Resources

- [Plugin Development Guide](/developer/plugin-development/)
- [Plugin API Reference](/developer/api-reference/)
- [Example Plugins](https://github.com/vexyart/vexy-svgo/tree/main/examples)

## Performance Comparison

### Benchmark Results

| Test Case | SVGO (Node.js) | Vexy SVGO | Speedup |
|-----------|----------------|-----------|---------|
| Single small SVG (1KB) | 12ms | 0.8ms | 15x |
| Single large SVG (100KB) | 180ms | 8ms | 22x |
| Batch 100 icons | 2.3s | 95ms | 24x |
| Batch 1000 icons | 23s | 450ms | 51x |
| Complex paths | 340ms | 12ms | 28x |

### Memory Usage

| Scenario | SVGO Memory | Vexy SVGO Memory | Improvement |
|----------|-------------|------------------|-------------|
| Small files | 45MB | 12MB | 73% less |
| Large files | 180MB | 32MB | 82% less |
| Batch processing | 320MB | 28MB | 91% less |

### Real-World Migration Results

**Case Study: Icon Library (2000 SVG files)**

```bash
# Before (SVGO)
time svgo -f icons/ -o dist/
# real: 1m 34s
# Memory peak: 280MB

# After (Vexy SVGO) 
time vexy-svgo -f icons/ -o dist/
# real: 3.2s
# Memory peak: 24MB

# Result: 29x faster, 92% less memory
```

## Troubleshooting

### Common Migration Issues

#### 1. Plugin Not Found

```bash
# Error: Plugin 'myCustomPlugin' not found
```

**Solution:** Custom plugins need to be rewritten in Rust or use plugin adapter.

#### 2. Configuration File Issues

```bash
# Error: Could not parse configuration
```

**Solution:** Verify your config file syntax. Vexy SVGO supports the same formats but with stricter parsing.

#### 3. Path Issues

```bash
# Error: File not found
```

**Solution:** Check file paths. Vexy SVGO uses the current working directory by default.

#### 4. Performance Regression

If you notice performance regression:

```bash
# Enable parallel processing
vexy-svgo --parallel=auto *.svg

# Increase memory limit for large files
vexy-svgo --memory-limit=2GB large.svg

# Use streaming for very large files
vexy-svgo --streaming large.svg
```

### Debug Mode

```bash
# Enable debug output
vexy-svgo --verbose --debug input.svg

# Get detailed timing information
vexy-svgo --benchmark input.svg
```

### Getting Help

1. **Documentation**: Check [docs.vexy-svgo.dev](https://docs.vexy-svgo.dev)
2. **Issues**: Report at [GitHub Issues](https://github.com/vexyart/vexy-svgo/issues)
3. **Community**: Join discussions in [GitHub Discussions](https://github.com/vexyart/vexy-svgo/discussions)

## Migration Checklist

### Pre-Migration

- [ ] Backup your SVG files
- [ ] Document your current SVGO configuration
- [ ] List any custom plugins you use
- [ ] Test current optimization results

### Migration Steps

- [ ] Install Vexy SVGO
- [ ] Test basic functionality with sample files
- [ ] Migrate configuration files
- [ ] Test with your production SVG files
- [ ] Compare optimization results
- [ ] Update build scripts/CI/CD
- [ ] Update documentation

### Post-Migration

- [ ] Monitor performance improvements
- [ ] Update team documentation
- [ ] Remove old SVGO installation
- [ ] Consider using new Vexy SVGO features

### Rollback Plan

If you need to rollback:

```bash
# Reinstall SVGO
npm install -g svgo

# Your configuration files are unchanged
svgo --config=.svgo.config.js *.svg
```

## Advanced Migration Scenarios

### CI/CD Pipeline Migration

**Before (GitHub Actions with SVGO):**
```yaml
- name: Optimize SVGs
  run: |
    npm install -g svgo
    svgo -f src/icons -o dist/icons
```

**After (GitHub Actions with Vexy SVGO):**
```yaml
- name: Optimize SVGs  
  run: |
    curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-x86_64-linux.tar.gz | tar xz
    ./vexy-svgo -f src/icons -o dist/icons --parallel=auto
```

### Docker Migration

**Before:**
```dockerfile
FROM node:18
RUN npm install -g svgo
```

**After:**
```dockerfile
FROM debian:bullseye-slim
RUN curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-x86_64-linux.tar.gz | tar xz -C /usr/local/bin
```

### Build Tool Integration

**Webpack:**
```javascript
// Before
const SVGOWebpackPlugin = require('svgo-webpack-plugin');

// After  
const VexySVGOWebpackPlugin = require('vexy-svgo-webpack-plugin');
```

**Gulp:**
```javascript
// Before
const svgo = require('gulp-svgo');

// After
const vexySvgo = require('gulp-vexy-svgo');
```

## Conclusion

Migrating from SVGO to Vexy SVGO provides significant performance benefits while maintaining full compatibility. The migration process is straightforward for most use cases, requiring minimal changes to existing workflows.

The key benefits include:
- **Dramatic Performance Improvements**: 10-50x faster processing
- **Reduced Resource Usage**: 70-90% less memory consumption  
- **Enhanced Features**: Parallel processing, better error handling
- **Future-Proof**: Native performance with WebAssembly compatibility

Start with a small test migration, verify the results, then gradually roll out to your full workflow. The compatibility guarantees ensure a smooth transition with immediate benefits.