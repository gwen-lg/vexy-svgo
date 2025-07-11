# Vexy SVGO Example Plugin - Accessibility Enhancement

This is a comprehensive example plugin for Vexy SVGO that demonstrates best practices for plugin development. The plugin automatically adds accessibility attributes to SVG elements.

## Features

- ✅ Adds ARIA labels to common SVG elements
- ✅ Detects and labels icon patterns
- ✅ Adds role attributes for better screen reader support
- ✅ Configurable via JSON parameters
- ✅ Respects existing accessibility attributes
- ✅ Supports custom label mappings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vexy_svgo-plugin-example = "0.1"
```

## Usage

### As a Library

```rust
use vexy_svgo_plugin_example::AccessibilityPlugin;
use vexy_svgo_core::{optimize_with_config, Config, PluginConfig};

let mut config = Config::default();
config.plugins.push(PluginConfig::WithParams {
    name: "addAccessibility".to_string(),
    params: serde_json::json!({
        "addRoles": true,
        "addLabels": true,
        "lang": "en",
        "customLabels": {
            "logo": "Company Logo",
            "nav-icon": "Navigation Menu"
        }
    }),
});

let svg = r#"<svg><rect x="0" y="0" width="100" height="100"/></svg>"#;
let result = optimize_with_config(svg, config)?;
```

### With CLI

```bash
vexy_svgo input.svg -o output.svg --enable addAccessibility --config '{"plugins":[{"name":"addAccessibility","params":{"addRoles":true}}]}'
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `addRoles` | boolean | `true` | Add ARIA role attributes |
| `addLabels` | boolean | `true` | Add aria-label attributes |
| `lang` | string | `"en"` | Language for generated labels |
| `customLabels` | object | `{}` | Custom label mappings (id -> label) |
| `addTitles` | boolean | `false` | Add title elements for tooltips |
| `skipClasses` | array | `["decorative", "background"]` | Skip elements with these classes |

## Examples

### Basic Shape Labeling

**Input:**
```svg
<svg>
  <circle cx="50" cy="50" r="40" fill="red"/>
  <rect x="10" y="10" width="30" height="30" fill="blue" stroke="black"/>
</svg>
```

**Output:**
```svg
<svg role="img">
  <circle cx="50" cy="50" r="40" fill="red" aria-label="circle, filled with red"/>
  <rect x="10" y="10" width="30" height="30" fill="blue" stroke="black" 
        aria-label="rectangle, filled with blue, outlined in black"/>
</svg>
```

### Custom Labels

**Configuration:**
```json
{
  "customLabels": {
    "logo": "ACME Corporation Logo",
    "menu-icon": "Main Navigation Menu"
  }
}
```

**Input:**
```svg
<svg>
  <g id="logo">
    <path d="M10,10 L50,10 L30,40 Z"/>
  </g>
  <g id="menu-icon">
    <rect y="0" width="20" height="3"/>
    <rect y="7" width="20" height="3"/>
    <rect y="14" width="20" height="3"/>
  </g>
</svg>
```

**Output:**
```svg
<svg role="img">
  <g id="logo" aria-label="ACME Corporation Logo">
    <path d="M10,10 L50,10 L30,40 Z" aria-label="custom shape"/>
  </g>
  <g id="menu-icon" aria-label="Main Navigation Menu">
    <rect y="0" width="20" height="3" aria-label="rectangle"/>
    <rect y="7" width="20" height="3" aria-label="rectangle"/>
    <rect y="14" width="20" height="3" aria-label="rectangle"/>
  </g>
</svg>
```

### Interactive Elements

**Input:**
```svg
<svg>
  <g onclick="handleClick()">
    <rect x="0" y="0" width="100" height="40"/>
    <text x="50" y="25" text-anchor="middle">Click Me</text>
  </g>
</svg>
```

**Output:**
```svg
<svg role="img">
  <g onclick="handleClick()" role="button" aria-label="group with 2 elements">
    <rect x="0" y="0" width="100" height="40" aria-label="rectangle"/>
    <text x="50" y="25" text-anchor="middle" role="button" aria-label="Click Me">Click Me</text>
  </g>
</svg>
```

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Benchmarking

```bash
cargo bench
```

## Architecture

The plugin follows Vexy SVGO's visitor pattern:

1. **Configuration Phase**: Parse and validate plugin parameters
2. **Traversal Phase**: Visit each element in the SVG AST
3. **Analysis Phase**: Determine appropriate accessibility attributes
4. **Modification Phase**: Add attributes without duplicating existing ones

### Key Components

- **AccessibilityPlugin**: Main plugin implementation
- **AccessibilityVisitor**: Traverses the AST and applies modifications
- **IconDetector**: Pattern matching for common icon shapes
- **Label Generators**: Create meaningful labels based on element properties

## Best Practices Demonstrated

1. **Configuration Validation**: Validates all parameters with serde
2. **Error Handling**: Uses anyhow with context for clear error messages
3. **Performance**: Early returns and minimal allocations
4. **Testing**: Comprehensive unit tests and benchmarks
5. **Documentation**: Inline documentation and examples
6. **Compatibility**: Respects existing attributes and SVGO patterns

## License

MIT License - See LICENSE file for details