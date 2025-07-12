---
nav_weight: 32
---

# Plugin Development Tutorial

This comprehensive guide will walk you through creating custom plugins for Vexy SVGO, from basic concepts to advanced techniques.

## Table of Contents

1. [Introduction](#introduction)
2. [Plugin Architecture](#plugin-architecture)
3. [Setting Up Your Development Environment](#setting-up-your-development-environment)
4. [Creating Your First Plugin](#creating-your-first-plugin)
5. [Advanced Plugin Development](#advanced-plugin-development)
6. [Testing Your Plugin](#testing-your-plugin)
7. [Publishing and Distribution](#publishing-and-distribution)
8. [Best Practices](#best-practices)

## Introduction

Vexy SVGO plugins are Rust modules that implement the `Plugin` trait from the `vexy-svgo-plugin-sdk`. They can traverse and modify the SVG AST (Abstract Syntax Tree) to perform optimizations.

### Why Create Plugins?

- **Custom Optimizations**: Implement domain-specific optimizations
- **Extended Functionality**: Add features beyond standard SVGO compatibility
- **Performance**: Native Rust performance for complex transformations
- **Integration**: Seamlessly integrate with existing Vexy SVGO workflows

## Plugin Architecture

### Core Concepts

1. **AST (Abstract Syntax Tree)**: The parsed representation of an SVG document
2. **Visitor Pattern**: Plugins traverse the AST using the visitor pattern
3. **Plugin Trait**: The interface all plugins must implement
4. **Configuration**: Plugins can accept parameters for customization

### Plugin Lifecycle

```
SVG Input → Parser → AST → Plugin Pipeline → Modified AST → Stringifier → Optimized SVG
```

## Setting Up Your Development Environment

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the Vexy SVGO repository
git clone https://github.com/vexyart/vexy-svgo
cd vexy-svgo

# Build the project
./build.sh
```

### Project Structure

Create a new directory for your plugin:

```bash
mkdir -p crates/plugin-sdk/src/plugins/my_custom_plugin
cd crates/plugin-sdk/src/plugins/my_custom_plugin
```

## Creating Your First Plugin

Let's create a simple plugin that adds a custom attribute to all `<rect>` elements.

### Step 1: Create the Plugin Module

Create `crates/plugin-sdk/src/plugins/my_custom_plugin/mod.rs`:

```rust
// this_file: crates/plugin-sdk/src/plugins/my_custom_plugin/mod.rs

use vexy-svgo-plugin-sdk::{Plugin, PluginMetadata};
use vexy-svgo-core::visitor::{Visitor, VisitorContext};
use vexy-svgo-core::ast::{Document, Element, Node};
use anyhow::Result;
use serde::{Deserialize, Serialize};

// ... (plugin implementation)
```

### Step 2: Register the Plugin

Add your plugin to `crates/plugin-sdk/src/plugins/mod.rs`:

```rust
// Add to the plugin modules
pub mod my_custom_plugin;

// In the create_default_registry function, add:
registry.register(Box::new(my_custom_plugin::MyCustomPlugin::new()));
```

### Step 3: Build and Test

```bash
# Run tests
cargo test my_custom_plugin

# Build the project
cargo build --release

# Test with CLI
echo '<svg><rect x="0" y="0" width="100" height="100"/></svg>' | \
  target/release/vexy-svgo --enable myCustomPlugin
```

## Advanced Plugin Development

### Working with Complex AST Transformations

Here's a more advanced plugin that optimizes gradients:

```rust
// this_file: crates/plugin-sdk/src/plugins/optimize_gradients/mod.rs

use vexy-svgo-plugin-sdk::Plugin;
use vexy-svgo-core::visitor::{Visitor, VisitorContext};
use vexy-svgo-core::ast::{Document, Element, Node};
use std::collections::{HashMap, HashSet};

pub struct OptimizeGradientsPlugin { // ... }

impl Plugin for OptimizeGradientsPlugin {
    fn metadata(&self) -> vexy-svgo-plugin-sdk::PluginMetadata {
        vexy-svgo-plugin-sdk::PluginMetadata {
            name: "optimizeGradients".to_string(),
            description: "Removes duplicate gradients and unused gradients".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Vexy SVGO Team".to_string()),
            tags: vec!["gradients", "optimization", "cleanup"],
            experimental: false,
        }
    }
    
    fn optimize(&mut self, document: &mut Document) -> anyhow::Result<()> {
        // ...
        Ok(())
    }
}
```

### Plugin Communication

Plugins can share data through the visitor context:

```rust
use vexy-svgo-core::visitor::{VisitorContext, ContextData};

impl<'a> Visitor<'a> for MyPlugin {
    fn visit_element_enter(
        &mut self, 
        element: &mut Element<'a>,
        context: &mut VisitorContext
    ) -> Result<()> {
        // ...
        Ok(())
    }
}
```

## Testing Your Plugin

### Unit Testing

Create comprehensive tests in your plugin module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use vexy-svgo-test-utils::{assert_svg_eq, optimize_with_plugin};
    
    #[test]
    fn test_removes_duplicate_gradients() {
        // ...
    }
}
```

### Integration Testing

Test your plugin with the full Vexy SVGO pipeline:

```rust
#[test]
fn test_plugin_in_pipeline() {
    let config = Config {
        plugins: vec![
            PluginConfig::Name("removeComments".to_string()),
            PluginConfig::Name("myCustomPlugin".to_string()),
            PluginConfig::WithParams {
                name: "optimizeGradients".to_string(),
                params: json!({"removeUnused": true}),
            },
        ],
        ..Default::default()
    };
    
    let result = optimize_with_config(svg_input, config).unwrap();
    // Assert expectations
}
```

### Performance Testing

```rust
#[bench]
fn bench_large_svg_optimization(b: &mut Bencher) {
    // ...
}
```

## Publishing and Distribution

### 1. Package Your Plugin

Create a separate crate for distribution:

```toml
# Cargo.toml
[package]
name = "vexy-svgo-plugin-custom"
version = "0.1.0"
edition = "2021"

[dependencies]
vexy-svgo-plugin-sdk = "2.0"
vexy-svgo-core = "2.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
vexy-svgo-test-utils = "2.0"
```

### 2. Documentation

Document your plugin thoroughly:

```rust
//! # Custom Vexy SVGO Plugin
//!
//! This plugin adds custom attributes to SVG elements based on configurable rules.
//!
//! ## Usage
//!
//! ```rust
//! use vexy-svgo-plugin-custom::MyCustomPlugin;
//! use vexy-svgo-core::Config;
//!
//! let mut config = Config::default();
//! config.plugins.push(PluginConfig::Name("myCustomPlugin".to_string()));
//! ```
//!
//! ## Configuration
//!
//! The plugin accepts the following parameters:
//!
//! - `attribute_name`: The name of the attribute to add (default: "data-custom")
//! - `attribute_value`: The value of the attribute (default: "processed")
//! - `target_class`: Optional class name to filter elements
```

### 3. Publish to crates.io

```bash
# Ensure tests pass
cargo test

# Check documentation
cargo doc --open

# Publish
cargo publish
```


## Best Practices

### 1. Performance Optimization

- **Minimize Allocations**: Reuse strings and collections where possible
- **Early Returns**: Skip processing when conditions aren't met
- **Batch Operations**: Group similar modifications together

```rust
impl<'a> Visitor<'a> for EfficientPlugin {
    fn visit_element_enter(&mut self, element: &mut Element<'a>) -> Result<()> {
        // ...
        Ok(())
    }
}
```

### 2. Error Handling

- **Graceful Degradation**: Don't break the entire optimization on minor errors
- **Informative Messages**: Provide context in error messages
- **Recovery Strategies**: Implement fallbacks for common issues

```rust
fn process_element(&mut self, element: &mut Element) -> Result<()> {
    // ...
}
```

### 3. Configuration Validation

```rust
impl Plugin for ValidatedPlugin {
    fn configure(&mut self, params: serde_json::Value) -> Result<()> {
        // ...
        Ok(())
    }
}
```

### 4. Compatibility

- **SVGO Parity**: Match SVGO behavior when implementing equivalent plugins
- **Backward Compatibility**: Don't break existing configurations
- **Feature Detection**: Check for optional dependencies

```rust
impl Plugin for CompatiblePlugin {
    fn optimize(&mut self, document: &mut Document) -> Result<()> {
        // ...
    }
}
```

## Plugin Ideas

Here are some plugin ideas to inspire your development:

1. **Accessibility Enhancer**: Add ARIA labels and roles automatically
2. **Animation Optimizer**: Optimize SMIL animations and CSS animations
3. **Icon Sprite Generator**: Combine multiple icons into a sprite sheet
4. **Responsive SVG**: Add viewBox and preserveAspectRatio for responsive designs
5. **Security Scanner**: Detect and remove potentially malicious content
6. **Style Optimizer**: Convert inline styles to CSS classes
7. **Path Simplifier**: Advanced path optimization beyond convertPathData
8. **Metadata Manager**: Add/update copyright, license, and attribution
9. **Theme Converter**: Convert colors to CSS variables for theming
10. **Performance Profiler**: Add timing marks for animation performance

## Resources

- [Vexy SVGO Plugin SDK Documentation](https://docs.rs/vexy-svgo-plugin-sdk)
- [SVG Specification](https://www.w3.org/TR/SVG2/)
- [SVGO Plugin Reference](https://github.com/svg/svgo#plugins)
- [Rust Visitor Pattern Guide](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)

## Conclusion

Creating Vexy SVGO plugins allows you to extend the optimizer with custom functionality while maintaining the performance benefits of Rust. Start with simple transformations and gradually build more complex optimizations as you become familiar with the AST and visitor pattern.

Remember to:
- Test thoroughly with various SVG inputs
- Document your plugin's behavior and configuration
- Consider performance implications
- Share your plugins with the community

Happy plugin development! 🚀