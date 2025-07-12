# Vexy SVGO TODO List - Linearized Action Items

## 1. Immediate Build Fixes

### 1.1. Fix Integration Test Binary Name Issue

- [x] Fix integration test in `crates/cli/tests/integration_test.rs` to use correct binary name 'vexy-svgo' instead of 'vexy_svgo'
- [x] Update log message in `release.sh` from "Starting VEXYSVGO release process" to "Starting Vexy SVGO release process"
- [x] Verify the CLI binary name in `crates/cli/Cargo.toml` is set to 'vexy-svgo'

## 2. Naming Unification (Immediate Priority)

This phase focuses on standardizing the naming conventions across the codebase, documentation, and CLI to ensure consistency and clarity.

1.  [x] **Standardize `vexy_svgo` (snake_case) usage:**

    - [x] Verify all existing uses adhere to `vexy_svgo` in Rust crate names, module paths, internal code identifiers (variables, functions), WASM file names, JavaScript module imports, configuration file names, database names, storage keys. (Keep `vexy_svgo` for internal Rust identifiers, crate names, module paths, WASM file names, and configuration files where snake_case is idiomatic.)

2.  [ ] **Standardize `Vexy SVGO` (Title Case, space separated) usage:**

    - [x] Change `VEXYSVGO` to `Vexy SVGO` in `test/svgo_compatibility_tests.rs` comment. (Change to `Vexy SVGO` for human-readable comments and documentation.)
    - [x] Change `Building VEXYSVGO...` to `Building Vexy SVGO...` in `test/comparative/test_plugins.sh`. (Change to `Vexy SVGO` for human-readable output.)
    - [x] Change `VEXYSVGO` to `Vexy SVGO` in `crates/ffi/src/lib.rs` comment (related to FFI function descriptions). (Change to `Vexy SVGO` for human-readable comments.)
    - [x] Change `VEXYSVGO Team` to `Vexy SVGO Team` in `CHANGELOG.md` and `scripts/marketplace-setup.sh`. (Change to `Vexy SVGO` for human-readable team names in documentation and scripts.)
        - [x] Change `VEXYSVGO WebAssembly module` to `Vexy SVGO WebAssembly module` in `crates/wasm/vexy_svgo.d.ts`. (Change to `Vexy SVGO` for human-readable descriptions in type definitions.)

3.  [ ] **Standardize `vexy-svgo` (kebab-case) usage for CLI and external references:**

    - [x] Rename CLI executable: Change the `vexy_svgo` binary name to `vexy-svgo`. This will involve updating `Cargo.toml` for the `cli` crate and build scripts. (Change to `vexy-svgo` for the CLI executable name and related build script references.)
      - [x] Update CLI command examples: Change all instances of `vexy_svgo` to `vexy-svgo` in `README.md`, `examples/cli-usage.md`, and `docs/plugin-marketplace.md` (CLI commands). (Change to `vexy-svgo` for all CLI command examples.)
    - [x] Update binary names in build scripts: Change `vexy_svgo-linux`, `vexy_svgo-macos-universal`, `vexy_svgo-windows` to `vexy-svgo-linux`, `vexy-svgo-macos-universal`, `vexy-svgo-windows` in `scripts/build.sh`. (Change to `vexy-svgo` for binary names in build scripts.)
    - [ ] Update repository URLs: Change `https://github.com/twardoch/vexy_svgo` to `https://github.com/twardoch/vexy-svgo` in `Cargo.toml`, `README.md`, `examples/wasm-enhanced-demo.html`, `docs/wasm-demo.html`, `docs/plugin-development.md`, `release.sh`, `issues/301.txt`. (Change to `vexy-svgo` for repository URLs.)
    - [ ] Update package manager instructions: Ensure `brew install vexy-svgo` and `choco install vexy-svgo` are used in `README.md`. (Change to `vexy-svgo` for package manager instructions.)
    - [ ] Update project root check in `release.sh`: Change `vexy_svgo` to `vexy-svgo`. (Change to `vexy-svgo` for project root checks in scripts.)
    - [ ] Update `docs/plugin-development.md`: Change `vexy_svgo` in clone/build/mkdir commands to `vexy-svgo`. (Change to `vexy-svgo` for commands in documentation.)

4.  [ ] **Remove `VEXYSVGO` (all caps) for general use:**
    - [ ] Change all instances identified in step 2.2 to `Vexy SVGO` or `vexy_svgo` as appropriate. (Remove `VEXYSVGO` and replace with `Vexy SVGO` for human-readable text or `vexy_svgo` for code identifiers, depending on context.)

## 3. 🔧 Phase 1: Code Cleanup (Immediate Priority)

### 3.1. Remove Unused Code

5. [ ] Fix unused variable warnings (e.g., `compile_time_features`, `format`, `registry`, `root_element`, `file_path`)
6. [ ] Remove dead code marked with `#[warn(dead_code)]`

### 3.2. Fix Structural Issues

7. [ ] Consolidate `PluginConfig` types (enum in `parser/config.rs` vs struct in `plugin_registry.rs`)
8. [ ] Implement proper plugin cloning or factory pattern for `get_plugins_by_category`
9. [ ] Complete TODO items in streaming parser implementation

### 3.3. Improve Error Handling

10. [ ] Create typed error enums for different error categories
11. [ ] Replace string errors with proper error types
12. [ ] Add proper error context throughout the codebase
13. [ ] Implement `std::error::Error` trait for all error types

## 4. 🚀 Phase 2: Feature Completion (Short-term)

### 4.1. Complete Parallel Processing

14. [ ] Verify parallel feature flag implementation
15. [ ] Fix Rayon imports and usage
16. [ ] Add tests for parallel execution
17. [ ] Document thread pool configuration
18. [ ] Document performance benefits of parallel processing

### 4.2. Plugin System Improvements

19. [ ] Implement plugin factory pattern
20. [ ] Add plugin validation before execution
21. [ ] Create plugin testing framework
22. [ ] Document plugin API

### 4.3. CLI Enhancements

23. [ ] Add progress bar for folder processing
24. [ ] Implement proper color output with `termcolor`/`colored`
25. [ ] Add `--verbose` flag with detailed logging
26. [ ] Add `--dry-run` option

## 5. 📚 Phase 3: Testing & Documentation (Medium-term)

### 5.1. Testing

27. [ ] Write unit tests for core AST functionality
28. [ ] Write unit tests for parser module
29. [ ] Write unit tests for optimizer module
30. [ ] Create integration tests for CLI
31. [ ] Add performance benchmarks
32. [ ] Create compatibility test suite with SVGO configs

### 5.2. Documentation

33. [ ] Generate API documentation with `rustdoc`
34. [ ] Write plugin development guide
35. [ ] Create migration guide from SVGO
36. [ ] Document performance tuning options
37. [ ] Add inline documentation for all public APIs

### 5.3. Examples

38. [ ] Create CLI usage examples
39. [ ] Write plugin development examples
40. [ ] Add Node.js integration example
41. [ ] Add Python integration example
42. [ ] Create WebAssembly usage examples

## 6. ⚡ Phase 4: Performance Optimization (Long-term)

### 6.1. Memory Optimization

43. [ ] Profile memory usage with large SVG files
44. [ ] Implement streaming for very large files
45. [ ] Add memory limit configuration
46. [ ] Optimize AST memory layout

### 6.2. Speed Optimization

47. [ ] Create comprehensive benchmarks vs SVGO
48. [ ] Profile and optimize hot paths
49. [ ] Implement SIMD optimizations for path data
50. [ ] Add parallel path processing

### 6.3. Streaming Improvements

51. [ ] Complete streaming parser implementation
52. [ ] Add streaming output support
53. [ ] Implement incremental optimization
54. [ ] Add chunked processing for large files

## 7. 🧹 Technical Debt

### 7.1. PROTECTED\_ placeholder cleanup

57. [ ] Add build verification steps
58. [ ] Create reproducible builds

### 7.2. Code Organization

59. [ ] Review and reorganize public API exports
60. [ ] Ensure consistent naming conventions
61. [ ] Clean up module structure
62. [ ] Remove duplicate code

### 7.3. Configuration System

63. [ ] Validate configuration loading and merging
64. [ ] Add JSON schema validation for configs
65. [ ] Implement `.svgo.config.js` compatibility layer
66. [ ] Add configuration migration tool

## 8. 🎯 Quality Assurance

### 8.1. CI/CD

67. [ ] Set up GitHub Actions workflow
68. [ ] Add coverage reporting with `codecov`/`coveralls`
69. [ ] Implement automated benchmarking
70. [ ] Add cross-platform testing

### 8.2. Code Quality

71. [ ] Configure `clippy` with strict lints
72. [ ] Add `rustfmt.toml` with project style
73. [ ] Set up pre-commit hooks
74. [ ] Add commit message linting

### 8.3. Release Process

75. [ ] Set up automated version bumping
76. [ ] Implement changelog generation
77. [ ] Create binary releases for all platforms
78. [ ] Set up `crates.io` publishing
79. [ ] Create Homebrew formula
80. [ ] Create npm package wrapper
