---
nav_weight: 34
# this_file: docs/developer/plugin-reference.md
layout: default
title: Plugin Implementation Reference
parent: Developer Guide
nav_order: 4
description: "Complete reference for all SVGO plugins and their implementation status"
---

# Plugin Implementation Reference
{: .no_toc }

Complete reference for all SVGO plugins and their implementation status in Vexy SVGO
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Overview

This document provides a comprehensive analysis of all SVGO plugins from the reference implementation, including their descriptions, default preset status, parameters, and implementation status in Vexy SVGO.

---

## Plugin Classification

### Default Preset Plugins (35 plugins)
These plugins are included in the SVGO default preset and are enabled by default:

| Plugin Name | Implementation Status | Description | Parameters |
|-------------|----------------------|-------------|------------|
| **removeDoctype** | ✅ **Implemented** | removes doctype declarations | None |
| **removeXMLProcInst** | ✅ **Implemented** | removes XML processing instructions | None |
| **removeComments** | ✅ **Implemented** | removes comments | `preservePatterns` (RegExp[] or false) |
| **removeDeprecatedAttrs** | ✅ **Implemented** | removes deprecated attributes | None |
| **removeMetadata** | ✅ **Implemented** | removes `<metadata>` elements | None |
| **removeEditorsNSData** | ✅ **Implemented** | removes editor-specific namespaces, elements, and attributes | None |
| **cleanupAttrs** | ✅ **Implemented** | cleanups attributes from newlines, trailing and repeating spaces | `newlines`, `trim`, `spaces` (boolean) |
| **mergeStyles** | ✅ **Implemented** | merge multiple style elements into one | None |
| **inlineStyles** | ❌ **Not Implemented** | inline styles (additional options) | `onlyMatchedOnce`, `removeMatchedSelectors`, `useMqs`, `usePseudos` |
| **minifyStyles** | ✅ **Implemented** | minifies styles and removes unused styles | `restructure`, `forceMediaMerge`, `comments`, `usage` |
| **cleanupIds** | ✅ **Implemented** | removes unused IDs and minifies used | `remove`, `minify`, `preserve`, `preservePrefixes`, `force` |
| **removeUselessDefs** | ✅ **Implemented** | removes elements in `<defs>` without an `id` | None |
| **cleanupNumericValues** | ✅ **Implemented** | rounds numeric values to the fixed precision, removes default "px" units | `floatPrecision`, `leadingZero`, `defaultPx`, `convertToPx` |
| **convertColors** | ✅ **Implemented** | converts colors: rgb() to #rrggbb and #rrggbb to #rgb | `currentColor`, `names2hex`, `rgb2hex`, `convertCase`, `shorthex`, `shortname` |
| **removeUnknownsAndDefaults** | ✅ **Implemented** | removes unknown elements content and attributes, removes attrs with default values | `unknownContent`, `unknownAttrs`, `defaultAttrs`, `defaultMarkupDeclarations`, `uselessOverrides`, `keepDataAttrs`, `keepAriaAttrs`, `keepRoleAttr` |
| **removeNonInheritableGroupAttrs** | ✅ **Implemented** | removes non-inheritable group's "presentation" attributes | None |
| **removeUselessStrokeAndFill** | ✅ **Implemented** | removes useless `stroke` and `fill` attributes | None |
| **cleanupEnableBackground** | ✅ **Implemented** | remove or cleanup enable-background attribute when possible | None |
| **removeHiddenElems** | ✅ **Implemented** | removes hidden elements (`display="none"` or `visibility="hidden"`) | None |
| **removeEmptyText** | ✅ **Implemented** | removes empty text elements | None |
| **convertShapeToPath** | ✅ **Implemented** | converts basic shapes to more compact path form | `convertArcs`, `floatPrecision` |
| **convertEllipseToCircle** | ✅ **Implemented** | converts non-eccentric `<ellipse>`s to `<circle>`s | None |
| **moveElemsAttrsToGroup** | ❌ **Not Implemented** | Move common attributes of group children to the group | None |
| **moveGroupAttrsToElems** | ❌ **Not Implemented** | moves some group attributes to the content elements | None |
| **collapseGroups** | ✅ **Implemented** | collapses useless groups | None |
| **convertPathData** | ✅ **Implemented** | optimizes path data: writes in shorter form, applies transformations | *[See detailed parameters below](#convertpathdata-parameters)* |
| **convertTransform** | ❌ **Not Implemented** | collapses multiple transformations and optimizes it | `convertToShorts`, `degPrecision`, `floatPrecision`, `transformPrecision`, `matrixToTransform`, `shortTranslate`, `shortScale`, `shortRotate`, `removeUseless`, `collapseIntoOne`, `leadingZero`, `negativeExtraSpace` |
| **removeEmptyAttrs** | ✅ **Implemented** | removes empty attributes | None |
| **removeEmptyContainers** | ✅ **Implemented** | removes empty container elements | None |
| **mergePaths** | ❌ **Not Implemented** | merges multiple paths in one if possible | `force`, `floatPrecision`, `noSpaceAfterFlags` |
| **removeUnusedNS** | ✅ **Implemented** | removes unused namespace declarations | None |
| **sortAttrs** | ✅ **Implemented** | Sort element attributes for better compression | `order`, `xmlnsOrder` |
| **sortDefsChildren** | ✅ **Implemented** | Sorts children of `<defs>` to improve compression | None |
| **removeDesc** | ✅ **Implemented** | removes `<desc>` | `removeAny` |

### Non-Default Plugins (18 plugins)
These plugins are available but not included in the default preset:

| Plugin Name | Implementation Status | Description | Parameters |
|-------------|----------------------|-------------|------------|
| **addAttributesToSVGElement** | ✅ **Implemented** | adds attributes to the root `<svg>` element | `attributes` |
| **addClassesToSVGElement** | ✅ **Implemented** | adds class names to the root `<svg>` element | `classNames` |
| **cleanupListOfValues** | ✅ **Implemented** | rounds numeric values in attributes that have a list of numbers | `floatPrecision`, `leadingZero` |
| **convertOneStopGradients** | ✅ **Implemented** | converts one-stop gradients to a plain color | None |
| **prefixIds** | ✅ **Implemented** | prefix IDs and class names | `prefix`, `prefixIds`, `prefixClassNames` |
| **removeAttrs** | ✅ **Implemented** | removes attributes by pattern | `attrs` |
| **removeAttributesBySelector** | ✅ **Implemented** | removes attributes that match CSS selectors | `selectors` |
| **removeDimensions** | ✅ **Implemented** | removes width/height attributes (preserves viewBox) | None |
| **removeElementsByAttr** | ✅ **Implemented** | removes arbitrary elements by ID or class | `id`, `class` |
| **removeOffCanvasPaths** | ✅ **Implemented** | removes elements that are drawn outside of the viewBox | None |
| **removeRasterImages** | ✅ **Implemented** | removes raster images | None |
| **removeScriptElement** | ✅ **Implemented** | removes `<script>` elements | None |
| **removeStyleElement** | ✅ **Implemented** | removes `<style>` elements | None |
| **removeTitle** | ✅ **Implemented** | removes `<title>` elements | None |
| **removeUselessTransforms** | ✅ **Implemented** | removes identity transforms | None |
| **removeViewBox** | ✅ **Implemented** | removes viewBox when possible | None |
| **removeXlink** | ✅ **Implemented** | removes deprecated xlink attributes | None |
| **removeXMLNS** | ✅ **Implemented** | removes xmlns attribute from root element | None |

---

## Implementation Statistics

- **Total SVGO Plugins:** 53
- **Fully Implemented:** 48 (90.6%)
- **Not Yet Implemented:** 5 (9.4%)

### Missing Plugins (High Priority)
1. **inlineStyles** - CSS inlining functionality
2. **moveElemsAttrsToGroup** - Attribute optimization
3. **moveGroupAttrsToElems** - Attribute optimization  
4. **convertTransform** - Transform optimization
5. **mergePaths** - Path merging optimization

---

## Detailed Plugin Parameters

### convertPathData Parameters

This is one of the most complex plugins with extensive parameter options:

```rust
pub struct ConvertPathDataParams {
    pub apply_transforms: bool,              // Apply transform matrices to path coordinates
    pub apply_transforms_stroked: bool,      // Apply transforms to stroked paths
    pub make_arcs: bool,                     // Convert smooth curves to arcs where possible
    pub straight_curves: bool,               // Convert Bezier curves to lines when appropriate
    pub line_shorthands: bool,               // Use shorthand commands (H, V instead of L)
    pub curve_smooth_shorthands: bool,       // Use smooth curve commands (S, T)
    pub float_precision: u8,                 // Coordinate precision (default: 3)
    pub transform_precision: u8,             // Transform matrix precision (default: 5)
    pub remove_useless: bool,                // Remove redundant commands
    pub collapse_repeated: bool,             // Collapse repeated commands
    pub utilize_absolute: bool,              // Use absolute coordinates when shorter
    pub leading_zero: bool,                  // Keep leading zeros in numbers
    pub negative_extra_space: bool,          // Add space before negative values
}
```

### cleanupIds Parameters

```rust
pub struct CleanupIdsParams {
    pub remove: bool,                        // Remove unused IDs
    pub minify: bool,                        // Minify used IDs (shorten them)
    pub preserve: Vec<String>,               // IDs to never remove/modify
    pub preserve_prefixes: Vec<String>,      // ID prefixes to preserve
    pub force: bool,                         // Force removal even if referenced
}
```

### convertColors Parameters

```rust
pub struct ConvertColorsParams {
    pub current_color: bool,                 // Convert colors to currentColor
    pub names2hex: bool,                     // Named colors to hex (#red → #ff0000)
    pub rgb2hex: bool,                       // RGB values to hex (rgb(255,0,0) → #ff0000)
    pub convert_case: String,                // Color case: "lower" | "upper"
    pub shorthex: bool,                      // Long hex to short (#ffffff → #fff)
    pub shortname: bool,                     // Hex to named colors (#ff0000 → #red)
}
```

---

## Plugin Architecture

### Plugin Trait

All plugins implement the core `Plugin` trait:

```rust
pub trait Plugin {
    fn metadata(&self) -> PluginMetadata;
    fn optimize(&mut self, document: &mut Document) -> Result<()>;
}

pub struct PluginMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub tags: Vec<&'static str>,
    pub experimental: bool,
}
```

### Visitor Pattern

Most plugins use the visitor pattern for AST traversal:

```rust
pub trait Visitor {
    fn visit_document(&mut self, doc: &mut Document) -> Result<()>;
    fn visit_element(&mut self, element: &mut Element) -> Result<()>;
    fn visit_attribute(&mut self, attr: &mut Attribute) -> Result<()>;
}
```

---

## Implementation Guidelines

### For Contributors

When implementing missing plugins:

1. **Study SVGO implementation** - Reference the original JavaScript
2. **Maintain API compatibility** - Same parameters and behavior
3. **Add comprehensive tests** - Test edge cases and compatibility
4. **Document thoroughly** - Parameters, examples, and limitations
5. **Follow Rust patterns** - Use Result types, proper error handling

### Plugin Development Steps

1. **Create plugin struct**
```rust
pub struct MyPlugin {
    params: MyPluginParams,
}
```

2. **Implement Plugin trait**
```rust
impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata { /* ... */ }
    fn optimize(&mut self, document: &mut Document) -> Result<()> { /* ... */ }
}
```

3. **Add tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_plugin() {
        // Test implementation
    }
}
```

4. **Register plugin**
```rust
// In plugin registry
registry.register("myPlugin", Box::new(MyPlugin::new));
```

---

## Migration from SVGO

### JavaScript to Rust Patterns

| SVGO (JavaScript) | Vexy SVGO (Rust) |
|-------------------|-------------------|
| `exports.fn = (root, params) => {}` | `impl Plugin for MyPlugin {}` |
| Dynamic typing | Strong static typing |
| Callback-based AST traversal | Visitor pattern |
| Runtime parameter validation | Compile-time type safety |
| Exception handling | Result<T, Error> pattern |

### Common Migration Challenges

1. **String handling** - UTF-8 safety and performance
2. **Regex patterns** - Different regex engine capabilities  
3. **Floating point precision** - Consistent numeric handling
4. **Error propagation** - Rust's Result pattern vs exceptions
5. **Memory management** - Ownership and borrowing

---

## Performance Characteristics

### High-Performance Plugins
- `removeComments` - Simple string operations
- `removeEmptyAttrs` - Attribute filtering
- `sortAttrs` - In-place sorting

### Medium-Performance Plugins  
- `convertColors` - String parsing and conversion
- `cleanupNumericValues` - Numeric operations
- `collapseGroups` - Structural analysis

### Complex Plugins
- `convertPathData` - Path geometry operations
- `mergePaths` - Advanced path analysis
- `inlineStyles` - CSS parsing and application

---

## Testing Strategy

### Unit Tests
Each plugin has comprehensive unit tests covering:
- Basic functionality
- Edge cases
- Parameter variations
- Error conditions

### Integration Tests
Plugin combinations are tested for:
- Order-dependent behavior
- Plugin interactions
- Performance regression

### Compatibility Tests
SVGO comparison tests ensure:
- Identical output for same inputs
- Parameter compatibility
- Error handling consistency

---

## Future Roadmap

### Phase 1: Complete Core Plugins
- ✅ Implement `inlineStyles` 
- ✅ Implement `moveElemsAttrsToGroup`
- ✅ Implement `moveGroupAttrsToElems`

### Phase 2: Advanced Optimizations
- ✅ Implement `convertTransform`
- ✅ Implement `mergePaths`
- ✅ Add `reusePaths` plugin

### Phase 3: Performance Optimizations
- SIMD-optimized path operations
- Parallel plugin execution
- Streaming large file support

### Phase 4: Extensions
- Custom plugin development framework
- Visual plugin debugging tools
- Plugin marketplace integration

---

## Contributing

### Getting Started

1. **Choose a missing plugin** from the list above
2. **Study the SVGO implementation** in `ref/svgo/plugins/`
3. **Create plugin structure** in `crates/plugin-sdk/src/plugins/`
4. **Write comprehensive tests** in the test module
5. **Add documentation** and examples

### Code Review Process

1. **Functionality review** - Correctness vs SVGO
2. **Performance review** - Benchmarks and profiling
3. **API review** - Parameter compatibility
4. **Documentation review** - Examples and clarity

---

## Resources

- [SVGO Plugin Reference](https://github.com/svg/svgo#built-in-plugins)
- [Vexy SVGO Plugin SDK](/developer/plugin-development/)
- [Contributing Guide](/developer/contributing/)
- [Performance Benchmarks](/developer/benchmarks/)

---

*This reference is automatically updated as new plugins are implemented. Last updated: 2024-07-12*