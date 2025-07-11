---
layout: default
title: Usage
nav_order: 2
description: "How to use Vexy SVGO CLI and library"
---

# Vexy SVGO Usage

`vexy_svgo` provides a powerful and efficient way to optimize SVG files, leveraging the performance benefits of Rust. It aims for API compatibility with `svgo` where applicable, making the transition for users familiar with the JavaScript version as smooth as possible.

## Command-Line Interface (CLI)

`vexy_svgo` offers a command-line interface with full SVGO compatibility and additional enhancements.

### Basic Usage

To optimize a single SVG file:

```bash
vexy_svgo input.svg -o output.svg
```

To use STDIN/STDOUT (default behavior when no arguments provided):

```bash
cat input.svg | vexy_svgo > output.svg
# or explicitly
vexy_svgo -i - -o -
```

To optimize a string directly:

```bash
vexy_svgo -s '<svg>...</svg>'
```

To optimize all SVG files in a folder:

```bash
vexy_svgo -f input_folder
# With recursive processing
vexy_svgo -f input_folder -r
# With exclusion patterns
vexy_svgo -f input_folder -r --exclude "node_modules|build"
```

### Options

`vexy_svgo` CLI options provide full `svgo` compatibility with additional features:

#### Input/Output Options
-   `-i, --input <FILE|DIR|->`: Input file, directory, or STDIN (`-`). Default: STDIN if no args
-   `-o, --output <FILE|DIR|->`: Output file, directory, or STDOUT (`-`). Default: STDOUT if no input file
-   `-s, --string <STRING>`: Process SVG string directly without file I/O
-   `-f, --folder <DIR>`: Process all SVG files in folder
-   `-r, --recursive`: Process folders recursively
-   `--exclude <PATTERN...>`: Exclude files matching regex patterns

#### Formatting Options
-   `--pretty`: Pretty print output SVG
-   `--indent <NUM>`: Indentation spaces (default: 2)
-   `--eol <lf|crlf>`: Line ending style (default: platform-specific)
-   `--final-newline`: Ensure trailing newline
-   `-p, --precision <NUM>`: Set numeric precision for all plugins

#### Plugin Options
-   `--config <FILE>`: Custom config file
-   `--disable <PLUGIN>`: Disable a plugin
-   `--enable <PLUGIN>`: Enable a plugin
-   `--show-plugins`: List all available plugins

#### Output Options
-   `--datauri <base64|enc|unenc>`: Output as Data URI
-   `--multipass`: Run optimizations multiple times
-   `-q, --quiet`: Only show error messages
-   `--no-color`: Disable colored output

#### Other Options
-   `-v, --version`: Show version
-   `-h, --help`: Show help

### CLI Examples

```bash
# Process multiple files
vexy_svgo icon1.svg icon2.svg icon3.svg

# Use default STDIN/STDOUT behavior
vexy_svgo < input.svg > output.svg

# Optimize with specific precision
vexy_svgo input.svg -o output.svg -p 3

# Pretty print with 4-space indentation
vexy_svgo input.svg -o output.svg --pretty --indent 4

# Process folder with exclusions
vexy_svgo -f ./assets -r --exclude "temp|backup" --exclude ".*\.min\.svg"

# Show optimization statistics
vexy_svgo large-file.svg -o optimized.svg
# Output: Optimized: 10.5 KB → 7.2 KB (31.4% reduction)
```

## As a Rust Library

`vexy_svgo` can be integrated directly into your Rust projects for programmatic SVG optimization. The core optimization function is designed to be intuitive and efficient.

### Basic Example

To use `vexy_svgo` in your Rust code, first ensure you've added it to your `Cargo.toml` (as described in the [Installation](/#installation) section). Then, you can use the `optimize` function:

```rust
use vexy_svgo::optimize;
use vexy_svgo::config::VexySvgoConfig;

fn main() {
    let svg_string = r#"
<svg xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  viewBox="0 0 100 100">
  <text x="50" y="50" text-anchor="middle">•ᴗ•</text>
</svg>
"#;

    let config = VexySvgoConfig::default();
    let result = optimize(svg_string, &config);

    match result {
        Ok(optimized_svg) => {
            println!("Optimized SVG:\n{}", optimized_svg.data);
        }
        Err(e) => {
            eprintln!("Error optimizing SVG: {}", e);
        }
    }
}
```

### Configuration

The `optimize` function in `vexy_svgo` takes an SVG string and a configuration object, similar to `svgo`'s `optimize(input, config)`.

```rust
pub struct VexySvgoConfig {
    pub path: Option<String>,
    pub plugins: Vec<PluginConfig>,
    pub multipass: bool,
    pub js2svg: Js2SvgConfig,
    pub datauri: Option<String>,
}

pub struct Js2SvgConfig {
    pub pretty: bool,
    pub indent: usize,
}

pub enum PluginConfig {
    // Represents a plugin enabled by its name (e.g., "removeDimensions")
    Enabled(String),
    // Represents a plugin with custom parameters
    WithParams {
        name: String,
        params: serde_json::Value, // Use serde_json::Value for flexible parameters
    },
}
```

**Comparison with `svgo`'s Configuration:**

`vexy_svgo`'s `VexySvgoConfig` directly maps to `svgo`'s configuration object. The `plugins` array in `svgo` can contain either plugin names (strings) or objects with `name` and `params`. In `vexy_svgo`, this is represented by the `PluginConfig` enum, allowing for both simple enablement and parameter customization.

### Example with Custom Plugins

```rust
use vexy_svgo::optimize;
use vexy_svgo::config::{VexySvgoConfig, PluginConfig};
use serde_json::json;

fn main() {
    let svg_string = r#"<svg width="100" height="100"><rect x="10" y="10" width="80" height="80" fill="red"/></svg>"#;

    let config = VexySvgoConfig {
        plugins: vec![
            PluginConfig::Enabled("removeDimensions".to_string()),
            PluginConfig::WithParams {
                name: "sortAttrs".to_string(),
                params: json!({
                    "xmlnsOrder": "alphabetical",
                }),
            },
        ],
        ..VexySvgoConfig::default()
    };

    let result = optimize(svg_string, &config);

    match result {
        Ok(optimized_svg) => {
            println!("Optimized SVG:\n{}", optimized_svg.data);
        }
        Err(e) => {
            eprintln!("Error optimizing SVG: {}", e);
        }
    }
}
```

## WebAssembly (WASM)

`vexy_svgo` is designed to be compiled to WebAssembly, allowing you to run SVG optimization directly in the browser or other WASM environments. This provides a significant performance boost compared to JavaScript-based optimizers in the browser.

Further details on WASM usage will be added as the compilation target matures.
