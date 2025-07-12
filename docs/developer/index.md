---
# this_file: docs/developer/index.md
layout: default
title: Developer Guide
nav_order: 4
description: "APIs, architecture, and contributing to Vexy SVGO"
has_children: true
permalink: /developer/
---

# Developer Guide
{: .no_toc }

APIs, architecture, and contributing to Vexy SVGO
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Overview

Vexy SVGO is designed for both end-users and developers. This section covers the technical aspects of the project, including APIs, architecture, and how to contribute.

### For Developers

- **🔧 Rust API**: Use Vexy SVGO as a library in Rust projects
- **🌐 WebAssembly**: Integrate into web applications and Node.js
- **🧩 Plugin Development**: Create custom optimization plugins
- **🤝 Contributing**: Help improve the project

---

## Architecture Overview

Vexy SVGO follows a modular architecture inspired by SVGO but optimized for Rust:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│                 │    │                 │    │                 │
│   Input SVG     │───▶│   Parser        │───▶│   AST           │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                       │
                                                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│                 │    │                 │    │                 │
│  Optimized SVG  │◀───│  Stringifier    │◀───│  Plugin Engine  │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Components

1. **Parser**: Converts SVG strings to Abstract Syntax Tree (AST)
2. **Plugin Engine**: Applies optimization plugins to the AST
3. **Stringifier**: Converts optimized AST back to SVG string
4. **CLI**: Command-line interface for end users

---

## Quick Start for Developers

### 1. Use as Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
vexy-svgo = { git = "https://github.com/vexyart/vexy-svgo" }
```

Basic usage:

```rust
use vexy_svgo::{optimize, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <circle cx="50" cy="50" r="40" fill="red"/>
    </svg>"#;
    
    let config = Config::default();
    let result = optimize(svg, &config)?;
    
    println!("Original: {} bytes", svg.len());
    println!("Optimized: {} bytes", result.data.len());
    println!("Reduction: {:.1}%", result.size_reduction);
    
    Ok(())
}
```

### 2. WebAssembly Integration

```javascript
// Load the WASM module
import init, { optimize } from './pkg/vexy_svgo_wasm.js';

async function optimizeSvg() {
    await init();
    
    const svg = '<svg>...</svg>';
    const config = {
        multipass: true,
        plugins: {
            removeComments: true,
            removeEmptyAttrs: true
        }
    };
    
    const result = optimize(svg, config);
    console.log('Optimized:', result.data);
}
```

### 3. Create Custom Plugin

```rust
use vexy_svgo_plugin_sdk::{Plugin, PluginMetadata};
use vexy_svgo_core::{ast::Document, visitor::Visitor};

pub struct MyCustomPlugin;

impl Plugin for MyCustomPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "myCustomPlugin".to_string(),
            description: "My custom optimization".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Your Name".to_string()),
            tags: vec!["custom"],
            experimental: false,
        }
    }
    
    fn optimize(&mut self, document: &mut Document) -> anyhow::Result<()> {
        // Your optimization logic here
        Ok(())
    }
}
```

---

## Developer Documentation Sections

<div class="d-flex flex-wrap">
  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">📚 API Reference</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Complete Rust API documentation</p>
        <a href="/developer/api-reference/" class="btn btn-sm">Read docs →</a>
      </div>
    </div>
  </div>

  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">🏗️ Architecture</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Deep dive into system design</p>
        <a href="/developer/architecture/" class="btn btn-sm">Read more →</a>
      </div>
    </div>
  </div>

  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">🧩 Plugin Development</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Create custom optimization plugins</p>
        <a href="/developer/plugin-development/" class="btn btn-sm">Learn how →</a>
      </div>
    </div>
  </div>

  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">🤝 Contributing</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Help improve Vexy SVGO</p>
        <a href="/developer/contributing/" class="btn btn-sm">Get started →</a>
      </div>
    </div>
  </div>
</div>

---

## Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional tools
cargo install wasm-pack  # For WebAssembly builds
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/vexyart/vexy-svgo
cd vexy-svgo

# Build all components
./build.sh all

# Or build specific components
./build.sh native     # CLI and library
./build.sh wasm       # WebAssembly
./build.sh docs       # Documentation
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --package vexy-svgo-core
cargo test --package vexy-svgo-cli

# Run benchmarks
cargo bench
```

---

## API Examples

### Basic Optimization

```rust
use vexy_svgo::{optimize, Config};

let svg = "<svg>...</svg>";
let result = optimize(svg, &Config::default())?;
println!("Optimized: {}", result.data);
```

### Custom Configuration

```rust
use vexy_svgo::{optimize, Config, PluginConfig};

let config = Config {
    multipass: true,
    plugins: vec![
        PluginConfig::Enabled("removeComments".to_string()),
        PluginConfig::WithParams {
            name: "convertColors".to_string(),
            params: serde_json::json!({
                "currentColor": true
            })
        }
    ],
    js2svg: Js2SvgConfig {
        pretty: true,
        indent: 2,
        ..Default::default()
    },
    ..Default::default()
};

let result = optimize(svg, &config)?;
```

### Streaming Interface

```rust
use vexy_svgo::stream::{SvgOptimizer, StreamConfig};

let mut optimizer = SvgOptimizer::new(StreamConfig::default());

// Process large files in chunks
optimizer.write_chunk(svg_chunk_1)?;
optimizer.write_chunk(svg_chunk_2)?;
let result = optimizer.finalize()?;
```

---

## WebAssembly Development

### Building WASM Module

```bash
# Build for web
wasm-pack build crates/wasm --target web --out-dir pkg-web

# Build for Node.js
wasm-pack build crates/wasm --target nodejs --out-dir pkg-node

# Build for bundlers
wasm-pack build crates/wasm --target bundler --out-dir pkg-bundler
```

### Integration Examples

#### React Component

```jsx
import { useEffect, useState } from 'react';
import init, { optimize } from 'vexy-svgo-wasm';

function SvgOptimizer() {
    const [wasmReady, setWasmReady] = useState(false);
    
    useEffect(() => {
        init().then(() => setWasmReady(true));
    }, []);
    
    const optimizeSvg = (svg) => {
        if (!wasmReady) return;
        return optimize(svg, { multipass: true });
    };
    
    // Component implementation...
}
```

#### Node.js Script

```javascript
const { optimize } = require('vexy-svgo-wasm');

async function optimizeFiles() {
    const fs = require('fs').promises;
    
    const svg = await fs.readFile('input.svg', 'utf8');
    const result = optimize(svg, {
        plugins: {
            removeComments: true,
            removeEmptyAttrs: true
        }
    });
    
    await fs.writeFile('output.svg', result.data);
    console.log(`Reduced size by ${result.sizeReduction}%`);
}
```

---

## Performance Considerations

### Optimization Tips

1. **Batch Processing**: Process multiple files together
2. **Plugin Selection**: Only enable needed plugins
3. **Memory Management**: Use streaming for large files
4. **Caching**: Cache parsed ASTs when processing similar files

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vexy_svgo::{optimize, Config};

fn benchmark_optimize(c: &mut Criterion) {
    let svg = include_str!("../testdata/large.svg");
    let config = Config::default();
    
    c.bench_function("optimize large svg", |b| {
        b.iter(|| optimize(black_box(svg), black_box(&config)))
    });
}

criterion_group!(benches, benchmark_optimize);
criterion_main!(benches);
```

---

## Getting Help

- **📖 Documentation**: Continue reading the developer guides
- **🐛 Issues**: Report bugs on [GitHub Issues](https://github.com/vexyart/vexy-svgo/issues)
- **💬 Discussions**: Ask questions in [GitHub Discussions](https://github.com/vexyart/vexy-svgo/discussions)
- **📧 Contact**: Email the [maintainers](mailto:twardoch@github.com)

---

*Ready to dive deeper? Continue to [API Reference](/developer/api-reference/) →*