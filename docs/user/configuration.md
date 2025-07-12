---
# this_file: docs/user/configuration.md
layout: default
title: Configuration
parent: User Guide
nav_order: 3
description: "Configure Vexy SVGO plugins and optimization settings"
---

# Configuration
{: .no_toc }

Configure Vexy SVGO plugins and optimization settings
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Overview

Vexy SVGO uses the same configuration format as SVGO, ensuring easy migration and compatibility. Configuration can be provided through files, command-line options, or programmatically.

### Configuration Priority

1. **Command-line options** (highest priority)
2. **Configuration files** (.svgo.config.js, .svgo.config.json)
3. **Default settings** (lowest priority)

---

## Configuration Files

### .svgo.config.js (SVGO Compatible)

The standard SVGO configuration format:

```javascript
module.exports = {
  multipass: true,
  js2svg: {
    pretty: true,
    indent: 2
  },
  plugins: [
    // Enable plugin with default settings
    'removeComments',
    'removeEmptyAttrs',
    
    // Enable plugin with custom parameters
    {
      name: 'convertColors',
      params: {
        currentColor: true,
        names2hex: true
      }
    },
    
    // Disable plugin that's enabled by default
    {
      name: 'removeViewBox',
      enabled: false
    }
  ]
};
```

### .svgo.config.json (Native Format)

JSON format for simpler configurations:

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
        "currentColor": true,
        "names2hex": true
      }
    }
  ]
}
```

### Configuration Discovery

Vexy SVGO looks for configuration files in this order:

1. File specified with `--config` option
2. `.svgo.config.js` in current directory
3. `.svgo.config.json` in current directory
4. `.svgo.config.js` in parent directories (walking up)
5. `.svgo.config.json` in parent directories (walking up)

---

## Core Settings

### multipass

Run optimization multiple times until no further improvements are made.

```javascript
{
  multipass: true  // Default: false
}
```

**Command line:** `--multipass`

### js2svg

Controls SVG output formatting:

```javascript
{
  js2svg: {
    pretty: true,           // Pretty print with indentation
    indent: 2,              // Indentation spaces
    eol: 'lf',             // Line ending: 'lf' or 'crlf'
    finalNewline: false     // Add newline at end
  }
}
```

**Command line:** `--pretty`, `--indent 4`, `--eol crlf`

### floatPrecision

Numeric precision for coordinates and values:

```javascript
{
  floatPrecision: 3  // Default: 3 decimal places
}
```

**Command line:** `-p 2`, `--precision 2`

---

## Plugin Configuration

### Plugin Format

Plugins can be configured in several ways:

```javascript
{
  plugins: [
    // Simple enable (uses default parameters)
    'removeComments',
    
    // Enable with custom parameters
    {
      name: 'cleanupNumericValues',
      params: {
        floatPrecision: 2,
        leadingZero: false
      }
    },
    
    // Explicitly disable
    {
      name: 'removeViewBox',
      enabled: false
    }
  ]
}
```

### Default Plugin Preset

Vexy SVGO includes a default preset similar to SVGO's:

```javascript
// Default plugins (applied when no config is specified)
const defaultPlugins = [
  'removeDoctype',
  'removeXMLProcInst', 
  'removeComments',
  'removeMetadata',
  'removeEditorsNSData',
  'cleanupAttrs',
  'mergeStyles',
  'cleanupIds',
  'removeUselessDefs',
  'cleanupNumericValues',
  'convertColors',
  'removeUnknownsAndDefaults',
  'removeNonInheritableGroupAttrs',
  'cleanupEnableBackground',
  'removeHiddenElems',
  'removeEmptyText',
  'convertShapeToPath',
  'convertEllipseToCircle',
  'collapseGroups',
  'convertPathData',
  'removeEmptyAttrs',
  'removeEmptyContainers',
  'removeUnusedNS',
  'sortAttrs',
  'sortDefsChildren',
  'removeDesc'
];
```

### Available Plugins

See the complete list with descriptions:

```bash
vexy-svgo --show-plugins
```

---

## Common Configuration Examples

### Web Development

Optimized for web use with readable output:

```javascript
module.exports = {
  multipass: true,
  js2svg: {
    pretty: true,
    indent: 2
  },
  plugins: [
    'removeComments',
    'removeEmptyAttrs',
    'convertColors',
    'cleanupIds',
    {
      name: 'cleanupNumericValues',
      params: { floatPrecision: 2 }
    },
    // Keep viewBox for responsive design
    { name: 'removeViewBox', enabled: false },
    // Keep dimensions for img tags
    { name: 'removeDimensions', enabled: false }
  ]
};
```

### Production Build

Maximum compression for production:

```javascript
module.exports = {
  multipass: true,
  js2svg: {
    pretty: false
  },
  plugins: [
    // Enable all safe optimizations
    'removeComments',
    'removeTitle',
    'removeDesc',
    'removeEmptyAttrs',
    'removeEmptyText',
    'removeEmptyContainers',
    'cleanupAttrs',
    'cleanupNumericValues',
    'convertColors',
    'convertPathData',
    'collapseGroups',
    'mergePaths',
    {
      name: 'cleanupNumericValues',
      params: { floatPrecision: 1 }
    }
  ]
};
```

### Icon Library

Preserve structure for icon systems:

```javascript
module.exports = {
  plugins: [
    'removeComments',
    'removeEmptyAttrs',
    'cleanupAttrs',
    'convertColors',
    {
      name: 'cleanupIds',
      params: { remove: false, minify: true }
    },
    // Preserve viewBox for icon scaling
    { name: 'removeViewBox', enabled: false },
    // Preserve groups for icon structure
    { name: 'collapseGroups', enabled: false }
  ]
};
```

### Print/High Quality

Maintain precision for print graphics:

```javascript
module.exports = {
  js2svg: {
    pretty: true,
    indent: 2
  },
  plugins: [
    'removeComments',
    'removeEmptyAttrs',
    'cleanupAttrs',
    {
      name: 'cleanupNumericValues',
      params: { floatPrecision: 5 }
    },
    // Disable aggressive optimizations
    { name: 'convertShapeToPath', enabled: false },
    { name: 'mergePaths', enabled: false }
  ]
};
```

---

## Command-Line Configuration

### Override Settings

```bash
# Use config file but override specific settings
vexy-svgo --config base.config.js --multipass --precision 1 input.svg

# Disable specific plugins
vexy-svgo --disable removeViewBox --disable removeDimensions input.svg

# Enable additional plugins  
vexy-svgo --enable cleanupIds --enable sortAttrs input.svg
```

### Multiple Plugin Operations

```bash
# Complex plugin configuration
vexy-svgo \
  --disable removeTitle \
  --disable removeDesc \
  --enable cleanupIds \
  --precision 2 \
  --multipass \
  input.svg
```

---

## Plugin Parameters

### cleanupNumericValues

```javascript
{
  name: 'cleanupNumericValues',
  params: {
    floatPrecision: 3,      // Decimal precision
    leadingZero: true,      // Keep leading zeros
    defaultPrecision: 2,    // Default for unspecified
    jsFloatPrecision: 0     // JavaScript number precision
  }
}
```

### convertColors

```javascript
{
  name: 'convertColors', 
  params: {
    currentColor: false,    // Convert to currentColor
    names2hex: true,        // Named colors to hex
    rgb2hex: true,          // rgb() to hex
    shorthex: true,         // #rrggbb to #rgb
    shortname: true         // hex to named colors
  }
}
```

### cleanupIds

```javascript
{
  name: 'cleanupIds',
  params: {
    remove: true,           // Remove unused IDs
    minify: true,           // Minify used IDs
    prefix: '',             // Add prefix to IDs
    preserve: []            // IDs to never remove
  }
}
```

### convertPathData

```javascript
{
  name: 'convertPathData',
  params: {
    applyTransforms: true,      // Apply transform matrices
    applyTransformsStroked: true, // Apply to stroked paths
    makeArcs: true,             // Convert to arcs where possible
    straightCurves: true,       // Convert curves to lines
    lineShorthands: true,       // Use H/V commands
    curveSmoothShorthands: true, // Use S/T commands
    floatPrecision: 3,          // Coordinate precision
    transformPrecision: 5,      // Transform precision
    removeUseless: true,        // Remove redundant commands
    collapseRepeated: true,     // Collapse repeated commands
    utilizeAbsolute: true,      // Convert to absolute when shorter
    leadingZero: true,          // Keep leading zeros
    negativeExtraSpace: true    // Add space before negative values
  }
}
```

---

## Environment Configuration

### Environment Variables

```bash
# Set default precision
export VEXY_SVGO_PRECISION=2

# Default config file location
export VEXY_SVGO_CONFIG=/path/to/config.js

# Disable colors in output
export NO_COLOR=1
```

### Project Settings

Create `.vexysvgorc` in your project root:

```json
{
  "precision": 2,
  "multipass": true,
  "configFile": "tools/svgo.config.js"
}
```

---

## Migration from SVGO

### Configuration Compatibility

Most SVGO configurations work directly:

```bash
# Your existing SVGO config should work
vexy-svgo --config .svgo.config.js input.svg
```

### Key Differences

| SVGO | Vexy SVGO | Notes |
|------|-----------|-------|
| `precision` | `floatPrecision` | Both supported |
| Plugin order | Plugin order | May differ slightly |
| Custom plugins | Not supported | Rust plugins only |

### Migration Checklist

1. ✅ Copy your `.svgo.config.js`
2. ✅ Test with sample files
3. ⚠️ Check output differences
4. ⚠️ Update custom plugins (if any)
5. ✅ Update build scripts

---

## Troubleshooting

### Configuration Not Found

```bash
# Check current directory
ls -la .svgo.config.*

# Check config discovery
vexy-svgo --verbose input.svg
```

### Plugin Errors

```bash
# List available plugins
vexy-svgo --show-plugins

# Test individual plugins
vexy-svgo --disable-all --enable removeComments input.svg
```

### Output Differences

```bash
# Compare with SVGO
svgo input.svg -o svgo-output.svg
vexy-svgo input.svg -o vexy-output.svg
diff svgo-output.svg vexy-output.svg
```

---

## Next Steps

- [Plugins](/plugins/) - Detailed plugin documentation
- [CLI Usage](/user/cli-usage/) - Command-line reference
- [Interactive Demo](/demo/) - Test configurations online

---

*Need help? Check our [GitHub Discussions](https://github.com/vexyart/vexy-svgo/discussions) for configuration examples and community support.*