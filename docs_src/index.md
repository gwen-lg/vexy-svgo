---
nav_weight: 10
# this_file: docs/index.md
layout: default
title: Vexy SVGO
nav_order: 1
description: "High-performance SVG optimizer - Native Rust port of SVGO"
permalink: /
---

# Vexy SVGO
{: .fs-9 }

High-performance SVG optimizer - Native Rust port of SVGO
{: .fs-6 .fw-300 }

[Get Started Now](/user/){: .btn .btn-primary .fs-5 .mb-4 .mb-md-0 .mr-2 }
[Try Demo](/demo/){: .btn .fs-5 .mb-4 .mb-md-0 .mr-2 }
[View on GitHub](https://github.com/vexyart/vexy-svgo){: .btn .fs-5 .mb-4 .mb-md-0 }

---

## ⚡ **12x Faster** than SVGO
{: .text-center }

Native Rust performance with full SVGO API compatibility
{: .text-center .fs-6 .fw-300 }

<div class="d-flex flex-justify-around flex-wrap my-6">
  <div class="text-center mb-4">
    <div class="fs-8 text-green-000">🚀</div>
    <div class="fw-500">Native Performance</div>
    <div class="fs-3 text-grey-dk-000">Rust-powered optimization</div>
  </div>
  <div class="text-center mb-4">
    <div class="fs-8 text-blue-000">🔄</div>
    <div class="fw-500">Drop-in Replacement</div>
    <div class="fs-3 text-grey-dk-000">Full SVGO compatibility</div>
  </div>
  <div class="text-center mb-4">
    <div class="fs-8 text-purple-000">🌐</div>
    <div class="fw-500">WebAssembly Ready</div>
    <div class="fs-3 text-grey-dk-000">Runs in browsers</div>
  </div>
</div>

---

## Quick Start

### Try Online Demo
Experience Vexy SVGO's power directly in your browser with our interactive WebAssembly demo.

[**🎮 Launch Interactive Demo →**](/demo/){: .btn .btn-outline .btn-lg }

### Install Locally

```bash
# Download pre-built binary
curl -sSL https://github.com/vexyart/vexy-svgo/releases/latest/download/vexy-svgo-macos.tar.gz | tar -xz

# Or build from source
git clone https://github.com/vexyart/vexy-svgo
cd vexy-svgo
cargo build --release
```

### Use as Library

```rust
use vexy_svgo::{optimize, Config};

let svg = "<svg>...</svg>";
let config = Config::default();
let result = optimize(svg, &config)?;
println!("Optimized: {}", result.data);
```

---

## Documentation Paths

<div class="d-flex flex-column flex-md-row mb-6">
  <div class="flex-auto mr-0 mr-md-4 mb-4 mb-md-0">
    <div class="Box Box--blue mb-3">
      <div class="Box-header">
        <h3 class="Box-title">👤 End Users</h3>
      </div>
      <div class="Box-body">
        <p class="mb-3">Getting started with Vexy SVGO CLI and basic usage.</p>
        <nav class="menu">
          <a href="/user/" class="menu-item">📖 User Guide</a>
          <a href="/user/installation/" class="menu-item">📦 Installation</a>
          <a href="/user/cli-usage/" class="menu-item">💻 CLI Usage</a>
          <a href="/user/configuration/" class="menu-item">⚙️ Configuration</a>
          <a href="/demo/" class="menu-item">🎮 Interactive Demo</a>
        </nav>
      </div>
    </div>
  </div>

  <div class="flex-auto">
    <div class="Box Box--purple mb-3">
      <div class="Box-header">
        <h3 class="Box-title">👨‍💻 Developers</h3>
      </div>
      <div class="Box-body">
        <p class="mb-3">APIs, architecture, and contributing to Vexy SVGO.</p>
        <nav class="menu">
          <a href="/developer/" class="menu-item">🔧 Developer Guide</a>
          <a href="/developer/api-reference/" class="menu-item">📚 API Reference</a>
          <a href="/developer/architecture/" class="menu-item">🏗️ Architecture</a>
          <a href="/developer/plugin-development/" class="menu-item">🧩 Plugin Development</a>
          <a href="/developer/contributing/" class="menu-item">🤝 Contributing</a>
        </nav>
      </div>
    </div>
  </div>
</div>

---

## Key Features

- **🏆 Superior Performance**: 12x faster than SVGO on npx, 7x faster on bunx
- **🔗 API Compatible**: Drop-in replacement for existing SVGO workflows  
- **🧩 50+ Plugins**: Full port of SVGO's optimization plugins
- **🌐 WebAssembly**: Runs in browsers with native performance
- **⚡ Multi-platform**: Native binaries for macOS, Windows, and Linux
- **📦 Multiple Interfaces**: CLI, Rust library, and WASM module
- **🛡️ Production Ready**: 353/353 tests passing, battle-tested

---

## Performance Comparison

| Tool | NPX Time | Bunx Time | Native Time |
|------|----------|-----------|-------------|
| SVGO | 847ms | 112ms | - |
| **Vexy SVGO** | **71ms** | **16ms** | **8ms** |
| **Speedup** | **12x** | **7x** | **105x** |

*Benchmark: 100 complex SVG files, average optimization time*

---

## Community & Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/vexyart/vexy-svgo/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/vexyart/vexy-svgo/discussions)  
- 📧 **Contact**: [Project Maintainers](mailto:twardoch@github.com)
- 📖 **Documentation**: You're reading it!

---

*Vexy SVGO is open source software released under the MIT License.*
