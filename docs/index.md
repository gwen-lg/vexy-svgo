---
# this_file: docs/index.md
layout: default
title: Home
nav_order: 1
description: "Vexy SVGO: A Native Rust SVG Optimizer"
permalink: /
---

# Vexy SVGO: A Native Rust SVG Optimizer

## 1. Introduction

`vexy-svgo` is a high-performance, native Rust port of `svgo` (SVG Optimizer), the popular Node.js-based tool for optimizing SVG vector graphics files. While `svgo` has been instrumental in reducing SVG file sizes by removing redundant information, minifying code, and applying various optimizations, `vexy-svgo` aims to bring these benefits to a new level with the power and efficiency of Rust.

This documentation serves as a comprehensive guide to `vexy-svgo`, detailing its structure, API, and plugin system. Throughout these pages, we will draw parallels and highlight key differences with the original JavaScript `svgo` reference implementation, providing context for developers familiar with `svgo` and a clear understanding for newcomers.

### 1.1. Current Status

- **Plugin Implementation**: 50 plugins implemented and fully functional
- **CLI Compatibility**: Full drop-in replacement for SVGO CLI
- **Test Coverage**: 353/353 tests passing (100% success rate)
- **SVGO Feature Parity**: High compatibility achieved
- **Build Status**: ✅ **STABLE** - Project compiles successfully
- **Performance**: **12x faster than SVGO** on npx, 7x faster on bunx

## 2. 📥 Download Vexy SVGO

<div class="text-center my-8 p-6 bg-base-100 rounded-lg border">
  <h3 class="text-2xl font-bold mb-4">Get Vexy SVGO Now</h3>
  <p class="mb-6 text-base-content/70">High-performance SVG optimization at your fingertips</p>
  
  <div class="flex flex-wrap justify-center gap-4 mb-6">
    <a href="https://github.com/vexyart/vexy-svgo/releases/latest" class="btn btn-primary" target="_blank" rel="noopener noreferrer">
      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path d="M5 20h14v-2H5v2zM19 9h-4V3H9v6H5l7 7 7-7z"/>
      </svg>
      Download Latest Release
    </a>
    <a href="https://github.com/vexyart/vexy-svgo" class="btn btn-outline" target="_blank" rel="noopener noreferrer">
      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
      </svg>
      View on GitHub
    </a>
  </div>
  
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
    <div class="flex flex-col items-center gap-2">
      <div class="w-8 h-8 bg-primary/10 rounded-full flex items-center justify-center">
        <svg class="w-4 h-4 text-primary" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
      </div>
      <strong>macOS</strong>
      <span class="text-xs text-base-content/60">Universal binary (.dmg)</span>
    </div>
    <div class="flex flex-col items-center gap-2">
      <div class="w-8 h-8 bg-primary/10 rounded-full flex items-center justify-center">
        <svg class="w-4 h-4 text-primary" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path d="M3 3h8v8H3V3zm10 0h8v8h-8V3zM3 13h8v8H3v-8zm10 0h8v8h-8v-8z"/>
        </svg>
      </div>
      <strong>Windows</strong>
      <span class="text-xs text-base-content/60">Executable (.zip)</span>
    </div>
    <div class="flex flex-col items-center gap-2">
      <div class="w-8 h-8 bg-primary/10 rounded-full flex items-center justify-center">
        <svg class="w-4 h-4 text-primary" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
      </div>
      <strong>Linux</strong>
      <span class="text-xs text-base-content/60">Binary (.tar.gz)</span>
    </div>
  </div>
</div>

## 3. Why Vexy SVGO?

The primary motivations behind developing `vexy-svgo` are rooted in the desire for superior performance, broader integration capabilities, and enhanced reliability for SVG optimization tasks.

-   **Unmatched Performance**: Leveraging Rust's focus on zero-cost abstractions, memory safety, and efficient concurrency, `vexy-svgo` processes SVG files significantly faster than its JavaScript counterpart. This makes it an ideal choice for:
    *   **Large-scale batch processing**: Optimizing thousands of SVG assets in build pipelines.
    *   **Real-time applications**: Where low latency SVG manipulation is critical.
    *   **Server-side rendering**: Reducing payload sizes and improving page load times.
-   **Seamless Native Integration**: As a native Rust library, `vexy-svgo` can be effortlessly integrated into a wide array of applications without the overhead of a Node.js runtime. This includes:
    *   **Desktop applications**: Building performant SVG tools.
    *   **Command-line interfaces (CLIs)**: Creating fast and efficient SVG optimization scripts.
    *   **Backend services**: Optimizing SVGs directly within Rust-based web servers or microservices.
    *   **Embedded systems**: Where resource constraints demand highly optimized code.
-   **WebAssembly (WASM) Compatibility**: `vexy-svgo` is meticulously designed with WebAssembly compilation in mind. This enables high-performance SVG optimization directly within web browsers, edge computing environments, or other WASM-compatible runtimes, unlocking new possibilities for client-side SVG processing.
-   **API Compatibility with `svgo`**: `vexy-svgo` strives for a high degree of API compatibility with `svgo` v4.0.0. This design choice significantly eases the transition for developers already familiar with `svgo`, allowing them to leverage their existing knowledge and configurations with minimal adjustments. Our goal is to ensure that if you know `svgo`, you'll feel right at home with `vexy-svgo`.

## 4. Key Features

`vexy-svgo` offers a robust set of features designed to provide comprehensive SVG optimization:

-   **Plugin-based Architecture**: A flexible and extensible system where individual optimization rules are encapsulated as plugins, allowing for fine-grained control over the optimization process.
-   **AST-based Transformations**: Utilizes an Abstract Syntax Tree (AST) for SVG manipulation, ensuring precise and reliable transformations.
-   **Comprehensive Optimization Plugins**: 50 production-ready plugins covering essential SVG optimizations.
-   **Enhanced CLI Tool**: Full SVGO CLI compatibility plus additional features like string input, better STDIN/STDOUT handling, and precision control.
-   **Rust Library**: A powerful and efficient Rust library for programmatic integration into your projects.
-   **WebAssembly (WASM) Compatibility**: `vexy-svgo` is meticulously designed with WebAssembly compilation in mind. This enables high-performance SVG optimization directly within web browsers, edge computing environments, or other WASM-compatible runtimes, unlocking new possibilities for client-side SVG processing. *(Details on WASM usage will be added as the WASM compilation target matures.)*
-   **Superior Performance**: **12x faster than SVGO** on npx, 7x faster on bunx for common optimization tasks.
-   **Benchmarking Tool**: A comprehensive benchmarking tool is available to compare `vexy-svgo`'s performance against `svgo`, generating Jekyll-compatible reports.

## 5. Project Structure

The `vexy-svgo` repository is organized to reflect its native Rust implementation while maintaining a clear reference to the original `svgo` structure for architectural guidance and functional parity testing:

-   **/vexy-svgo**: Contains the core Rust library and the `vexy-svgo` CLI application. This is where the primary Rust source code resides.
-   **/src**: Within the `vexy-svgo` directory, this folder holds the Rust source code for `vexy-svgo`'s core components, including the parser, optimizer, stringifier, and individual plugin implementations.
-   **/ref/svgo**: This directory contains the complete `svgo` v4.0.0 JavaScript reference implementation. It serves as a crucial benchmark for functional parity testing and provides architectural insights during the porting process.
-   **/docs**: This folder contains the project's documentation, which you are currently reading.
-   **/tests**: Comprehensive test suites for `vexy-svgo`, including integration and unit tests. Many of these tests are designed to mirror `svgo`'s test cases, ensuring that `vexy-svgo` produces identical optimization results.

## 6. Installation

To get started with `vexy-svgo`, you'll need to have Rust and Cargo (Rust's package manager) installed on your system. If you don't have them, you can install them conveniently via `rustup`, the recommended Rust toolchain installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the `rustup` installation. Once Rust and Cargo are set up, you have two primary ways to use `vexy-svgo`:

### 6.1. As a Command-Line Tool

#### 6.1.1. From Crates.io (when published)

You can install `vexy-svgo` as a global command-line tool:

```bash
cargo install vexy-svgo
```

#### 6.1.2. From Source

To build from the latest source:

```bash
git clone https://github.com/vexyart/vexy-svgo
cd vexy-svgo
cargo build --release
# The binary will be at ./target/release/vexy-svgo
```

Once installed, `vexy-svgo` can be used as a drop-in replacement for the `svgo` CLI with enhanced features.

### 6.2. As a Rust Library

To integrate `vexy-svgo` directly into your Rust project as a dependency, add it to your `Cargo.toml` file:

```toml
[dependencies]
vexy-svgo = { git = "https://github.com/vexyart/vexy-svgo" }
# Or when published to crates.io:
# vexy-svgo = "0.1.0"
```

After adding the dependency, you can use the `optimize` function:

```rust
use vexy-svgo::{optimize, config::Config};

let svg = "<svg>...</svg>";
let config = Config::default();
let optimized_svg = optimize(svg, Some(config));
```
