# Vexy SVGO Improvement Plan

## 1. Overview

This document outlines a comprehensive plan to improve and fix the Vexy SVGO codebase after the rename from SVGN. The project has been successfully renamed and builds correctly, but several areas need attention for production readiness.


### 1.1. Identified Issues

#### 1.1.1. Code Quality Issues

- [ ] **Unused imports and variables**: Multiple warnings about unused code
- [ ] **Dead code**: Functions like `escape_attribute_to` are never used
- [ ] **Incomplete implementations**: Some functions have TODO comments indicating missing functionality

#### 1.1.2. Structural Issues

- [ ] **Duplicate PluginConfig types**: There are two different PluginConfig types (enum vs struct) causing confusion
- [ ] **Missing Plugin cloning**: The `get_plugins_by_category` function can't properly clone plugins
- [ ] **Incomplete streaming parser**: Some streaming functionality appears unfinished
- [ ] **WASM compilation failures**: Multiple API changes and missing dependencies (see issues/621.txt)

#### 1.1.3. Missing Features

- [ ] **Parallel processing**: Feature flag exists but implementation may be incomplete
- [ ] **Plugin factory pattern**: Needed for proper plugin instantiation
- [ ] **Error handling**: Some error paths use generic string errors instead of typed errors
- [ ] **Platform deliverables**: Windows and Linux packages pending (cross-compilation required)


### 1.2. Phase 1: Code Cleanup (Immediate)

1. **Remove unused code**

- [ ] Clean up all unused imports
- [ ] Remove or implement unused functions
- [ ] Fix all compiler warnings
- [ ] Fix unused variable warnings (e.g., `file_path` is never read)

2. **Fix structural issues**

- [ ] Consolidate PluginConfig types into a single, well-designed type
- [ ] Implement proper plugin cloning or factory pattern
- [ ] Complete the streaming parser implementation

3. **Improve error handling**
- [ ] Create typed error enums for different error categories
- [ ] Replace string errors with proper error types
- [ ] Add context to errors for better debugging
- [ ] Implement `std::error::Error` trait for all error types

### 1.3. Phase 2: Feature Completion (Short-term)

1. **Complete parallel processing**

- [ ] Verify parallel feature implementation
- [ ] Fix Rayon imports and usage
- [ ] Add tests for parallel execution
- [ ] Document thread pool configuration
- [ ] Document performance benefits

2. **Plugin system improvements**

- [ ] Implement plugin factory pattern
- [ ] Add plugin validation
- [ ] Create plugin testing framework
- [ ] Document plugin API

3. **CLI enhancements**
- [ ] Add progress indicators for folder processing
- [ ] Implement proper color output support
- [ ] Add verbose logging options
- [ ] Add `--dry-run` option

### 1.4. Phase 3: Testing & Documentation (Medium-term)

1. **Comprehensive testing**

- [ ] Unit tests for all core functionality (AST, parser, optimizer modules)
- [ ] Integration tests for CLI
- [ ] Performance benchmarks
- [ ] Compatibility tests with SVGO configs

2. **Documentation**

- [ ] API documentation for all public types (generate with rustdoc)
- [ ] Plugin development guide
- [ ] Migration guide from SVGO
- [ ] Performance tuning guide
- [ ] Add inline documentation for all public APIs

3. **Examples**
- [ ] CLI usage examples
- [ ] Plugin development examples
- [ ] Integration examples (Node.js, Python, etc.)
- [ ] Create WebAssembly usage examples

### 1.5. Phase 4: Performance Optimization (Long-term)

1. **Memory optimization**

- [ ] Profile memory usage for large SVG files
- [ ] Implement memory-efficient parsing strategies (including streaming for very large files)
- [ ] Add memory usage limits and controls
- [ ] Optimize AST memory layout

2. **Speed optimization**

- [ ] Benchmark against SVGO (create comprehensive benchmarks)
- [ ] Optimize hot paths (profile and optimize)
- [ ] Implement SIMD optimizations where applicable (for path data)
- [ ] Add parallel path processing

3. **Streaming improvements**
- [ ] Complete streaming parser implementation
- [ ] Add streaming output support
- [ ] Implement incremental optimization
- [ ] Add chunked processing for large files

## 2. Technical Debt Items

1. **Build verification**

- [ ] Add build verification steps
- [ ] Create reproducible builds

2. **Import/Export organization**

- [ ] Review and reorganize public API exports
- [ ] Ensure consistent naming conventions
- [ ] Clean up module structure
- [ ] Remove duplicate code

3. **Configuration system**
- [ ] Validate configuration loading and merging
- [ ] Add configuration schema validation
- [ ] Support for .svgo.config.js compatibility
- [ ] Add configuration migration tool

## 3. Quality Assurance

1. **Continuous Integration**

- [ ] Set up GitHub Actions for automated testing
- [ ] Add coverage reporting (with codecov/coveralls)
- [ ] Implement automated benchmarking
- [ ] Add cross-platform testing

2. **Code quality tools**

- [ ] Configure clippy with strict lints
- [ ] Add rustfmt configuration (with project style)
- [ ] Set up pre-commit hooks
- [ ] Add commit message linting

3. **Release process**
- [ ] Automated version bumping
- [ ] Changelog generation
- [ ] Binary distribution for multiple platforms
- [ ] Set up crates.io publishing
- [ ] Create Homebrew formula
- [ ] Create npm package wrapper

## 4. Nice-to-Have Features

1. **SVG validation**
- [ ] Add SVG validation before optimization
2. **SVG diff tool**
- [ ] Implement SVG diff tool
3. **Batch processing**
- [ ] Add batch processing with glob patterns
4. **GUI wrapper**
- [ ] Create GUI wrapper
5. **Plugin marketplace integration**
- [ ] Add plugin marketplace integration
6. **Custom plugin loader**
- [ ] Implement custom plugin loader
7. **Telemetry/analytics**
- [ ] Add telemetry/analytics (opt-in)
8. **VS Code extension**
- [ ] Create VS Code extension
9. **Docker image**
- [ ] Add Docker image
10. **Watch mode**
- [ ] Implement watch mode for development

## 5. Conclusion

The Vexy SVGO project has a solid foundation but needs systematic improvements to reach production quality. This plan prioritizes immediate fixes to get a working release, followed by feature completion and long-term optimizations. The focus should be on maintaining SVGO compatibility while leveraging Rust's performance advantages.

### 5.1. Recent Progress

- [ ] ⚠️ WASM build still failing due to API changes

### 5.2. Known Issues

1. **WASM Build Failures** (High Priority)

- [ ] Missing imports and changed APIs
- [ ] Missing web-sys dependency
- [ ] wasm-bindgen String handling issues
- [ ] See issues/621.txt for detailed analysis

2. **Platform Deliverables** (Medium Priority)
- [ ] Windows and Linux packages require cross-compilation setup
- [ ] macOS packages successfully created
