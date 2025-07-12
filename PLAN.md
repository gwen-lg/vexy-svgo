# Vexy SVGO Improvement Plan

## Overview

This document outlines a comprehensive plan to improve and fix the Vexy SVGO codebase after the rename from SVGN. The project has been successfully renamed and builds correctly, but several areas need attention for production readiness.

## Current Status

### Completed Tasks
1. ✅ Fixed all import statements from `vexy_svgo_core` to `vexy_svgo_core`
2. ✅ Updated all SVGN references in comments to Vexy SVGO
3. ✅ Fixed corrupted files with PROTECTED_ placeholders
4. ✅ Fixed missing struct fields (Element.attributes)
5. ✅ Fixed CLI parsing and type issues
6. ✅ Project now builds successfully

### Identified Issues

#### 1. Code Quality Issues
- **Unused imports and variables**: Multiple warnings about unused code
- **Dead code**: Functions like `escape_attribute_to` are never used
- **Incomplete implementations**: Some functions have TODO comments indicating missing functionality

#### 2. Structural Issues
- **Duplicate PluginConfig types**: There are two different PluginConfig types (enum vs struct) causing confusion
- **Missing Plugin cloning**: The `get_plugins_by_category` function can't properly clone plugins
- **Incomplete streaming parser**: Some streaming functionality appears unfinished

#### 3. Missing Features
- **Parallel processing**: Feature flag exists but implementation may be incomplete
- **Plugin factory pattern**: Needed for proper plugin instantiation
- **Error handling**: Some error paths use generic string errors instead of typed errors

## Improvement Plan

### Phase 0: Naming Unification (Immediate)

This phase focuses on standardizing the naming conventions across the codebase, documentation, and CLI to ensure consistency and clarity.

1.  **Standardize `vexy_svgo` (snake_case) usage:**
    *   **Scope:** Rust crate names, module paths, internal code identifiers (variables, functions), WASM file names, JavaScript module imports, configuration file names, database names, storage keys.
    *   **Action:** Verify all existing uses adhere to `vexy_svgo`. No changes are anticipated here as current usage seems consistent.

2.  **Standardize `Vexy SVGO` (Title Case, space separated) usage:**
    *   **Scope:** Human-readable project name in documentation titles, UI text, general prose, team names.
    *   **Action:**
        *   Change `VEXYSVGO` to `Vexy SVGO` in `test/svgo_compatibility_tests.rs` comment.
        *   Change `Building VEXYSVGO...` to `Building Vexy SVGO...` in `test/comparative/test_plugins.sh`.
        *   Change `VEXYSVGO` to `Vexy SVGO` in `crates/ffi/src/lib.rs` comment (related to FFI function descriptions).
        *   Change `VEXYSVGO Team` to `Vexy SVGO Team` in `scripts/marketplace-setup.sh`.
        *   Change `VEXYSVGO WebAssembly module` to `Vexy SVGO WebAssembly module` in `crates/wasm/vexy_svgo.d.ts`.

3.  **Standardize `vexy-svgo` (kebab-case) usage for CLI and external references:**
    *   **Scope:** CLI command name, package manager names (Homebrew, Chocolatey), repository names, URLs, binary names.
    *   **Action:** This is the most significant change and requires careful execution.
        *   **Rename CLI executable:** Change the `vexy_svgo` binary name to `vexy-svgo`. This will involve updating `Cargo.toml` for the `cli` crate and build scripts.
        *   **Update CLI command examples:** Change all instances of `vexy_svgo` to `vexy-svgo` in `README.md`, `examples/cli-usage.md`, and `docs/plugin-marketplace.md` (CLI commands).
        *   **Update binary names in build scripts:** Change `vexy_svgo-linux`, `vexy_svgo-macos-universal`, `vexy_svgo-windows` to `vexy-svgo-linux`, `vexy_svgo-macos-universal`, `vexy-svgo-windows` in `scripts/build.sh`.
        *   **Update repository URLs:** Change `https://github.com/twardoch/vexy_svgo` to `https://github.com/twardoch/vexy-svgo` in `Cargo.toml`, `README.md`, `examples/wasm-enhanced-demo.html`, `docs/wasm-demo.html`, `docs/plugin-development.md`, `release.sh`, `issues/301.txt`. (Note: This might require an actual GitHub repository rename, which is outside the scope of direct file modification but should be noted).
        *   **Update package manager instructions:** Ensure `brew install vexy-svgo` and `choco install vexy-svgo` are used in `README.md`. (Currently `vexy_svgo` is used, which is inconsistent with kebab-case for package managers).
        *   **Update project root check in `release.sh`:** Change `vexy_svgo` to `vexy-svgo`.
        *   **Update `docs/plugin-development.md`:** Change `vexy_svgo` in clone/build/mkdir commands to `vexy-svgo`.

4.  **Remove `VEXYSVGO` (all caps) for general use:**
    *   **Scope:** Any instance where it's not a specific FFI function prefix or a constant.
    *   **Action:** All instances identified in step 2.2 will be changed to `Vexy SVGO` or `vexy_svgo` as appropriate.

### Phase 1: Code Cleanup (Immediate)

1. **Remove unused code**
   - Clean up all unused imports
   - Remove or implement unused functions
   - Fix all compiler warnings
   - Fix unused variable warnings (e.g., `file_path` is never read)

2. **Fix structural issues**
   - Consolidate PluginConfig types into a single, well-designed type
   - Implement proper plugin cloning or factory pattern
   - Complete the streaming parser implementation

3. **Improve error handling**
   - Create typed error enums for different error categories
   - Replace string errors with proper error types
   - Add context to errors for better debugging
   - Implement `std::error::Error` trait for all error types

### Phase 2: Feature Completion (Short-term)

1. **Complete parallel processing**
   - Verify parallel feature implementation
   - Fix Rayon imports and usage
   - Add tests for parallel execution
   - Document thread pool configuration
   - Document performance benefits

2. **Plugin system improvements**
   - Implement plugin factory pattern
   - Add plugin validation
   - Create plugin testing framework
   - Document plugin API

3. **CLI enhancements**
   - Add progress indicators for folder processing
   - Implement proper color output support
   - Add verbose logging options
   - Add `--dry-run` option

### Phase 3: Testing & Documentation (Medium-term)

1. **Comprehensive testing**
   - Unit tests for all core functionality (AST, parser, optimizer modules)
   - Integration tests for CLI
   - Performance benchmarks
   - Compatibility tests with SVGO configs

2. **Documentation**
   - API documentation for all public types (generate with rustdoc)
   - Plugin development guide
   - Migration guide from SVGO
   - Performance tuning guide
   - Add inline documentation for all public APIs

3. **Examples**
   - CLI usage examples
   - Plugin development examples
   - Integration examples (Node.js, Python, etc.)
   - Create WebAssembly usage examples

### Phase 4: Performance Optimization (Long-term)

1. **Memory optimization**
   - Profile memory usage for large SVG files
   - Implement memory-efficient parsing strategies (including streaming for very large files)
   - Add memory usage limits and controls
   - Optimize AST memory layout

2. **Speed optimization**
   - Benchmark against SVGO (create comprehensive benchmarks)
   - Optimize hot paths (profile and optimize)
   - Implement SIMD optimizations where applicable (for path data)
   - Add parallel path processing

3. **Streaming improvements**
   - Complete streaming parser implementation
   - Add streaming output support
   - Implement incremental optimization
   - Add chunked processing for large files

## Technical Debt Items

1. **PROTECTED_ placeholder cleanup**
   - While fixed, the root cause of these placeholders should be investigated
   - Ensure no build scripts are introducing these corruptions
   - Add build verification steps
   - Create reproducible builds

2. **Import/Export organization**
   - Review and reorganize public API exports
   - Ensure consistent naming conventions
   - Clean up module structure
   - Remove duplicate code

3. **Configuration system**
   - Validate configuration loading and merging
   - Add configuration schema validation
   - Support for .svgo.config.js compatibility
   - Add configuration migration tool

## Quality Assurance

1. **Continuous Integration**
   - Set up GitHub Actions for automated testing
   - Add coverage reporting (with codecov/coveralls)
   - Implement automated benchmarking
   - Add cross-platform testing

2. **Code quality tools**
   - Configure clippy with strict lints
   - Add rustfmt configuration (with project style)
   - Set up pre-commit hooks
   - Add commit message linting

3. **Release process**
   - Automated version bumping
   - Changelog generation
   - Binary distribution for multiple platforms
   - Set up crates.io publishing
   - Create Homebrew formula
   - Create npm package wrapper

## Nice-to-Have Features

1. **SVG validation**
   - Add SVG validation before optimization
2. **SVG diff tool**
   - Implement SVG diff tool
3. **Batch processing**
   - Add batch processing with glob patterns
4. **GUI wrapper**
   - Create GUI wrapper
5. **Plugin marketplace integration**
   - Add plugin marketplace integration
6. **Custom plugin loader**
   - Implement custom plugin loader
7. **Telemetry/analytics**
   - Add telemetry/analytics (opt-in)
8. **VS Code extension**
   - Create VS Code extension
9. **Docker image**
   - Add Docker image
10. **Watch mode**
    - Implement watch mode for development

## Conclusion

The Vexy SVGO project has a solid foundation but needs systematic improvements to reach production quality. This plan prioritizes immediate fixes to get a working release, followed by feature completion and long-term optimizations. The focus should be on maintaining SVGO compatibility while leveraging Rust's performance advantages.

### Build Log Analysis and Suggestions

The build log indicates that the main project builds successfully for macOS, but the WebAssembly (WASM) build fails due to issues with the `getrandom` crate.

**Problem:** The `getrandom` crate, a dependency, is not correctly configured for the `wasm32-unknown-unknown` target. It requires the `wasm_js` feature to be enabled for WASM builds.

**Suggestion:**
- **Modify `crates/wasm/Cargo.toml`:** Add `features = ["js"]` to the `getrandom` dependency under the `[dependencies]` section, specifically for the `wasm32-unknown-unknown` target. This will enable the necessary WASM-specific features for `getrandom`.
