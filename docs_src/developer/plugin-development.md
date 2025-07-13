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

Create `crates/plugin-sdk/src/plugins/my_custom_plugin.rs`:

```rust
// this_file: crates/plugin-sdk/src/plugins/my_custom_plugin.rs

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for the custom plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MyCustomPluginConfig {
    /// Attribute name to add
    #[serde(default = "default_attr_name")]
    pub attr_name: String,
    /// Attribute value to add
    #[serde(default = "default_attr_value")]  
    pub attr_value: String,
    /// Only process elements with these tag names
    #[serde(default)]
    pub target_elements: Vec<String>,
}

fn default_attr_name() -> String {
    "data-processed".to_string()
}

fn default_attr_value() -> String {
    "true".to_string()
}

impl Default for MyCustomPluginConfig {
    fn default() -> Self {
        Self {
            attr_name: default_attr_name(),
            attr_value: default_attr_value(),
            target_elements: vec!["rect".to_string()],
        }
    }
}

/// Plugin that adds custom attributes to specified elements
#[derive(Clone)]
pub struct MyCustomPlugin {
    config: MyCustomPluginConfig,
}

impl MyCustomPlugin {
    pub fn new() -> Self {
        Self {
            config: MyCustomPluginConfig::default(),
        }
    }

    pub fn with_config(config: MyCustomPluginConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<MyCustomPluginConfig> {
        if params.is_null() {
            Ok(MyCustomPluginConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    fn process_element(&self, element: &mut Element) {
        // Check if this element should be processed
        if self.config.target_elements.is_empty() 
           || self.config.target_elements.contains(&element.name.to_string()) {
            // Add the custom attribute
            element.attributes.insert(
                self.config.attr_name.clone(),
                self.config.attr_value.clone(),
            );
        }

        // Recursively process child elements
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.process_element(elem);
            }
        }
    }
}

impl Default for MyCustomPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MyCustomPlugin {
    fn name(&self) -> &'static str {
        "myCustomPlugin"
    }

    fn description(&self) -> &'static str {
        "adds custom attributes to specified elements"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.process_element(&mut document.root);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::borrow::Cow;

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    #[test]
    fn test_plugin_info() {
        let plugin = MyCustomPlugin::new();
        assert_eq!(plugin.name(), "myCustomPlugin");
        assert_eq!(plugin.description(), "adds custom attributes to specified elements");
    }

    #[test]
    fn test_adds_attribute_to_target_elements() {
        let plugin = MyCustomPlugin::new();
        let mut doc = Document::new();
        
        let mut svg = create_element("svg");
        svg.children.push(Node::Element(create_element("rect")));
        svg.children.push(Node::Element(create_element("circle")));
        doc.root = svg;

        plugin.apply(&mut doc).unwrap();

        // Check that rect got the attribute
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.attributes.get("data-processed"), Some(&"true".to_string()));
        }
        
        // Check that circle did not get the attribute (not in target_elements)
        if let Node::Element(circle) = &doc.root.children[1] {
            assert!(!circle.attributes.contains_key("data-processed"));
        }
    }

    #[test]
    fn test_custom_configuration() {
        let config = MyCustomPluginConfig {
            attr_name: "data-custom".to_string(),
            attr_value: "processed".to_string(),
            target_elements: vec!["circle".to_string()],
        };
        let plugin = MyCustomPlugin::with_config(config);
        
        let mut doc = Document::new();
        let mut svg = create_element("svg");
        svg.children.push(Node::Element(create_element("circle")));
        doc.root = svg;

        plugin.apply(&mut doc).unwrap();

        if let Node::Element(circle) = &doc.root.children[0] {
            assert_eq!(circle.attributes.get("data-custom"), Some(&"processed".to_string()));
        }
    }
}
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

Here's a more advanced plugin that optimizes gradients by removing duplicates:

```rust
// this_file: crates/plugin-sdk/src/plugins/optimize_gradients.rs

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptimizeGradientsConfig {
    /// Remove unused gradients
    #[serde(default = "default_true")]
    pub remove_unused: bool,
    /// Merge duplicate gradients
    #[serde(default = "default_true")]  
    pub merge_duplicates: bool,
}

fn default_true() -> bool { true }

impl Default for OptimizeGradientsConfig {
    fn default() -> Self {
        Self {
            remove_unused: true,
            merge_duplicates: true,
        }
    }
}

#[derive(Clone)]
pub struct OptimizeGradientsPlugin {
    config: OptimizeGradientsConfig,
}

impl OptimizeGradientsPlugin {
    pub fn new() -> Self {
        Self {
            config: OptimizeGradientsConfig::default(),
        }
    }

    fn collect_gradient_info(&self, document: &Document) -> (HashMap<String, String>, HashSet<String>) {
        let mut gradient_definitions = HashMap::new(); // id -> serialized gradient
        let mut used_gradients = HashSet::new();

        // Find all gradient definitions in <defs>
        self.find_gradients(&document.root, &mut gradient_definitions);
        
        // Find all gradient references 
        self.find_gradient_references(&document.root, &mut used_gradients);

        (gradient_definitions, used_gradients)
    }

    fn find_gradients(&self, element: &Element, gradients: &mut HashMap<String, String>) {
        if element.name == "defs" {
            for child in &element.children {
                if let Node::Element(elem) = child {
                    if matches!(elem.name.as_ref(), "linearGradient" | "radialGradient") {
                        if let Some(id) = elem.attributes.get("id") {
                            // Create a canonical representation of the gradient
                            let canonical = self.serialize_gradient(elem);
                            gradients.insert(id.clone(), canonical);
                        }
                    }
                }
            }
        }

        // Recursively search child elements
        for child in &element.children {
            if let Node::Element(elem) = child {
                self.find_gradients(elem, gradients);
            }
        }
    }

    fn find_gradient_references(&self, element: &Element, used: &mut HashSet<String>) {
        // Check fill and stroke attributes for url() references
        for attr_value in ["fill", "stroke"].iter().filter_map(|attr| element.attributes.get(*attr)) {
            if attr_value.starts_with("url(#") && attr_value.ends_with(')') {
                let id = &attr_value[5..attr_value.len()-1];
                used.insert(id.to_string());
            }
        }

        // Recursively search child elements
        for child in &element.children {
            if let Node::Element(elem) = child {
                self.find_gradient_references(elem, used);
            }
        }
    }

    fn serialize_gradient(&self, gradient: &Element) -> String {
        // Create a canonical string representation for comparison
        let mut parts = vec![gradient.name.to_string()];
        
        // Add sorted attributes (excluding id)
        let mut attrs: Vec<_> = gradient.attributes.iter()
            .filter(|(key, _)| *key != "id")
            .collect();
        attrs.sort_by_key(|(key, _)| *key);
        
        for (key, value) in attrs {
            parts.push(format!("{}={}", key, value));
        }

        // Add stops
        for child in &gradient.children {
            if let Node::Element(stop) = child {
                if stop.name == "stop" {
                    let mut stop_attrs: Vec<_> = stop.attributes.iter().collect();
                    stop_attrs.sort_by_key(|(key, _)| *key);
                    let stop_str = stop_attrs.iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join(" ");
                    parts.push(format!("stop[{}]", stop_str));
                }
            }
        }

        parts.join("|")
    }

    fn remove_unused_gradients(&self, document: &mut Document, used: &HashSet<String>) {
        self.remove_unused_from_element(&mut document.root, used);
    }

    fn remove_unused_from_element(&self, element: &mut Element, used: &HashSet<String>) {
        if element.name == "defs" {
            element.children.retain(|child| {
                if let Node::Element(elem) = child {
                    if matches!(elem.name.as_ref(), "linearGradient" | "radialGradient") {
                        if let Some(id) = elem.attributes.get("id") {
                            return used.contains(id);
                        }
                    }
                }
                true
            });
        }

        // Recursively process child elements
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.remove_unused_from_element(elem, used);
            }
        }
    }
}

impl Default for OptimizeGradientsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for OptimizeGradientsPlugin {
    fn name(&self) -> &'static str {
        "optimizeGradients"
    }

    fn description(&self) -> &'static str {
        "removes duplicate and unused gradients"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        if params.is_null() {
            Ok(())
        } else {
            serde_json::from_value::<OptimizeGradientsConfig>(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))?;
            Ok(())
        }
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        let (gradients, used) = self.collect_gradient_info(document);

        if self.config.remove_unused {
            self.remove_unused_gradients(document, &used);
        }

        if self.config.merge_duplicates {
            // TODO: Implement duplicate merging logic
            // This would involve finding gradients with identical canonical forms
            // and updating references to use a single gradient definition
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    #[test]
    fn test_removes_unused_gradients() {
        let plugin = OptimizeGradientsPlugin::new();
        let mut doc = Document::new();
        
        // Create SVG with unused gradient
        let mut svg = create_element("svg");
        let mut defs = create_element("defs");
        
        let mut used_gradient = create_element("linearGradient");
        used_gradient.attributes.insert("id".to_string(), "used".to_string());
        
        let mut unused_gradient = create_element("linearGradient");  
        unused_gradient.attributes.insert("id".to_string(), "unused".to_string());
        
        defs.children.push(Node::Element(used_gradient));
        defs.children.push(Node::Element(unused_gradient));
        
        let mut rect = create_element("rect");
        rect.attributes.insert("fill".to_string(), "url(#used)".to_string());
        
        svg.children.push(Node::Element(defs));
        svg.children.push(Node::Element(rect));
        doc.root = svg;

        plugin.apply(&mut doc).unwrap();

        // Check that unused gradient was removed
        if let Node::Element(defs_elem) = &doc.root.children[0] {
            assert_eq!(defs_elem.children.len(), 1);
            if let Node::Element(gradient) = &defs_elem.children[0] {
                assert_eq!(gradient.attributes.get("id"), Some(&"used".to_string()));
            }
        }
    }
}
```

### Plugin State and Context

Plugins can maintain internal state to coordinate complex optimizations:

```rust
pub struct StatefulPlugin {
    config: StatefulPluginConfig,
    // Maintain state between element visits
    collected_ids: HashSet<String>,
    id_references: HashMap<String, Vec<String>>,
}

impl Plugin for StatefulPlugin {
    fn apply(&self, document: &mut Document) -> Result<()> {
        // First pass: collect information
        self.collect_information(&document.root);
        
        // Second pass: apply optimizations based on collected data
        self.apply_optimizations(&mut document.root);
        
        Ok(())
    }
}
```

## Testing Your Plugin

### Unit Testing

Create comprehensive tests using the current testing framework:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::borrow::Cow;
    use vexy_svgo_core::ast::{Document, Element, Node};

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    #[test]
    fn test_plugin_basic_functionality() {
        let plugin = MyCustomPlugin::new();
        let mut doc = Document::new();
        
        // Create test SVG structure
        let mut svg = create_element("svg");
        svg.children.push(Node::Element(create_element("rect")));
        doc.root = svg;

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Verify expected changes
        if let Node::Element(rect) = &doc.root.children[0] {
            assert!(rect.attributes.contains_key("data-processed"));
            assert_eq!(rect.attributes.get("data-processed"), Some(&"true".to_string()));
        }
    }

    #[test]
    fn test_plugin_configuration() {
        let config = MyCustomPluginConfig {
            attr_name: "data-test".to_string(),
            attr_value: "yes".to_string(),
            target_elements: vec!["circle".to_string()],
        };
        let plugin = MyCustomPlugin::with_config(config);
        
        // Test with custom configuration
        let mut doc = Document::new();
        let mut svg = create_element("svg");
        svg.children.push(Node::Element(create_element("circle")));
        doc.root = svg;

        plugin.apply(&mut doc).unwrap();

        if let Node::Element(circle) = &doc.root.children[0] {
            assert_eq!(circle.attributes.get("data-test"), Some(&"yes".to_string()));
        }
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = MyCustomPlugin::new();

        // Valid parameters
        assert!(plugin.validate_params(&json!(null)).is_ok());
        assert!(plugin.validate_params(&json!({})).is_ok());
        assert!(plugin.validate_params(&json!({
            "attrName": "custom",
            "attrValue": "value",
            "targetElements": ["rect", "circle"]
        })).is_ok());

        // Invalid parameters
        assert!(plugin.validate_params(&json!({
            "invalidField": true
        })).is_err());
    }
}

// Use the plugin fixture testing framework for SVGO compatibility
// This macro automatically generates tests based on SVGO test fixtures
crate::plugin_fixture_tests!(MyCustomPlugin, "myCustomPlugin");
```

### SVGO Compatibility Testing

The `plugin_fixture_tests!` macro automatically generates compatibility tests based on the original SVGO test fixtures. These tests ensure your plugin behaves identically to the SVGO equivalent:

- **Automatic Test Generation**: Tests are created from `testdata/plugins/pluginName/*.txt` files
- **Input/Output Validation**: Each test compares your plugin's output with SVGO's expected output
- **Parameter Testing**: Tests various plugin configurations and edge cases
- **Regression Prevention**: Catches any behavioral differences from SVGO

To add fixture tests for your plugin:

1. Add your plugin test cases to `testdata/plugins/myCustomPlugin/`
2. Use the format: `01.txt`, `02.txt`, etc.
3. Each file contains input SVG, expected output, and optional configuration
4. Add the macro call: `crate::plugin_fixture_tests!(MyCustomPlugin, "myCustomPlugin");`

Example fixture file (`testdata/plugins/myCustomPlugin/01.txt`):
```
===
Add custom attribute to rect elements
===

<svg xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="100"/>
  <circle r="50"/>
</svg>

@@@

<svg xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="100" data-processed="true"/>
  <circle r="50"/>
</svg>
```

### Integration Testing

Test your plugin with the full Vexy SVGO pipeline:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use vexy_svgo_core::{optimize_with_config, Config, PluginConfig};
    use serde_json::json;

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
            multipass: false,
            ..Default::default()
        };

        let svg_input = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <!-- Comment to remove -->
            <defs>
                <linearGradient id="unused"/>
                <linearGradient id="used"/>
            </defs>
            <rect fill="url(#used)"/>
        </svg>"#;

        let result = optimize_with_config(svg_input, config).unwrap();
        
        // Verify all plugins ran correctly
        assert!(!result.data.contains("<!-- Comment to remove -->"));
        assert!(result.data.contains("data-processed"));
        assert!(!result.data.contains("id=\"unused\""));
        assert!(result.data.contains("id=\"used\""));
    }
}
```

### Performance Testing

Add benchmarks using the Criterion framework:

```rust
#[cfg(test)]
mod benches {
    use super::*;
    use criterion::{black_box, Criterion};

    pub fn benchmark_plugin(c: &mut Criterion) {
        let svg = generate_large_test_svg(1000); // Create test SVG with 1000 elements
        let plugin = MyCustomPlugin::new();

        c.bench_function("my_custom_plugin", |b| {
            b.iter(|| {
                let mut doc = vexy_svgo_core::parse_svg(black_box(&svg)).unwrap();
                plugin.apply(&mut doc).unwrap();
            });
        });
    }

    fn generate_large_test_svg(element_count: usize) -> String {
        let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg">"#);
        for i in 0..element_count {
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="10" height="10"/>"#,
                i % 100, i / 100
            ));
        }
        svg.push_str("</svg>");
        svg
    }
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
vexy-svgo-plugin-sdk = "1.0"
vexy-svgo-core = "1.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
vexy-svgo-test-utils = "1.0"
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