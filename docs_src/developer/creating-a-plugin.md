# Creating a Plugin

This tutorial will guide you through the process of creating a simple Vexy SVGO plugin.

## 1. Scaffolding the Plugin

First, create a new directory for your plugin in the `examples` directory:

```bash
mkdir -p examples/example-plugin/src
```

Next, create a `Cargo.toml` file for your plugin:

```toml
[package]
name = "example-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
vexy-svgo-core = { path = "../../crates/core" }
vexy-svgo-plugin-sdk = { path = "../../crates/plugin-sdk" }
```

## 2. Writing the Plugin

Now, let's write the plugin code in `examples/example-plugin/src/lib.rs`:

```rust
use vexy_svgo_core::ast::{Document, Element, Node};
use vexy_svgo_core::error::VexyError;
use vexy_svgo_core::plugin_registry::Plugin;

pub struct RemoveGElementPlugin;

impl Plugin for RemoveGElementPlugin {
    fn name(&self) -> &'static str {
        "removeGElement"
    }

    fn description(&self) -> &'static str {
        "Removes all <g> elements from an SVG."
    }

    fn apply(&self, document: &mut Document) -> Result<(), VexyError> {
        document.root.children.retain(|node| match node {
            Node::Element(element) => element.name != "g",
            _ => true,
        });
        Ok(())
    }
}
```

This plugin, `RemoveGElementPlugin`, removes all `<g>` elements from the SVG.

## 3. Using the Plugin

To use this plugin, you would need to load it into the plugin registry. This is typically done in the application layer. For example, in the CLI, you would modify the `main.rs` file to register the plugin.
