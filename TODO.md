# Vexy SVGO TODO List - Linearized Action Items

## 1. Phase 1: Code Cleanup (Immediate)

### 1.1. Remove unused code

- [ ] Clean up all unused imports
- [ ] Remove or implement unused functions
- [ ] Fix all compiler warnings
- [ ] Fix unused variable warnings (e.g., `file_path` is never read)

### 1.2. Fix structural issues

- [ ] Consolidate PluginConfig types into a single, well-designed type
- [ ] Implement proper plugin cloning or factory pattern
- [ ] Complete the streaming parser implementation

### 1.3. Improve error handling

- [ ] Create typed error enums for different error categories
- [ ] Replace string errors with proper error types
- [ ] Add context to errors for better debugging
- [ ] Implement `std::error::Error` trait for all error types

## 2. Phase 2: Feature Completion (Short-term)

### 2.1. Complete parallel processing

- [ ] Verify parallel feature implementation
- [ ] Fix Rayon imports and usage
- [ ] Add tests for parallel execution
- [ ] Document thread pool configuration
- [ ] Document performance benefits

### 2.2. Plugin system improvements

- [ ] Implement plugin factory pattern
- [ ] Add plugin validation
- [ ] Create plugin testing framework
- [ ] Document plugin API

### 2.3. CLI enhancements

- [ ] Add progress indicators for folder processing
- [ ] Implement proper color output support
- [ ] Add verbose logging options
- [ ] Add `--dry-run` option

## 3. Phase 3: Testing & Documentation (Medium-term)

### 3.1. Comprehensive testing

- [ ] Unit tests for all core functionality (AST, parser, optimizer modules)
- [ ] Integration tests for CLI
- [ ] Performance benchmarks
- [ ] Compatibility tests with SVGO configs

### 3.2. Documentation

- [ ] API documentation for all public types (generate with rustdoc)
- [ ] Plugin development guide
- [ ] Migration guide from SVGO
- [ ] Performance tuning guide
- [ ] Add inline documentation for all public APIs

### 3.3. Examples

- [ ] CLI usage examples
- [ ] Plugin development examples
- [ ] Integration examples (Node.js, Python, etc.)
- [ ] Create WebAssembly usage examples

## 4. Phase 4: Performance Optimization (Long-term)

### 4.1. Memory optimization

- [ ] Profile memory usage for large SVG files
- [ ] Implement memory-efficient parsing strategies (including streaming for very large files)
- [ ] Add memory usage limits and controls
- [ ] Optimize AST memory layout

### 4.2. Speed optimization

- [ ] Benchmark against SVGO (create comprehensive benchmarks)
- [ ] Optimize hot paths (profile and optimize)
- [ ] Implement SIMD optimizations where applicable (for path data)
- [ ] Add parallel path processing

### 4.3. Streaming improvements

- [ ] Complete streaming parser implementation
- [ ] Add streaming output support
- [ ] Implement incremental optimization
- [ ] Add chunked processing for large files

## 5. Technical Debt Items

### 5.1. Build verification

- [ ] Add build verification steps
- [ ] Create reproducible builds

### 5.2. Import/Export organization

- [ ] Review and reorganize public API exports
- [ ] Ensure consistent naming conventions
- [ ] Clean up module structure
- [ ] Remove duplicate code

### 5.3. Configuration system

- [ ] Validate configuration loading and merging
- [ ] Add configuration schema validation
- [ ] Support for .svgo.config.js compatibility
- [ ] Add configuration migration tool

## 6. Quality Assurance

### 6.1. Continuous Integration

- [ ] Set up GitHub Actions for automated testing
- [ ] Add coverage reporting (with codecov/coveralls)
- [ ] Implement automated benchmarking
- [ ] Add cross-platform testing

### 6.2. Code quality tools

- [ ] Configure clippy with strict lints
- [ ] Add rustfmt configuration (with project style)
- [ ] Set up pre-commit hooks
- [ ] Add commit message linting

### 6.3. Release process

- [ ] Automated version bumping
- [ ] Changelog generation
- [ ] Binary distribution for multiple platforms
- [ ] Set up crates.io publishing
- [ ] Create Homebrew formula
- [ ] Create npm package wrapper

## 7. Nice-to-Have Features

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