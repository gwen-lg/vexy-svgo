---
this_file: docs/_tools/index.md
layout: default
title: Vexy SVGO Tools
description: Online tools for SVG optimization and manipulation
permalink: /tools/
nav_order: 6
---

# Vexy SVGO Tools

A collection of web-based tools for SVG optimization and manipulation, featuring both client-side and server-side processing options.

## Available Tools

### [Vexy SVGO WebAssembly Optimizer](/tools/optimizer/)
Our flagship **client-side** SVG optimization tool. Upload your SVG files and optimize them directly in your browser using WebAssembly technology - no server processing required.

**Features:**
- 🔒 **Complete Privacy**: All processing happens locally in your browser - files never leave your device
- ⚡ **12x Faster**: Native Rust performance compiled to WebAssembly, 12x faster than SVGO
- 🎛️ **Full SVGO Compatibility**: API-compatible port supporting all major SVGO plugins
- 📊 **Real-time Preview**: See optimizations applied instantly with no network delays
- 📦 **Batch Processing**: Upload and optimize multiple files simultaneously
- 🌐 **Works Offline**: No server required - works completely offline

[**Launch Vexy SVGO WebAssembly Tool →**](/tools/optimizer/){: .btn .btn-primary .btn-lg}

---

### [SVGO Server Optimizer](/tools/svgo-optimizer/)
Traditional **server-side** SVG optimization using the original SVGO JavaScript implementation. Files are processed on our servers for compatibility with complex optimization scenarios.

**Features:**
- 🔧 **Original SVGO**: Uses the proven JavaScript implementation with 100% compatibility
- 🖥️ **Server Processing**: No browser limitations on file size or complexity
- 📊 **Real-time Preview**: See optimizations applied instantly
- 📦 **Batch Processing**: Upload and optimize multiple files at once
- 🔒 **Secure Processing**: Files processed securely and not stored on servers
- ⚙️ **Advanced Options**: Access to all SVGO configuration options

[**Launch SVGO Server Tool →**](/tools/svgo-optimizer/){: .btn .btn-secondary .btn-lg}

---

## Tool Comparison

<div class="overflow-x-auto">
  <table class="table w-full">
    <thead>
      <tr>
        <th>Feature</th>
        <th>Vexy SVGO WebAssembly</th>
        <th>SVGO Server</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>**Privacy**</td>
        <td>🟢 Complete (local processing)</td>
        <td>🟡 Good (secure server processing)</td>
      </tr>
      <tr>
        <td>**Performance**</td>
        <td>🟢 12x faster than SVGO</td>
        <td>🟡 Standard SVGO speed</td>
      </tr>
      <tr>
        <td>**Compatibility**</td>
        <td>🟢 API-compatible SVGO port</td>
        <td>🟢 100% original SVGO</td>
      </tr>
      <tr>
        <td>**File Size Limits**</td>
        <td>🟡 Browser memory dependent</td>
        <td>🟢 No practical limits</td>
      </tr>
      <tr>
        <td>**Offline Usage**</td>
        <td>🟢 Works completely offline</td>
        <td>🔴 Requires internet connection</td>
      </tr>
      <tr>
        <td>**Implementation**</td>
        <td>Rust → WebAssembly</td>
        <td>Original JavaScript</td>
      </tr>
    </tbody>
  </table>
</div>

---

## Coming Soon

### SVG Analyzer
Analyze SVG files for potential optimization opportunities and structural issues.

### SVG Converter  
Convert between different SVG formats and export to other vector formats.

### SVG Validator
Validate SVG files against web standards and accessibility guidelines.

### Plugin Tester
Test individual SVGO plugins to understand their specific optimizations.

---

## Technical Details

### Vexy SVGO WebAssembly Tool
- **Backend**: Vexy SVGO (Rust) compiled to WebAssembly
- **Processing**: Client-side in browser memory
- **Privacy**: Complete - files never leave your device
- **Performance**: 12x faster than SVGO
- **Dependencies**: WebAssembly runtime support

### SVGO Server Tool
- **Backend**: Original SVGO JavaScript implementation
- **Processing**: Server-side with secure handling
- **Compatibility**: 100% original SVGO behavior
- **File Limits**: No practical size restrictions
- **Dependencies**: Internet connection required

### Shared Infrastructure
- **Frontend**: Modern JavaScript with progressive enhancement
- **UI**: DaisyUI + Tailwind CSS for responsive design
- **Documentation**: Jekyll with just-the-docs theme
- **Development**: Rust toolchain with comprehensive test suite

## Browser Requirements

These tools require a modern browser with WebAssembly support:

- **Chrome**: 57+ / Chromium 57+
- **Firefox**: 52+
- **Safari**: 11+
- **Edge**: 16+

## Performance

WebAssembly allows us to achieve near-native performance in the browser:

- **File Processing**: 2-3x faster than JavaScript implementations
- **Memory Usage**: Efficient memory management with predictable performance
- **Bundle Size**: Optimized WASM bundles under 2MB compressed
- **Load Time**: Fast initialization and caching for repeated use

## Open Source

All tools are open source and available on GitHub:

- [Vexy SVGO Core](https://github.com/vexyart/vexy-svgo) - The Rust implementation
- [Web Tools](https://github.com/vexyart/vexy-svgo/tree/main/docs) - Frontend code for these tools

## Feedback

Found a bug or have a feature request? Please [open an issue](https://github.com/vexyart/vexy-svgo/issues) on GitHub.

---

*Last updated: {{ site.time | date: '%B %d, %Y' }}*