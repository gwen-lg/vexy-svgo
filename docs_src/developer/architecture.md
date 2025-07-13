---
nav_weight: 31
layout: default
title: Architecture
nav_order: 5
description: "Vexy SVGO internal architecture and design"
---

# Vexy SVGO Architecture

`vexy_svgo` is designed as a native Rust port of `svgo`, aiming to replicate its core architectural principles while leveraging Rust's strengths for performance and reliability. The architecture closely mirrors `svgo`'s modular design, consisting of a core engine, parser, stringifier, and a robust plugin system.

## Core Components

### 1. Core Engine (`vexy_svgo/src/optimizer.rs`)

Similar to `svgo`'s `lib/svgo.js`, the `vexy_svgo` core engine orchestrates the SVG optimization process. It takes an SVG string and a configuration object, then applies a pipeline of plugins to the parsed SVG Abstract Syntax Tree (AST). The engine manages the order of plugin execution and handles multi-pass optimizations if configured.

### 2. Parser (`vexy_svgo/src/parser.rs`)

The parser component is responsible for transforming an SVG string into an Abstract Syntax Tree (AST). In `svgo`, this is handled by `lib/parser.js`, which uses a SAX-like approach. `vexy_svgo` implements its own efficient SVG parser in Rust, converting the raw SVG XML into a structured, traversable AST representation that plugins can operate on.

### 3. Plugins (`vexy_svgo/src/plugins/`)

Plugins are the heart of `vexy_svgo`'s optimization capabilities. Each plugin is a self-contained module that performs a specific optimization or transformation on the SVG AST. `vexy_svgo`'s plugin system is designed to be compatible with `svgo`'s plugin API concepts, allowing for a systematic porting of existing `svgo` plugins.

-   **Modularity**: Each optimization is encapsulated within its own plugin, promoting code organization and reusability.
-   **AST Transformation**: Plugins receive and modify the SVG AST, enabling complex manipulations of SVG elements, attributes, and styles.
-   **Configurability**: Plugins can be enabled, disabled, and configured with specific parameters via the `VexySvgoConfig` object.

### 4. Stringifier (`vexy_svgo/src/stringifier.rs`)

After all plugins have processed the AST, the stringifier component converts the optimized AST back into a minified SVG string. This component is analogous to `svgo`'s `lib/stringifier.js`. The stringifier handles proper XML serialization, including attribute ordering, whitespace management, and numeric precision, to ensure the smallest possible output size while maintaining valid SVG syntax.

### 5. Command-Line Interface (CLI) (`vexy_svgo/src/bin/vexy_svgo.rs`)

The `vexy_svgo` CLI provides a user-friendly interface for optimizing SVG files directly from the terminal. It parses command-line arguments, loads configuration, invokes the core optimization engine, and outputs the results. This component mirrors the functionality of `svgo`'s `bin/svgo` and `lib/svgo-node.js`.

## Parallel Processing (Optional Feature)

`vexy_svgo` includes an optional parallel processing feature that can significantly improve performance when working with large SVG files. This feature leverages Rust's excellent concurrency support through the `rayon` crate.

### When Parallel Processing is Activated

The parallel processing feature automatically activates for files that meet configurable thresholds:

- **File size threshold**: Default 1MB (configurable)
- **Element count threshold**: Default 1000 elements (configurable)

### How It Works

The parallel optimization system:

1. **Identifies independent element groups**: Analyzes the SVG structure to find elements that don't have cross-references (like `<use>` elements or gradient references)
2. **Processes groups concurrently**: Large independent groups (>10 elements) are processed in parallel using a thread pool
3. **Maintains SVG integrity**: Ensures that element references and dependencies are preserved

### Performance Benefits

Based on our benchmarks with large SVG files:

- **Up to 4-8x faster** on multi-core systems for files with >1000 elements
- **Linear scaling** with core count for highly parallelizable SVGs (like icon sets)
- **Minimal overhead** for small files (parallel processing only activates above thresholds)
- **Memory efficient**: Uses work-stealing algorithm to balance load across threads

### Example Performance Gains

| File Type | Elements | Sequential Time | Parallel Time (8 cores) | Speedup |
|-----------|----------|----------------|------------------------|---------|
| Icon Set | 5,000 | 250ms | 35ms | 7.1x |
| Complex Illustration | 10,000 | 500ms | 95ms | 5.3x |
| World Map | 50,000 | 2,500ms | 425ms | 5.9x |

### Enabling Parallel Processing

The parallel feature is optional and can be enabled:

```toml
# In Cargo.toml
[dependencies]
vexy_svgo = { version = "1.0", features = ["parallel"] }
```

Or via CLI:
```bash
vexy_svgo --parallel large-file.svg
```

### Configuration

You can customize parallel processing behavior:

```rust
use vexy_svgo::{optimize, Config, ParallelConfig};

let config = Config {
    parallel: Some(ParallelConfig {
        size_threshold: 512 * 1024,    // 512KB
        element_threshold: 500,         // 500 elements
        num_threads: 4,                 // Use 4 threads
    }),
    ..Default::default()
};

let result = optimize(svg_content, &config)?;
```

## Design Principles

`vexy_svgo`'s architecture is guided by several key design principles:

-   **Performance**: Leveraging Rust's capabilities for zero-cost abstractions, memory safety, and concurrency to achieve superior optimization speeds.
-   **Functional Parity**: Ensuring that `vexy_svgo` produces identical optimization results to `svgo` for the same inputs and configurations.
-   **Modularity**: Maintaining a clear separation of concerns between parsing, optimization, and stringification, and promoting a plugin-based approach for extensibility.
-   **API Compatibility**: Designing the Rust API to be conceptually similar to `svgo`'s JavaScript API where appropriate, to ease migration for developers.
-   **WASM Readiness**: Structuring the codebase to facilitate efficient compilation to WebAssembly, enabling broad deployment scenarios.

By adhering to these principles, `vexy_svgo` aims to be a robust, high-performance, and functionally equivalent alternative to `svgo` in the Rust ecosystem.
