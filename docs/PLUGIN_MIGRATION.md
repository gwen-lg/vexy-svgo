# Plugin Migration Guide

This guide explains how to migrate existing SVGO plugins to the new vexy_svgo plugin architecture.

## Overview

The new plugin architecture uses composition over inheritance with a visitor pattern for SVG document traversal. Each plugin implements the `Plugin` trait and uses visitors to perform transformations.

## Migration Process

### 1. Plugin Structure

**Old SVGO Plugin (JavaScript):**
```javascript
exports.type = 'visitor';
exports.name = 'removeComments';
exports.description = 'removes comments';

exports.fn = (root, params) => {
  // Plugin logic here
};
```

**New Vexy SVGO Plugin (Rust):**
```rust
use crate::Plugin;
use vexy_svgo_core::ast::{Document, Element};
use vexy_svgo_core::visitor::Visitor;
use anyhow::Result;

pub struct RemoveCommentsPlugin {
    preserve_patterns: bool,
}

impl Plugin for RemoveCommentsPlugin {
    fn name(&self) -> &'static str {
        "removeComments"
    }

    fn description(&self) -> &'static str {
        "Remove comments from SVG document"
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        let mut visitor = CommentRemovalVisitor::new(self.preserve_patterns);
        vexy_svgo_core::visitor::walk_document(&mut visitor, document)?;
        Ok(())
    }
}
```

### 2. Visitor Implementation

Create a visitor struct that implements the `Visitor` trait:

```rust
struct CommentRemovalVisitor {
    preserve_patterns: bool,
}

impl Visitor<'_> for CommentRemovalVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Apply transformations to the element
        element.children.retain(|child| {
            match child {
                Node::Comment(comment) => self.should_keep_comment(comment),
                _ => true,
            }
        });
        Ok(())
    }
}
```

### 3. Parameter Validation

Implement parameter validation in the plugin:

```rust
fn validate_params(&self, params: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(preserve) = params.get("preservePatterns") {
        if !preserve.is_boolean() {
            return Err(anyhow::anyhow!(
                "preservePatterns must be a boolean"
            ));
        }
    }
    Ok(())
}
```

## Examples

### Example 1: RemoveComments Plugin

**Original SVGO Logic:**
- Remove all comments from SVG documents
- Optionally preserve "legal" comments (starting with !)

**Migration Steps:**
1. Create `RemoveCommentsPlugin` struct with configuration
2. Implement `Plugin` trait with name, description, and apply methods
3. Create `CommentRemovalVisitor` that filters comments based on configuration
4. Use visitor pattern to traverse and modify the document

**Files:**
- `crates/plugin-sdk/src/plugins/remove_comments.rs`
- Tests in the same file with `#[cfg(test)]`

### Example 2: RemoveEmptyAttrs Plugin

**Original SVGO Logic:**
- Remove attributes with empty values
- Optionally preserve specific attributes (class, id)

**Migration Steps:**
1. Create `RemoveEmptyAttrsPlugin` struct with preservation settings
2. Implement visitor that filters empty attributes
3. Add logic to handle preservation rules
4. Comprehensive testing for edge cases

**Files:**
- `crates/plugin-sdk/src/plugins/remove_empty_attrs.rs`

## Best Practices

### 1. Configuration Management

```rust
pub struct MyPlugin {
    option1: bool,
    option2: String,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            option1: true,
            option2: "default".to_string(),
        }
    }

    pub fn with_options(option1: bool, option2: String) -> Self {
        Self { option1, option2 }
    }
}
```

### 2. Error Handling

Use `anyhow::Result` for error handling and provide meaningful error messages:

```rust
fn validate_params(&self, params: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(value) = params.get("myParam") {
        if !value.is_boolean() {
            return Err(anyhow::anyhow!("myParam must be a boolean, got: {}", value));
        }
    }
    Ok(())
}
```

### 3. Testing Strategy

Create comprehensive tests covering:

- Plugin creation and configuration
- Parameter validation (valid and invalid cases)
- Visitor logic isolation
- Integration with documents
- Edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use vexy_svgo_core::ast::Document;
    use serde_json::json;

    #[test]
    fn test_plugin_creation() {
        let plugin = MyPlugin::new();
        assert_eq!(plugin.name(), "myPlugin");
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = MyPlugin::new();
        assert!(plugin.validate_params(&json!({})).is_ok());
        assert!(plugin.validate_params(&json!({"invalid": "value"})).is_err());
    }

    #[test]
    fn test_plugin_application() {
        let plugin = MyPlugin::new();
        let mut doc = Document::new();
        // Set up test document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
        // Verify transformations
    }
}
```

### 4. Integration Testing

Create integration tests that verify plugins work together:

```rust
// crates/plugin-sdk/tests/integration_test.rs
#[test]
fn test_multiple_plugins() {
    let mut registry = PluginRegistry::new();
    registry.register(RemoveCommentsPlugin::new());
    registry.register(RemoveEmptyAttrsPlugin::new());
    
    let configs = vec![
        PluginConfig { name: "removeComments".to_string(), /* ... */ },
        PluginConfig { name: "removeEmptyAttrs".to_string(), /* ... */ },
    ];
    
    let mut doc = create_test_document();
    registry.apply_plugins(&mut doc, &configs).unwrap();
    // Verify combined effects
}
```

## Plugin Registry Integration

### 1. Register Plugins

```rust
// In create_default_registry() function
let mut registry = PluginRegistry::new();
registry.register(RemoveCommentsPlugin::new());
registry.register(RemoveEmptyAttrsPlugin::new());
// Add more plugins...
```

### 2. Plugin Configuration

```rust
let config = PluginConfig {
    name: "removeComments".to_string(),
    params: json!({
        "preservePatterns": true
    }),
    enabled: true,
};
```

## Common Migration Patterns

### 1. Simple Element Transformation

```rust
impl Visitor<'_> for MyVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Modify attributes
        element.attributes.retain(|name, value| {
            // Filtering logic
        });
        
        // Modify children
        element.children.retain(|child| {
            // Filtering logic
        });
        
        Ok(())
    }
}
```

### 2. Conditional Processing

```rust
impl Visitor<'_> for MyVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        if self.should_process_element(&element.name) {
            // Apply transformations
        }
        Ok(())
    }
}
```

### 3. Stateful Visitors

```rust
struct StatefulVisitor {
    state: SomeState,
    counters: HashMap<String, usize>,
}

impl Visitor<'_> for StatefulVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Update state based on element
        self.state.update(&element);
        Ok(())
    }
}
```

## Performance Considerations

1. **Minimize Allocations**: Reuse data structures where possible
2. **Efficient Filtering**: Use `retain()` instead of collect/filter/rebuild
3. **Early Returns**: Skip processing when conditions aren't met
4. **Visitor Efficiency**: Only implement necessary visitor methods

## File Organization

```
crates/plugin-sdk/src/plugins/
├── mod.rs                    # Plugin exports
├── remove_comments.rs        # Remove comments plugin
├── remove_empty_attrs.rs     # Remove empty attributes plugin
└── ...                       # Additional plugins

crates/plugin-sdk/tests/
├── integration_test.rs       # Plugin integration tests
├── registry_test.rs          # Registry system tests
└── ...                       # Additional test suites

crates/plugin-sdk/examples/
├── plugin_composition.rs     # Multi-plugin usage example
└── ...                       # Additional examples
```

## Migration Checklist

For each plugin migration:

- [ ] Create plugin struct with configuration options
- [ ] Implement `Plugin` trait (name, description, validate_params, apply)
- [ ] Create visitor struct implementing `Visitor` trait
- [ ] Add comprehensive unit tests
- [ ] Add integration tests
- [ ] Update plugin registry in `create_default_registry()`
- [ ] Add plugin to module exports in `mod.rs`
- [ ] Document any SVGO compatibility differences
- [ ] Verify performance characteristics

## SVGO Compatibility Notes

- Plugin names should match SVGO plugin names exactly
- Parameter names should match SVGO parameter names
- Default behaviors should match SVGO defaults
- Document any intentional differences in behavior
- Maintain backward compatibility where possible