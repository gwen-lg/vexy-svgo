# Vexy SVGO Improvement Plan

## 1. Overview

This document outlines a comprehensive plan to improve and fix the Vexy SVGO codebase after the rename from SVGN. The project has been successfully renamed and builds correctly, but several areas need attention for production readiness.


### 1.1. Identified Issues

#### 1.1.1. Code Quality Issues

- [x] **Unused imports and variables**: Multiple warnings about unused code - Fixed all 49 warnings
- [x] **Dead code**: Functions like `escape_attribute_to` are never used - Fixed all dead code warnings
- [ ] **Incomplete implementations**: Some functions have TODO comments indicating missing functionality

#### 1.1.2. Structural Issues

- [x] **Duplicate PluginConfig types**: There are two different PluginConfig types (enum vs struct) causing confusion - Only one PluginConfig type exists
- [x] **Missing Plugin cloning**: The `get_plugins_by_category` function can't properly clone plugins - Factory pattern correctly implemented
- [ ] **Incomplete streaming parser**: Some streaming functionality appears unfinished
- [ ] **WASM compilation failures**: Multiple API changes and missing dependencies (see issues/621.txt)

#### 1.1.3. Missing Features

- [x] **Parallel processing**: Feature flag exists but implementation may be incomplete - Implementation is complete with rayon, feature is optional
- [x] **Plugin factory pattern**: Needed for proper plugin instantiation - Already implemented
- [x] **Error handling**: Some error paths use generic string errors instead of typed errors - Typed error system already implemented
- [ ] **Platform deliverables**: Windows and Linux packages pending (cross-compilation required)


### 1.2. Phase 1: Code Cleanup (Immediate)

1. **Remove unused code**

- [x] Clean up all unused imports - Fixed all 49 warnings in plugin-sdk
- [x] Remove or implement unused functions - Completed
- [x] Fix all compiler warnings - All warnings resolved
- [x] Fix unused variable warnings (e.g., `file_path` is never read) - Fixed

2. **Fix structural issues**

- [x] Consolidate PluginConfig types into a single, well-designed type - Only one type exists
- [x] Implement proper plugin cloning or factory pattern - Factory pattern already implemented
- [x] Complete the streaming parser implementation - fixed quick-xml 0.31 configuration

3. **Improve error handling** - COMPLETED
- [x] Create typed error enums for different error categories - Already implemented
- [x] Replace string errors with proper error types - Mostly done, using thiserror
- [x] Add context to errors for better debugging - DetailedParseError provides context
- [x] Implement `std::error::Error` trait for all error types - Done via thiserror

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

## 5. Immediate Issues from Latest Build (2025-07-13)

### 5.1. Build Issues Found

1. **WASM Optimization Tools Missing** (Low Priority)
   - wasm-opt and wasm-snip not found, but builds complete successfully
   - These are optional optimizations

2. **Test Build Failure** (CRITICAL)
   - `vexy-svgo-test-utils` fails to compile: `can't find crate for 'svgn'`
   - This is a remnant from the SVGN rename that needs to be fixed

3. **Compilation Warnings** (High Priority) - FIXED
   - [x] 49 warnings in vexy-svgo-plugin-sdk - All warnings resolved
   - [x] Unused imports, dead code, unused fields - Fixed
   - [x] Unexpected cfg conditions for "dynamic-loading" feature - Fixed

4. **WASM Dead Code** (Medium Priority) - FIXED
   - [x] `StreamingState::Error` variant is never constructed in enhanced.rs - Actually is being used

5. **Missing LICENSE File** (Medium Priority) - FIXED
   - [x] License key is set in Cargo.toml but no LICENSE file found - LICENSE file exists

### 5.2. Release Issues Found

1. **Uncommitted Changes** (Immediate)
   - Release failed due to uncommitted documentation changes
   - Working directory must be clean before release

2. **Version Mismatch** (High Priority)
   - Build shows version 1.5.1
   - Release trying to increment from 1.0.23 to 1.0.24
   - Version inconsistency needs to be resolved

## 6. Conclusion

The Vexy SVGO project has a solid foundation but needs systematic improvements to reach production quality. This plan prioritizes immediate fixes to get a working release, followed by feature completion and long-term optimizations. The focus should be on maintaining SVGO compatibility while leveraging Rust's performance advantages.

### 6.1. Recent Progress

- [x] WASM builds are now successful (with optional optimization warnings)
- [x] macOS universal binary builds successfully

### 6.2. Known Issues

1. **Test Utils Build Failure** (CRITICAL Priority)
   - Missing crate reference to old 'svgn' name
   - Blocks test execution

2. **Code Quality Issues** (High Priority)
   - 49 compilation warnings need to be addressed
   - Unused code and imports throughout plugin-sdk

3. **Version Management** (High Priority)
   - Version mismatch between build (1.5.1) and release (1.0.x)
   - Need consistent versioning strategy

## 7. Release Blockers (2025-07-12)

### 7.1. Critical Issues to Fix Before Release

1. **Git Working Directory Must Be Clean**
   - Files modified: TODO.md, build.err.txt, dist/vexy-svgo-1.5.1-macos-universal.tar.gz, issues/121.txt
   - Need to either commit or revert these changes before release

2. **Version Consistency**
   - Cargo.toml shows version 1.5.1
   - Release script trying to increment from 1.0.23 to 1.0.24
   - Need to align version numbering across all files

### 7.2. Immediate Action Items

1. **Clean Git Status**
   - [ ] Review and commit necessary changes
   - [ ] Ensure all build artifacts are in .gitignore

2. **Fix Version Numbering**
   - [ ] Decide on correct version (1.0.24 or 1.5.1)
   - [ ] Update all version references consistently
   - [ ] Ensure release script uses correct version

3. **Build Process**
   - [ ] The build.sh script completed successfully
   - [ ] WASM build started but was cut off (may need timeout increase)
   - [ ] Consider separating WASM build from main build process
