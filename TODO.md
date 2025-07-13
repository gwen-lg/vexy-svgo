# Vexy SVGO TODO List - Linearized Action Items

PRIORITY TOP: `release.sh` must not touch `rust-version = "1.58.0"` in Cargo.toml

## 0. Release Blockers (IMMEDIATE - Fix Before Release)

### 0.1. Clean Git Working Directory

- [ ] Commit or revert changes to: TODO.md, build.err.txt, dist/vexy-svgo-1.5.1-macos-universal.tar.gz, issues/121.txt
- [ ] Ensure build artifacts are in .gitignore

### 0.2. Fix Version Consistency

- [ ] Decide on correct version: 1.0.24 or 1.5.1
- [ ] Update all version references consistently
- [ ] Fix release script to use correct version (currently trying 1.0.23 -> 1.0.24 but build shows 1.5.1)

### 0.3. Build Process Issues

- [ ] WASM build timeout - consider increasing timeout or separating WASM build
- [ ] Install optional WASM optimization tools (wasm-opt, wasm-snip) - low priority

## 1. Critical Fixes (Fix Before Next Build)

### 1.1. Fix Test Build Failure

- [x] Fix `vexy-svgo-test-utils` compilation error: replace 'svgn' crate reference with 'vexy-svgo'
- [x] Search for any remaining 'svgn' references in the codebase

### 1.2. Add Missing LICENSE File

- [x] Create LICENSE file in repository root (MIT license already exists)

## 2. Phase 1: Code Cleanup (After Release)

### 2.1. Remove unused code

- [x] Clean up all unused imports (49 warnings in plugin-sdk)
- [x] Remove or implement unused functions
- [x] Fix all compiler warnings
- [x] Fix unused variable warnings (e.g., `file_path` is never read)
- [x] Fix dead code warnings (unused struct fields, methods)
- [x] Remove or use `StreamingState::Error` variant in WASM enhanced.rs - already being used
- [x] Fix unexpected cfg condition for "dynamic-loading" feature

### 2.2. Fix structural issues

- [x] Consolidate PluginConfig types into a single, well-designed type - only one type exists
- [x] Implement proper plugin cloning or factory pattern - factory pattern already implemented
- [x] Complete the streaming parser implementation - fixed quick-xml 0.31 configuration

### 2.3. Improve error handling

- [x] Create typed error enums for different error categories - Already implemented
- [x] Replace string errors with proper error types - Mostly done, using thiserror
- [x] Add context to errors for better debugging - DetailedParseError provides context
- [x] Implement `std::error::Error` trait for all error types - Done via thiserror

## 3. Phase 2: Feature Completion (Short-term)

### 3.1. Complete parallel processing

- [x] Verify parallel feature implementation - Complete with rayon, optional feature
- [x] Fix Rayon imports and usage - Already working correctly
- [x] Add tests for parallel execution - Tests exist in parallel.rs
- [x] Document thread pool configuration - Documented in ParallelConfig
- [ ] Document performance benefits - Still needs benchmarks

### 3.2. Plugin system improvements

- [ ] Implement plugin factory pattern
- [ ] Add plugin validation
- [ ] Create plugin testing framework
- [ ] Document plugin API

### 3.3. CLI enhancements

- [ ] Add progress indicators for folder processing
- [ ] Implement proper color output support
- [ ] Add verbose logging options
- [ ] Add `--dry-run` option

## 4. Phase 3: Testing & Documentation (Medium-term)

### 4.1. Comprehensive testing

- [ ] Unit tests for all core functionality (AST, parser, optimizer modules)
- [ ] Integration tests for CLI
- [ ] Performance benchmarks
- [ ] Compatibility tests with SVGO configs

### 4.2. Documentation

- [ ] API documentation for all public types (generate with rustdoc)
- [ ] Plugin development guide
- [ ] Migration guide from SVGO
- [ ] Performance tuning guide
- [ ] Add inline documentation for all public APIs

### 4.3. Examples

- [ ] CLI usage examples
- [ ] Plugin development examples
- [ ] Integration examples (Node.js, Python, etc.)
- [ ] Create WebAssembly usage examples

## 5. Phase 4: Performance Optimization (Long-term)

### 5.1. Memory optimization

- [ ] Profile memory usage for large SVG files
- [ ] Implement memory-efficient parsing strategies (including streaming for very large files)
- [ ] Add memory usage limits and controls
- [ ] Optimize AST memory layout

### 5.2. Speed optimization

- [ ] Benchmark against SVGO (create comprehensive benchmarks)
- [ ] Optimize hot paths (profile and optimize)
- [ ] Implement SIMD optimizations where applicable (for path data)
- [ ] Add parallel path processing

### 5.3. Streaming improvements

- [ ] Complete streaming parser implementation
- [ ] Add streaming output support
- [ ] Implement incremental optimization
- [ ] Add chunked processing for large files

## 6. Technical Debt Items

### 6.1. Build verification

- [ ] Add build verification steps
- [ ] Create reproducible builds

### 6.2. Import/Export organization

- [ ] Review and reorganize public API exports
- [ ] Ensure consistent naming conventions
- [ ] Clean up module structure
- [ ] Remove duplicate code

### 6.3. Configuration system

- [ ] Validate configuration loading and merging
- [ ] Add configuration schema validation
- [ ] Support for .svgo.config.js compatibility
- [ ] Add configuration migration tool

## 7. Quality Assurance

### 7.1. Continuous Integration

- [ ] Set up GitHub Actions for automated testing
- [ ] Add coverage reporting (with codecov/coveralls)
- [ ] Implement automated benchmarking
- [ ] Add cross-platform testing

### 7.2. Code quality tools

- [ ] Configure clippy with strict lints
- [ ] Add rustfmt configuration (with project style)
- [ ] Set up pre-commit hooks
- [ ] Add commit message linting

### 7.3. Release process

- [ ] Automated version bumping
- [ ] Changelog generation
- [ ] Binary distribution for multiple platforms
- [ ] Set up crates.io publishing
- [ ] Create Homebrew formula
- [ ] Create npm package wrapper

## 8. Nice-to-Have Features

- [ ] Add SVG validation before optimization
- [ ] Implement SVG diff tool
- [ ] Add batch processing with glob patterns
- [ ] Create GUI wrapper
- [ ] Add plugin marketplace integration
- [ ] Implement custom plugin loader
- [ ] Add telemetry/analytics (opt-in)
- [ ] Create VS Code extension
- [ ] Add Docker image
- [ ] Implement watch mode for development
