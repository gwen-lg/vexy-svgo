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

Vexy SVGO plugins are Rust modules that implement the `Plugin` trait from the `vexy_svgo-plugin-sdk`. They can traverse and modify the SVG AST (Abstract Syntax Tree) to perform optimizations.

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
git clone https://github.com/twardoch/vexy_svgo.git
cd vexy_svgo

# Build the project
./build.sh
```

### Project Structure

Create a new directory for your plugin:

```bash
mkdir -p vexy_svgo/src/plugins/my_custom_plugin
cd vexy_svgo/src/plugins/my_custom_plugin
```

## Creating Your First Plugin

Let's create a simple plugin that adds a custom attribute to all `<rect>` elements.

### Step 1: Create the Plugin Module

Create `vexy_svgo/src/plugins/my_custom_plugin/mod.rs`:

```rust
// this_file: vexy_svgo/src/plugins/my_custom_plugin/mod.rs

use vexy_svgo_plugin_sdk::{Plugin, PluginMetadata};
use vexy_svgo_core::visitor::{Visitor, VisitorContext};
use vexy_svgo_core::ast::{Document, Element, Node};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Configuration for our custom plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomPluginConfig {
    /// The attribute name to add
    pub attribute_name: String,
    /// The attribute value to add
    pub attribute_value: String,
    /// Only process elements with this class
    pub target_class: Option<String>,
}

impl Default for CustomPluginConfig {
    fn default() -> Self {
        Self {
            attribute_name: "data-custom".to_string(),
            attribute_value: "processed".to_string(),
            target_class: None,
        }
    }
}

/// Our custom plugin implementation
pub struct MyCustomPlugin {
    config: CustomPluginConfig,
}

impl MyCustomPlugin {
    pub fn new() -> Self {
        Self {
            config: CustomPluginConfig::default(),
        }
    }
    
    pub fn with_config(config: CustomPluginConfig) -> Self {
        Self { config }
    }
}

impl Plugin for MyCustomPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "myCustomPlugin".to_string(),
            description: "Adds custom attributes to rect elements".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Your Name".to_string()),
            tags: vec!["custom", "attribute", "rect"],
            experimental: false,
        }
    }
    
    fn optimize(&mut self, document: &mut Document) -> Result<()> {
        // Create a visitor for our plugin
        let mut visitor = CustomVisitor {
            config: &self.config,
            modifications: 0,
        };
        
        // Visit the document
        visitor.visit_document(document)?;
        
        // Log results if in debug mode
        if cfg!(debug_assertions) {
            eprintln!("MyCustomPlugin: Modified {} elements", visitor.modifications);
        }
        
        Ok(())
    }
    
    fn configure(&mut self, params: serde_json::Value) -> Result<()> {
        self.config = serde_json::from_value(params)?;
        Ok(())
    }
}

/// The visitor that does the actual work
struct CustomVisitor<'a> {
    config: &'a CustomPluginConfig,
    modifications: usize,
}

impl<'a> Visitor<'a> for CustomVisitor<'a> {
    fn visit_element_enter(&mut self, element: &mut Element<'a>) -> Result<()> {
        // Check if this is a rect element
        if element.name != "rect" {
            return Ok(());
        }
        
        // Check target class if specified
        if let Some(target_class) = &self.config.target_class {
            let class_attr = element.attributes.get("class");
            if !class_attr.map(|c| c.contains(target_class)).unwrap_or(false) {
                return Ok(());
            }
        }
        
        // Add the custom attribute
        element.attributes.insert(
            self.config.attribute_name.clone(),
            self.config.attribute_value.clone(),
        );
        
        self.modifications += 1;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vexy_svgo_core::parser::parse_svg_string;
    use vexy_svgo_core::stringifier::stringify;
    
    #[test]
    fn test_basic_functionality() {
        let svg = r#"<svg><rect x="0" y="0" width="100" height="100"/></svg>"#;
        let mut document = parse_svg_string(svg).unwrap();
        
        let mut plugin = MyCustomPlugin::new();
        plugin.optimize(&mut document).unwrap();
        
        let result = stringify(&document).unwrap();
        assert!(result.contains("data-custom=\"processed\""));
    }
    
    #[test]
    fn test_with_config() {
        let svg = r#"<svg><rect class="target" width="50" height="50"/></svg>"#;
        let mut document = parse_svg_string(svg).unwrap();
        
        let config = CustomPluginConfig {
            attribute_name: "data-id".to_string(),
            attribute_value: "custom-rect".to_string(),
            target_class: Some("target".to_string()),
        };
        
        let mut plugin = MyCustomPlugin::with_config(config);
        plugin.optimize(&mut document).unwrap();
        
        let result = stringify(&document).unwrap();
        assert!(result.contains("data-id=\"custom-rect\""));
    }
}
```

### Step 2: Register the Plugin

Add your plugin to `vexy_svgo/src/plugins/mod.rs`:

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
  target/release/vexy_svgo --enable myCustomPlugin
```

## Advanced Plugin Development

### Working with Complex AST Transformations

Here's a more advanced plugin that optimizes gradients:

```rust
// this_file: vexy_svgo/src/plugins/optimize_gradients/mod.rs

use vexy_svgo_plugin_sdk::Plugin;
use vexy_svgo_core::visitor::{Visitor, VisitorContext};
use vexy_svgo_core::ast::{Document, Element, Node};
use std::collections::{HashMap, HashSet};

pub struct OptimizeGradientsPlugin {
    gradients: HashMap<String, Element>,
    gradient_usage: HashMap<String, usize>,
    duplicates: HashMap<String, String>, // hash -> id mapping
}

impl OptimizeGradientsPlugin {
    pub fn new() -> Self {
        Self {
            gradients: HashMap::new(),
            gradient_usage: HashMap::new(),
            duplicates: HashMap::new(),
        }
    }
    
    fn calculate_gradient_hash(&self, gradient: &Element) -> String {
        // Create a deterministic hash of gradient properties
        use std::collections::BTreeMap;
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        
        // Sort attributes for deterministic hashing
        let attrs: BTreeMap<_, _> = gradient.attributes.iter().collect();
        for (key, value) in attrs {
            if key != "id" { // Exclude ID from hash
                key.hash(&mut hasher);
                value.hash(&mut hasher);
            }
        }
        
        // Hash child elements
        for child in &gradient.children {
            if let Node::Element(elem) = child {
                elem.name.hash(&mut hasher);
                let child_attrs: BTreeMap<_, _> = elem.attributes.iter().collect();
                for (k, v) in child_attrs {
                    k.hash(&mut hasher);
                    v.hash(&mut hasher);
                }
            }
        }
        
        format!("{:x}", hasher.finish())
    }
}

impl Plugin for OptimizeGradientsPlugin {
    fn metadata(&self) -> vexy_svgo_plugin_sdk::PluginMetadata {
        vexy_svgo_plugin_sdk::PluginMetadata {
            name: "optimizeGradients".to_string(),
            description: "Removes duplicate gradients and unused gradients".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Vexy SVGO Team".to_string()),
            tags: vec!["gradients", "optimization", "cleanup"],
            experimental: false,
        }
    }
    
    fn optimize(&mut self, document: &mut Document) -> anyhow::Result<()> {
        // First pass: collect all gradients
        self.visit_document(document)?;
        
        // Second pass: find duplicates
        let mut hash_to_id: HashMap<String, String> = HashMap::new();
        for (id, gradient) in &self.gradients {
            let hash = self.calculate_gradient_hash(gradient);
            if let Some(existing_id) = hash_to_id.get(&hash) {
                self.duplicates.insert(id.clone(), existing_id.clone());
            } else {
                hash_to_id.insert(hash, id.clone());
            }
        }
        
        // Third pass: update references and remove duplicates
        let mut updater = GradientUpdater {
            duplicates: &self.duplicates,
            unused_gradients: HashSet::new(),
        };
        updater.visit_document(document)?;
        
        Ok(())
    }
}

impl<'a> Visitor<'a> for OptimizeGradientsPlugin {
    fn visit_element_enter(&mut self, element: &mut Element<'a>) -> anyhow::Result<()> {
        // Collect gradients
        if element.name == "linearGradient" || element.name == "radialGradient" {
            if let Some(id) = element.attributes.get("id") {
                self.gradients.insert(id.clone(), element.clone());
            }
        }
        
        // Track gradient usage
        for (_, value) in &element.attributes {
            if let Some(url) = extract_url_reference(value) {
                *self.gradient_usage.entry(url).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }
}

struct GradientUpdater<'a> {
    duplicates: &'a HashMap<String, String>,
    unused_gradients: HashSet<String>,
}

impl<'a> Visitor<'a> for GradientUpdater<'a> {
    fn visit_element_enter(&mut self, element: &mut Element<'a>) -> anyhow::Result<()> {
        // Update references to duplicate gradients
        for (attr_name, attr_value) in element.attributes.iter_mut() {
            if let Some(url) = extract_url_reference(attr_value) {
                if let Some(canonical_id) = self.duplicates.get(&url) {
                    *attr_value = format!("url(#{})", canonical_id);
                }
            }
        }
        
        Ok(())
    }
    
    fn visit_element_exit(&mut self, element: &mut Element<'a>) -> anyhow::Result<()> {
        // Remove duplicate gradient definitions
        if element.name == "linearGradient" || element.name == "radialGradient" {
            if let Some(id) = element.attributes.get("id") {
                if self.duplicates.contains_key(id) {
                    // Mark for removal by clearing the element
                    element.name = "_remove".to_string();
                }
            }
        }
        
        Ok(())
    }
}

fn extract_url_reference(value: &str) -> Option<String> {
    if value.starts_with("url(#") && value.ends_with(')') {
        Some(value[5..value.len()-1].to_string())
    } else {
        None
    }
}
```

### Plugin Communication

Plugins can share data through the visitor context:

```rust
use vexy_svgo_core::visitor::{VisitorContext, ContextData};

impl<'a> Visitor<'a> for MyPlugin {
    fn visit_element_enter(
        &mut self, 
        element: &mut Element<'a>,
        context: &mut VisitorContext
    ) -> Result<()> {
        // Store data for other plugins
        context.set_data("element_count", 
            context.get_data::<usize>("element_count").unwrap_or(0) + 1
        );
        
        // Read data from previous plugins
        if let Some(colors) = context.get_data::<Vec<String>>("used_colors") {
            // Process based on colors used in document
        }
        
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
    use vexy_svgo_test_utils::{assert_svg_eq, optimize_with_plugin};
    
    #[test]
    fn test_removes_duplicate_gradients() {
        let input = r#"
            <svg>
                <defs>
                    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="0%">
                        <stop offset="0%" style="stop-color:rgb(255,255,0)" />
                        <stop offset="100%" style="stop-color:rgb(255,0,0)" />
                    </linearGradient>
                    <linearGradient id="grad2" x1="0%" y1="0%" x2="100%" y2="0%">
                        <stop offset="0%" style="stop-color:rgb(255,255,0)" />
                        <stop offset="100%" style="stop-color:rgb(255,0,0)" />
                    </linearGradient>
                </defs>
                <rect fill="url(#grad1)" />
                <rect fill="url(#grad2)" />
            </svg>
        "#;
        
        let expected = r#"
            <svg>
                <defs>
                    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="0%">
                        <stop offset="0%" style="stop-color:rgb(255,255,0)" />
                        <stop offset="100%" style="stop-color:rgb(255,0,0)" />
                    </linearGradient>
                </defs>
                <rect fill="url(#grad1)" />
                <rect fill="url(#grad1)" />
            </svg>
        "#;
        
        let result = optimize_with_plugin(input, OptimizeGradientsPlugin::new());
        assert_svg_eq(&result, expected);
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
    let large_svg = generate_large_svg_with_gradients(1000);
    let plugin = OptimizeGradientsPlugin::new();
    
    b.iter(|| {
        let mut doc = parse_svg_string(&large_svg).unwrap();
        plugin.optimize(&mut doc).unwrap();
    });
}
```

## Publishing and Distribution

### 1. Package Your Plugin

Create a separate crate for distribution:

```toml
# Cargo.toml
[package]
name = "vexy_svgo-plugin-custom"
version = "0.1.0"
edition = "2021"

[dependencies]
vexy_svgo-plugin-sdk = "2.0"
vexy_svgo-core = "2.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
vexy_svgo-test-utils = "2.0"
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
//! use vexy_svgo_plugin_custom::MyCustomPlugin;
//! use vexy_svgo_core::Config;
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
        // Early return for non-target elements
        if !self.should_process(element) {
            return Ok(());
        }
        
        // Reuse allocated strings
        self.buffer.clear();
        self.buffer.push_str(&element.attributes.get("class").unwrap_or_default());
        
        // Batch attribute updates
        element.attributes.reserve(self.new_attributes.len());
        for (key, value) in &self.new_attributes {
            element.attributes.insert(key.clone(), value.clone());
        }
        
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
    match self.try_optimize(element) {
        Ok(()) => Ok(()),
        Err(e) if e.is_recoverable() => {
            eprintln!("Warning in {}: {}", self.metadata().name, e);
            Ok(()) // Continue processing
        }
        Err(e) => Err(e).context(format!(
            "Failed to process {} element", 
            element.name
        )),
    }
}
```

### 3. Configuration Validation

```rust
impl Plugin for ValidatedPlugin {
    fn configure(&mut self, params: serde_json::Value) -> Result<()> {
        let config: PluginConfig = serde_json::from_value(params)?;
        
        // Validate configuration
        if config.threshold < 0.0 || config.threshold > 1.0 {
            return Err(anyhow!("Threshold must be between 0 and 1"));
        }
        
        if config.target_elements.is_empty() {
            return Err(anyhow!("At least one target element must be specified"));
        }
        
        self.config = config;
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
        // Check SVGO compatibility mode
        if self.svgo_compatible {
            self.optimize_svgo_way(document)
        } else {
            self.optimize_enhanced(document)
        }
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

- [Vexy SVGO Plugin SDK Documentation](https://docs.rs/vexy_svgo-plugin-sdk)
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