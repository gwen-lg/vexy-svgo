# Vexy SVGO Plugin SDK

The Vexy SVGO Plugin SDK provides everything you need to create custom plugins for the Vexy SVGO SVG optimizer. This SDK includes traits, utilities, testing frameworks, and documentation to help you build high-performance plugins that integrate seamlessly with Vexy SVGO.

## Table of Contents

- [Quick Start](#quick-start)
- [Plugin Architecture](#plugin-architecture)
- [Creating Your First Plugin](#creating-your-first-plugin)
- [Testing Framework](#testing-framework)
- [Advanced Features](#advanced-features)
- [Publishing](#publishing)
- [Examples](#examples)

## Quick Start

Add the plugin SDK to your `Cargo.toml`:

```toml
[dependencies]
vexy_svgo-plugin-sdk = "2.0"
vexy_svgo-core = "2.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
vexy_svgo-test-utils = "2.0"
```

Create a basic plugin:

```rust
use vexy_svgo_plugin_sdk::{Plugin, PluginMetadata};
use vexy_svgo_core::{ast::Document, visitor::Visitor};

#[derive(Default)]
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "myPlugin".to_string(),
            description: "Does something useful".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Your Name".to_string()),
            tags: vec!["optimization"],
            experimental: false,
        }
    }

    fn optimize(&mut self, document: &mut Document) -> anyhow::Result<()> {
        // Your optimization logic here
        Ok(())
    }
}
```

## Plugin Architecture

### Core Concepts

1. **Plugin Trait**: The main interface all plugins must implement
2. **Visitor Pattern**: Traverse and modify the SVG AST
3. **Configuration**: Accept parameters from users
4. **Metadata**: Provide information about your plugin
5. **Error Handling**: Robust error reporting and recovery

### Plugin Lifecycle

```
Initialize → Configure → Optimize → Report
     ↓            ↓         ↓         ↓
   Setup     Parse Params   Process   Results
```

## Creating Your First Plugin

### Step 1: Implement the Plugin Trait

```rust
use vexy_svgo_plugin_sdk::{Plugin, PluginMetadata, PluginResult};
use vexy_svgo_core::{ast::Document, visitor::{Visitor, VisitorContext}};
use anyhow::Result;

pub struct RemoveEmptyElementsPlugin {
    removed_count: usize,
}

impl Default for RemoveEmptyElementsPlugin {
    fn default() -> Self {
        Self { removed_count: 0 }
    }
}

impl Plugin for RemoveEmptyElementsPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "removeEmptyElements".to_string(),
            description: "Removes empty elements that have no content or attributes".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Vexy SVGO Team".to_string()),
            tags: vec!["cleanup", "size-reduction"],
            experimental: false,
        }
    }

    fn optimize(&mut self, document: &mut Document) -> Result<PluginResult> {
        self.removed_count = 0;
        let mut visitor = EmptyElementRemover {
            removed_count: &mut self.removed_count,
        };
        
        visitor.visit_document(document)?;
        
        Ok(PluginResult {
            modified: self.removed_count > 0,
            changes: self.removed_count,
            messages: if self.removed_count > 0 {
                vec![format!("Removed {} empty elements", self.removed_count)]
            } else {
                vec![]
            },
        })
    }
}

struct EmptyElementRemover<'a> {
    removed_count: &'a mut usize,
}

impl<'a> Visitor<'a> for EmptyElementRemover<'a> {
    fn visit_element_exit(&mut self, element: &mut Element<'a>) -> Result<()> {
        if element.children.is_empty() && element.attributes.is_empty() {
            // Mark for removal
            element.name = "_remove".to_string();
            *self.removed_count += 1;
        }
        Ok(())
    }
}
```

### Step 2: Add Configuration Support

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RemoveEmptyConfig {
    /// Remove elements with only whitespace content
    pub remove_whitespace_only: bool,
    /// Preserve elements with these names
    pub preserve_elements: Vec<String>,
}

impl Default for RemoveEmptyConfig {
    fn default() -> Self {
        Self {
            remove_whitespace_only: true,
            preserve_elements: vec!["title".to_string(), "desc".to_string()],
        }
    }
}

impl Plugin for RemoveEmptyElementsPlugin {
    // ... previous implementation

    fn configure(&mut self, params: serde_json::Value) -> Result<()> {
        let config: RemoveEmptyConfig = serde_json::from_value(params)?;
        self.config = config;
        Ok(())
    }
}
```

### Step 3: Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use vexy_svgo_test_utils::{test_plugin, assert_svg_eq};

    #[test]
    fn removes_empty_elements() {
        let input = r#"<svg><g></g><rect width="10" height="10"/></svg>"#;
        let expected = r#"<svg><rect width="10" height="10"/></svg>"#;
        
        let result = test_plugin(input, RemoveEmptyElementsPlugin::default());
        assert_svg_eq(&result, expected);
    }

    #[test]
    fn preserves_configured_elements() {
        let config = RemoveEmptyConfig {
            preserve_elements: vec!["title".to_string()],
            ..Default::default()
        };

        let input = r#"<svg><title></title><g></g></svg>"#;
        let expected = r#"<svg><title></title></svg>"#;
        
        let mut plugin = RemoveEmptyElementsPlugin::default();
        plugin.configure(serde_json::to_value(config).unwrap()).unwrap();
        
        let result = test_plugin(input, plugin);
        assert_svg_eq(&result, expected);
    }
}
```

## Testing Framework

The SDK includes comprehensive testing utilities:

### Basic Testing

```rust
use vexy_svgo_test_utils::*;

#[test]
fn test_basic_optimization() {
    let plugin = MyPlugin::default();
    let input = "<svg><rect/></svg>";
    let result = test_plugin(input, plugin);
    
    // Assert the result
    assert!(result.contains("rect"));
}
```

### Advanced Testing

```rust
use vexy_svgo_test_utils::*;

#[test]
fn test_with_config() {
    let config = serde_json::json!({
        "removeComments": true,
        "preserveIds": ["important-id"]
    });
    
    let result = test_plugin_with_config(
        "<svg><!-- comment --><g id=\"important-id\"/></svg>",
        MyPlugin::default(),
        config
    );
    
    assert!(!result.contains("<!--"));
    assert!(result.contains("important-id"));
}
```

### Performance Testing

```rust
use vexy_svgo_test_utils::*;
use criterion::{black_box, Criterion};

#[bench]
fn bench_plugin(c: &mut Criterion) {
    let svg = generate_large_svg(1000);
    let plugin = MyPlugin::default();
    
    c.bench_function("my_plugin", |b| {
        b.iter(|| test_plugin(black_box(&svg), black_box(plugin.clone())))
    });
}
```

### Compatibility Testing

```rust
#[test]
fn test_svgo_compatibility() {
    let test_cases = load_svgo_test_cases("removeComments");
    
    for case in test_cases {
        let result = test_plugin(&case.input, MyPlugin::default());
        assert_svg_equivalent(&result, &case.expected);
    }
}
```

## Advanced Features

### Context-Aware Plugins

```rust
use vexy_svgo_core::visitor::VisitorContext;

impl<'a> Visitor<'a> for MyVisitor<'a> {
    fn visit_element_enter(
        &mut self, 
        element: &mut Element<'a>,
        context: &mut VisitorContext
    ) -> Result<()> {
        // Access document-wide information
        let total_elements = context.get_data::<usize>("element_count")
            .unwrap_or(0);
        
        // Share data with other plugins
        context.set_data("processed_elements", total_elements + 1);
        
        Ok(())
    }
}
```

### Multi-pass Plugins

```rust
impl Plugin for MultiPassPlugin {
    fn optimize(&mut self, document: &mut Document) -> Result<PluginResult> {
        let mut total_changes = 0;
        let mut pass = 1;
        
        loop {
            let mut changes = 0;
            // Run optimization pass
            // ... increment changes
            
            total_changes += changes;
            
            if changes == 0 || pass >= MAX_PASSES {
                break;
            }
            
            pass += 1;
        }
        
        Ok(PluginResult {
            modified: total_changes > 0,
            changes: total_changes,
            messages: vec![format!("Completed {} passes", pass)],
        })
    }
}
```

### Async Plugins (Feature Gated)

```rust
#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "async")]
#[async_trait]
impl AsyncPlugin for NetworkPlugin {
    async fn optimize_async(&mut self, document: &mut Document) -> Result<PluginResult> {
        // Perform async operations (e.g., external API calls)
        let data = fetch_optimization_data().await?;
        
        // Apply optimizations
        Ok(PluginResult::default())
    }
}
```

## Publishing

### Package Structure

```
my-vexy_svgo-plugin/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── lib.rs
│   ├── plugin.rs
│   └── config.rs
├── tests/
│   ├── integration_tests.rs
│   └── compatibility_tests.rs
├── benches/
│   └── benchmarks.rs
└── examples/
    └── usage.rs
```

### Cargo.toml Template

```toml
[package]
name = "vexy_svgo-plugin-my-plugin"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "Vexy SVGO plugin for custom optimization"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/vexy_svgo-plugin-my-plugin"
keywords = ["svg", "optimization", "vexy_svgo", "plugin"]
categories = ["multimedia::images", "web-programming"]

[dependencies]
vexy_svgo-plugin-sdk = "2.0"
vexy_svgo-core = "2.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
vexy_svgo-test-utils = "2.0"
criterion = { version = "0.5", features = ["html_reports"] }
tokio = { version = "1.0", features = ["macros"] }

[[bench]]
name = "plugin_benchmarks"
harness = false

[features]
default = []
async = ["async-trait", "tokio"]
```

### Documentation Requirements

1. **API Documentation**: Use `///` comments for all public items
2. **Examples**: Include usage examples in docstrings
3. **Configuration**: Document all configuration options
4. **Performance**: Include benchmark results
5. **Compatibility**: Document SVGO compatibility

### Release Checklist

- [ ] All tests pass (`cargo test`)
- [ ] Benchmarks run successfully (`cargo bench`)
- [ ] Documentation builds (`cargo doc`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Format is correct (`cargo fmt --check`)
- [ ] CHANGELOG.md is updated
- [ ] Version is bumped in Cargo.toml
- [ ] Git tag is created
- [ ] Published to crates.io (`cargo publish`)

## Examples

### Color Optimization Plugin

```rust
use vexy_svgo_plugin_sdk::{Plugin, PluginMetadata};
use regex::Regex;

pub struct OptimizeColorsPlugin {
    hex_regex: Regex,
    rgb_regex: Regex,
}

impl Default for OptimizeColorsPlugin {
    fn default() -> Self {
        Self {
            hex_regex: Regex::new(r"#([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])").unwrap(),
            rgb_regex: Regex::new(r"rgb\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap(),
        }
    }
}

impl Plugin for OptimizeColorsPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "optimizeColors".to_string(),
            description: "Optimizes color values for smaller size".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Vexy SVGO Team".to_string()),
            tags: vec!["colors", "size-reduction"],
            experimental: false,
        }
    }

    fn optimize(&mut self, document: &mut Document) -> anyhow::Result<()> {
        let mut visitor = ColorOptimizer {
            hex_regex: &self.hex_regex,
            rgb_regex: &self.rgb_regex,
            optimizations: 0,
        };
        
        visitor.visit_document(document)?;
        Ok(())
    }
}
```

### Path Simplification Plugin

```rust
use vexy_svgo_plugin_sdk::{Plugin, PluginMetadata};
use vexy_svgo_core::path::{PathParser, PathOptimizer};

pub struct SimplifyPathsPlugin {
    precision: u8,
    tolerance: f64,
}

impl Plugin for SimplifyPathsPlugin {
    fn optimize(&mut self, document: &mut Document) -> anyhow::Result<()> {
        let mut visitor = PathSimplifier {
            precision: self.precision,
            tolerance: self.tolerance,
            simplified: 0,
        };
        
        visitor.visit_document(document)?;
        Ok(())
    }
}
```

## SDK Utilities

### Plugin Registry

```rust
use vexy_svgo_plugin_sdk::registry::PluginRegistry;

let mut registry = PluginRegistry::new();
registry.register(Box::new(MyPlugin::default()));
registry.register_factory("myPlugin", || Box::new(MyPlugin::default()));

let plugin = registry.create_plugin("myPlugin")?;
```

### Configuration Helpers

```rust
use vexy_svgo_plugin_sdk::config::{ConfigBuilder, ValidationRule};

let config = ConfigBuilder::new()
    .add_field("threshold", ValidationRule::Float { min: 0.0, max: 1.0 })
    .add_field("enabled", ValidationRule::Boolean)
    .add_field("elements", ValidationRule::Array)
    .build();
```

### Error Handling

```rust
use vexy_svgo_plugin_sdk::error::{PluginError, PluginErrorKind};

fn process_element(element: &Element) -> Result<(), PluginError> {
    if element.name.is_empty() {
        return Err(PluginError::new(
            PluginErrorKind::InvalidElement,
            "Element name cannot be empty"
        ).with_context("processing element"));
    }
    Ok(())
}
```

## Best Practices

1. **Performance**: Minimize allocations, use efficient algorithms
2. **Memory**: Clean up resources, avoid memory leaks
3. **Errors**: Provide helpful error messages with context
4. **Configuration**: Validate all input parameters
5. **Testing**: Write comprehensive tests including edge cases
6. **Documentation**: Document all public APIs thoroughly
7. **Compatibility**: Follow SVGO conventions where applicable

## Support

- [Plugin Development Guide](https://docs.vexy_svgo.org/plugins)
- [API Reference](https://docs.rs/vexy_svgo-plugin-sdk)
- [Examples Repository](https://github.com/vexyart/vexy-svgo/tree/main/examples)
- [Community Discord](https://discord.gg/vexy_svgo)

## License

This SDK is licensed under the same terms as Vexy SVGO: MIT OR Apache-2.0
