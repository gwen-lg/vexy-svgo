---
nav_weight: 24
# this_file: docs/user/plugins.md
layout: default
title: Plugin Reference
parent: User Guide
nav_order: 5
description: "Complete reference of Vexy SVGO optimization plugins"
---

# Plugin Reference
{: .no_toc }

Complete reference of Vexy SVGO optimization plugins
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Overview

Plugins are the core of Vexy SVGO's optimization capabilities. Each plugin performs a specific transformation on the SVG's Abstract Syntax Tree (AST) to reduce file size and improve rendering efficiency.

Vexy SVGO maintains full compatibility with SVGO's plugin system while delivering significant performance improvements through native Rust implementation.

---

## Plugin Categories

### Basic Optimization Plugins

These plugins handle fundamental SVG cleanup tasks:

#### `removeComments`
**Description:** Removes comments from SVG (preserves legal comments starting with `!`)
**Default:** ✅ Enabled
**Parameters:** None

```bash
# Disable in CLI
vexy-svgo --disable removeComments input.svg
```

#### `removeDoctype` 
**Description:** Removes doctype declarations
**Default:** ✅ Enabled
**Parameters:** None

#### `removeXMLProcInst`
**Description:** Removes XML processing instructions
**Default:** ✅ Enabled
**Parameters:** None

#### `removeMetadata`
**Description:** Removes `<metadata>` elements
**Default:** ✅ Enabled
**Parameters:** None

#### `removeTitle`
**Description:** Removes `<title>` elements
**Default:** ❌ Disabled (accessibility)
**Parameters:** None

#### `removeDesc`
**Description:** Removes `<desc>` elements  
**Default:** ✅ Enabled
**Parameters:** None

---

### Attribute Optimization

#### `cleanupAttrs`
**Description:** Cleans up attributes from newlines, trailing, and repeating spaces
**Default:** ✅ Enabled
**Parameters:**
- `newlines` (boolean) - Remove newlines
- `trim` (boolean) - Trim whitespace  
- `spaces` (boolean) - Collapse spaces

#### `removeEmptyAttrs`
**Description:** Removes empty attributes
**Default:** ✅ Enabled
**Parameters:** None

#### `removeUnknownsAndDefaults`
**Description:** Removes unknown elements and default values
**Default:** ✅ Enabled
**Parameters:**
- `unknownContent` (boolean)
- `unknownAttrs` (boolean) 
- `defaultAttrs` (boolean)
- `keepDataAttrs` (boolean)
- `keepAriaAttrs` (boolean)

#### `sortAttrs`
**Description:** Sorts element attributes for better compression
**Default:** ✅ Enabled
**Parameters:**
- `order` (string[]) - Custom attribute order
- `xmlnsOrder` (string) - XML namespace order

---

### Style and Color Optimization

#### `convertColors`
**Description:** Converts colors to optimal format (rgb→hex, names→hex)
**Default:** ✅ Enabled
**Parameters:**
- `currentColor` (boolean) - Convert to currentColor
- `names2hex` (boolean) - Named colors to hex
- `rgb2hex` (boolean) - RGB to hex
- `shorthex` (boolean) - #rrggbb to #rgb
- `shortname` (boolean) - Hex to named colors

```javascript
// Example configuration
{
  name: 'convertColors',
  params: {
    currentColor: true,
    names2hex: true,
    rgb2hex: true,
    shorthex: true
  }
}
```

#### `mergeStyles`
**Description:** Merges multiple `<style>` elements into one
**Default:** ✅ Enabled
**Parameters:** None

#### `minifyStyles`
**Description:** Basic CSS minification
**Default:** ✅ Enabled
**Parameters:**
- `restructure` (boolean)
- `comments` (string|boolean)

#### `convertStyleToAttrs`
**Description:** Converts styles to presentation attributes
**Default:** ✅ Enabled
**Parameters:** None

---

### Numeric Value Optimization

#### `cleanupNumericValues`
**Description:** Rounds numeric values to fixed precision, removes default units
**Default:** ✅ Enabled
**Parameters:**
- `floatPrecision` (number) - Decimal precision (default: 3)
- `leadingZero` (boolean) - Keep leading zeros
- `defaultPx` (boolean) - Remove default px units

```javascript
{
  name: 'cleanupNumericValues',
  params: {
    floatPrecision: 2,
    leadingZero: false,
    defaultPx: true
  }
}
```

---

### Structure Optimization

#### `removeEmptyContainers`
**Description:** Removes empty container elements
**Default:** ✅ Enabled
**Parameters:** None

#### `removeEmptyText`
**Description:** Removes empty text elements
**Default:** ✅ Enabled
**Parameters:** None

#### `collapseGroups`
**Description:** Collapses useless groups (`<g>`)
**Default:** ✅ Enabled
**Parameters:** None

#### `removeUselessDefs`
**Description:** Removes `<defs>` elements without IDs
**Default:** ✅ Enabled
**Parameters:** None

#### `removeHiddenElems`
**Description:** Removes hidden elements (display:none, visibility:hidden)
**Default:** ✅ Enabled
**Parameters:** None

---

### Shape Conversion

#### `convertShapeToPath`
**Description:** Converts basic shapes to `<path>` elements
**Default:** ✅ Enabled
**Parameters:**
- `convertArcs` (boolean) - Convert arcs
- `floatPrecision` (number) - Coordinate precision

#### `convertEllipseToCircle`
**Description:** Converts `<ellipse>` to `<circle>` when possible
**Default:** ✅ Enabled
**Parameters:** None

---

### Path Optimization

#### `convertPathData` ⭐
**Description:** Comprehensive path optimization - converts coordinates, removes redundant commands
**Default:** ✅ Enabled
**Parameters:**
- `applyTransforms` (boolean) - Apply transform matrices
- `makeArcs` (boolean) - Convert to arcs where possible
- `straightCurves` (boolean) - Convert curves to lines
- `lineShorthands` (boolean) - Use H/V commands
- `floatPrecision` (number) - Coordinate precision
- `removeUseless` (boolean) - Remove redundant commands
- `collapseRepeated` (boolean) - Collapse repeated commands

```javascript
{
  name: 'convertPathData',
  params: {
    floatPrecision: 3,
    applyTransforms: true,
    makeArcs: true,
    straightCurves: true,
    lineShorthands: true,
    removeUseless: true
  }
}
```

---

### ID and Reference Management

#### `cleanupIds`
**Description:** Minifies and removes unused IDs
**Default:** ✅ Enabled  
**Parameters:**
- `remove` (boolean) - Remove unused IDs
- `minify` (boolean) - Minify used IDs
- `prefix` (string) - Add prefix to IDs
- `preserve` (string[]) - IDs to never remove

```javascript
{
  name: 'cleanupIds',
  params: {
    remove: true,
    minify: true,
    prefix: 'icon-',
    preserve: ['logo', 'main-graphic']
  }
}
```

#### `removeUnusedNS`
**Description:** Removes unused namespace declarations
**Default:** ✅ Enabled
**Parameters:** None

---

## Advanced Plugins

### Transform Management

#### `removeUselessStrokeAndFill`
**Description:** Removes unnecessary stroke and fill attributes
**Default:** ✅ Enabled
**Parameters:** None

#### `cleanupEnableBackground`
**Description:** Removes or cleans up enable-background attribute
**Default:** ✅ Enabled
**Parameters:** None

#### `removeNonInheritableGroupAttrs`
**Description:** Removes non-inheritable group attributes
**Default:** ✅ Enabled
**Parameters:** None

---

## Not Yet Implemented

These complex plugins require additional work:

### `mergePaths`
**Description:** Merge multiple paths into one
**Status:** ⏳ Coming soon
**Complexity:** High - requires path geometry analysis

### `moveElemsAttrsToGroup`
**Description:** Move common attributes to parent group
**Status:** ⏳ Coming soon
**Complexity:** Medium - requires attribute inheritance analysis

### `moveGroupAttrsToElems` 
**Description:** Move group attributes to child elements
**Status:** ⏳ Coming soon
**Complexity:** Medium - requires attribute distribution logic

### `convertTransform`
**Description:** Optimizes transform attributes
**Status:** ⏳ Coming soon
**Complexity:** High - requires matrix operations

### `applyTransforms`
**Description:** Applies transform matrices to coordinates
**Status:** ⏳ Coming soon
**Complexity:** Very High - requires geometric transformations

---

## Configuration Examples

### Web Development
```javascript
{
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
    { name: 'removeViewBox', enabled: false }
  ]
}
```

### Maximum Compression
```javascript
{
  multipass: true,
  plugins: [
    'removeComments',
    'removeTitle',
    'removeDesc',
    'removeEmptyAttrs',
    'cleanupAttrs',
    'convertColors',
    'convertPathData',
    'collapseGroups',
    {
      name: 'cleanupNumericValues', 
      params: { floatPrecision: 1 }
    }
  ]
}
```

### Icon Library
```javascript
{
  plugins: [
    'removeComments',
    'removeEmptyAttrs',
    'cleanupAttrs',
    'convertColors',
    {
      name: 'cleanupIds',
      params: { remove: false, minify: true }
    },
    // Preserve structure for icons
    { name: 'removeViewBox', enabled: false },
    { name: 'collapseGroups', enabled: false }
  ]
}
```

---

## CLI Usage

### List Available Plugins
```bash
vexy-svgo --show-plugins
```

### Enable/Disable Plugins
```bash
# Disable specific plugins
vexy-svgo --disable removeComments --disable removeTitle input.svg

# Enable additional plugins  
vexy-svgo --enable cleanupIds --enable sortAttrs input.svg
```

### Plugin-Specific Configuration
```bash
# Use config file for complex plugin settings
vexy-svgo --config plugins.config.js input.svg
```

---

## Migration from SVGO

### Plugin Compatibility

| SVGO Plugin | Vexy SVGO | Status |
|-------------|-----------|--------|
| removeComments | ✅ removeComments | Identical |
| removeEmptyAttrs | ✅ removeEmptyAttrs | Identical |
| convertColors | ✅ convertColors | Compatible |
| cleanupIDs | ✅ cleanupIds | Same functionality |
| convertPathData | ✅ convertPathData | Enhanced |
| mergePaths | ⏳ Coming soon | Not yet available |

### Breaking Changes

1. **Plugin names:** Some plugins use camelCase instead of kebab-case
2. **Parameters:** Some parameter names may differ slightly
3. **Custom plugins:** JavaScript plugins need to be rewritten in Rust

---

## Performance Impact

### High Impact (Major size reduction)
- `convertPathData` - Path optimization
- `convertColors` - Color optimization  
- `cleanupNumericValues` - Precision reduction
- `removeComments` - Comment removal

### Medium Impact
- `removeEmptyAttrs` - Attribute cleanup
- `collapseGroups` - Structure optimization
- `cleanupIds` - ID optimization

### Low Impact (Quality of life)
- `sortAttrs` - Better compression
- `removeDoctype` - Minimal size gain
- `cleanupEnableBackground` - Edge case optimization

---

## Next Steps

- [Configuration](/user/configuration/) - Learn how to configure plugins
- [CLI Usage](/user/cli-usage/) - Command-line plugin control
- [Interactive Demo](/demo/) - Test plugins in your browser

---

*For the latest plugin status and implementation details, see our [GitHub repository](https://github.com/vexyart/vexy-svgo).*