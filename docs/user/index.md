---
# this_file: docs/user/index.md
layout: default
title: User Guide
nav_order: 3
description: "Complete guide to using Vexy SVGO for end users"
has_children: true
permalink: /user/
---

# User Guide
{: .no_toc }

Complete guide to using Vexy SVGO for end users
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Welcome to Vexy SVGO

Vexy SVGO is a high-performance, native Rust port of the popular SVG optimizer SVGO. This guide will help you get started with optimizing your SVG files using our fast, reliable, and feature-complete optimizer.

### What Makes Vexy SVGO Special

- **🚀 12x Faster**: Native Rust performance beats Node.js implementations
- **🔄 Drop-in Compatible**: Full SVGO API compatibility 
- **🌐 Runs Everywhere**: CLI, library, and WebAssembly support
- **🛡️ Production Ready**: 353/353 tests passing, battle-tested

---

## Quick Start

### 1. Try the Interactive Demo

The fastest way to experience Vexy SVGO is through our browser-based demo:

[**🎮 Launch Demo →**](/demo/){: .btn .btn-primary }

### 2. Install Locally

Choose your preferred installation method:

#### Pre-built Binaries (Recommended)

```bash
# macOS (Universal)
curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-macos.tar.gz | tar -xz

# Windows
curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-windows.zip -o vexy-svgo.zip && unzip vexy-svgo.zip

# Linux
curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-linux.tar.gz | tar -xz
```

#### From Source

```bash
git clone https://github.com/vexyart/vexy-svgo
cd vexy-svgo
cargo build --release
# Binary will be at ./target/release/vexy-svgo
```

### 3. Basic Usage

```bash
# Optimize a single file
vexy-svgo input.svg -o output.svg

# Process multiple files
vexy-svgo *.svg

# Use with pipes
cat input.svg | vexy-svgo > output.svg

# Process entire directories
vexy-svgo -f ./icons/ -r
```

---

## User Guide Sections

<div class="d-flex flex-wrap">
  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">📦 Installation</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Complete installation guide for all platforms</p>
        <a href="/guide/installation/" class="btn btn-sm">Read more →</a>
      </div>
    </div>
  </div>

  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">💻 CLI Usage</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Master the command-line interface</p>
        <a href="/guide/cli-usage/" class="btn btn-sm">Read more →</a>
      </div>
    </div>
  </div>

  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">⚙️ Configuration</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Configure plugins and optimization settings</p>
        <a href="/guide/configuration/" class="btn btn-sm">Read more →</a>
      </div>
    </div>
  </div>

  <div class="flex-auto mr-4 mb-4" style="min-width: 250px;">
    <div class="Box">
      <div class="Box-header">
        <h3 class="Box-title">🧩 Plugins</h3>
      </div>
      <div class="Box-body">
        <p class="mb-2">Complete plugin reference and usage</p>
        <a href="/guide/plugins/" class="btn btn-sm">Read more →</a>
      </div>
    </div>
  </div>
</div>

---

## Common Tasks

### Optimize for Web

```bash
# Standard web optimization
vexy-svgo input.svg -o output.svg --pretty

# Remove unnecessary precision
vexy-svgo input.svg -o output.svg -p 2

# Maximum compression
vexy-svgo input.svg -o output.svg --multipass
```

### Batch Processing

```bash
# Process all SVGs in a directory
vexy-svgo -f ./assets/icons/ -r

# With specific plugins
vexy-svgo -f ./icons/ --disable removeViewBox --enable cleanupIds

# Custom output directory
vexy-svgo -f ./src/icons/ -o ./dist/icons/ -r
```

### Integration Examples

```bash
# Build pipeline
npm run build && find dist -name "*.svg" -exec vexy-svgo {} \;

# Git pre-commit hook
git diff --cached --name-only --diff-filter=A | grep '\.svg$' | xargs vexy-svgo

# Watch mode (with external tool)
watchman-make -p '**/*.svg' -t optimize-svgs
```

---

## Performance Tips

### 1. Use Native Binary
The native binary is always fastest. Avoid wrappers when possible.

### 2. Batch Processing
Process multiple files at once rather than one-by-one:

```bash
# ✅ Fast - batch processing
vexy-svgo *.svg

# ❌ Slow - individual processing
for file in *.svg; do vexy-svgo "$file"; done
```

### 3. Optimize Plugin Selection
Disable unnecessary plugins for faster processing:

```bash
# For build pipelines, focus on size reduction
vexy-svgo input.svg --disable removeTitle --disable removeDesc

# For icon sprites, keep structure
vexy-svgo input.svg --disable removeViewBox --disable removeDimensions
```

---

## Migration from SVGO

Vexy SVGO is designed as a drop-in replacement for SVGO. Most existing workflows will work unchanged:

```bash
# Replace this
npx svgo input.svg -o output.svg

# With this
vexy-svgo input.svg -o output.svg
```

### Key Differences

| Feature | SVGO | Vexy SVGO |
|---------|------|-----------|
| **Performance** | Baseline | 12x faster |
| **Installation** | npm install | Single binary |
| **Config Files** | .svgo.config.js | .svgo.config.json |
| **Plugin API** | JavaScript | Rust (for custom plugins) |

### Configuration Migration

Most SVGO configurations work directly:

```bash
# Your existing config works
vexy-svgo --config .svgo.config.js input.svg
```

---

## Getting Help

- **📖 Documentation**: Continue reading this guide
- **🎮 Demo**: Try our [interactive demo](/demo/)
- **🐛 Issues**: Report problems on [GitHub](https://github.com/vexyart/vexy-svgo/issues)
- **💬 Discussions**: Join [GitHub Discussions](https://github.com/vexyart/vexy-svgo/discussions)

---

*Ready to get started? Continue to [Installation](/guide/installation/) →*