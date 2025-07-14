# Vexy SVGO Improvement Plan

This document outlines a series of proposed improvements for the Vexy SVGO codebase. These changes focus on enhancing robustness, efficiency, and maintainability through targeted, quality-of-life upgrades rather than new features.

## 1. Enhanced Build & CI Pipeline

### 1.1. Introduce Build Verification & Reproducibility
**Goal:** Ensure that builds are consistent and verifiable, reducing the risk of environment-specific errors.

**Detailed Steps:**
1.  **Add a build verification script:** Create a new script `scripts/verify-build.sh` that performs a clean build and then runs tests against the release artifacts. This script will be crucial for ensuring that what gets published is what was tested.
    ```bash
    #!/bin/bash
    set -euo pipefail
    
    echo "--- Verifying clean release build ---"
    cargo clean
    cargo build --release
    
    echo "--- Running tests against release build ---"
    cargo test --release
    
    echo "--- Build verification successful ---"
    ```
2.  **Integrate into CI:** Update the `.github/workflows/ci.yml` to include a new job that runs this verification script. This ensures that every pull request and push to `main` is verified.
    ```yaml
    jobs:
      verify-build:
        name: Verify Release Build
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
          - uses: actions/cache@v4
            with:
              path: |
                ~/.cargo/bin/
                ~/.cargo/registry/index/
                ~/.cargo/registry/cache/
                ~/.cargo/git/db/
                target/
              key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          - name: Run build verification script
            run: ./scripts/verify-build.sh
    ```

### 1.2. Refine Pre-Commit Hooks
**Goal:** Catch more issues locally before they ever reach CI, speeding up the development cycle.

**Detailed Steps:**
1.  **Add `cargo-deny` to pre-commit:** Integrate `cargo-deny` to check for duplicate dependencies, security advisories, and license compatibility on every commit.
    ```yaml
    -   repo: https://github.com/EmbarkStudios/cargo-deny
        rev: 0.14.0
        hooks:
          - id: cargo-deny
            args: [ "check" ]
    ```
2.  **Add `codespell` for typo checking:** Integrate `codespell` to automatically fix common misspellings in the codebase.
    ```yaml
    -   repo: https://github.com/codespell-project/codespell
        rev: v2.2.6
        hooks:
          - id: codespell
            args: ["-w"]
    ```

## 2. Codebase & Module Organization

### 2.1. Standardize Module Exports
**Goal:** Create a more consistent and predictable public API by standardizing how modules export their functionality.

**Detailed Steps:**
1.  **Adopt `pub use` for re-exports:** In `crates/vexy_svgo/src/lib.rs`, refactor all public-facing modules to use `pub use` for re-exporting types and functions. This will create a flatter, more accessible API for consumers of the crate.
    
    *Example (before):*
    ```rust
    // In crates/vexy_svgo/src/lib.rs
    pub mod config;
    pub mod error;
    pub mod optimizer;
    ```
    
    *Example (after):*
    ```rust
    // In crates/vexy_svgo/src/lib.rs
    pub use crate::config::{Config, OptimizerConfig};
    pub use crate::error::VexyError;
    pub use crate::optimizer::optimize;
    ```
2.  **Audit public APIs:** Systematically review every `pub` item in the library crates (`core`, `plugin-sdk`, `vexy_svgo`) to ensure that only intentionally public items are exposed. Add `#[doc(hidden)]` to items that are public for technical reasons but not part of the stable API.

### 2.2. Centralize Error Handling
**Goal:** Improve error handling by creating a single, unified error type for the entire application.

**Detailed Steps:**
1.  **Create a top-level `Error` enum:** In `crates/vexy_svgo/src/error.rs`, define a comprehensive `Error` enum that encapsulates all possible failure modes, from I/O errors to parsing and optimization failures.
    ```rust
    use thiserror::Error;
    
    #[derive(Debug, Error)]
    pub enum VexyError {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),
    
        #[error("XML parsing error: {0}")]
        Parsing(#[from] roxmltree::Error),
    
        #[error("Plugin '{0}' failed: {1}")]
        Plugin(String, String),
    
        #[error("Invalid configuration: {0}")]
        Config(String),
    }
    ```
2.  **Refactor crates to use the unified error type:** Replace all instances of custom error types in other crates with `VexyError`. Use `Result<T, VexyError>` as the standard return type for fallible operations.

## 3. Configuration Management

### 3.1. Implement Configuration Schema Validation
**Goal:** Provide users with clear, immediate feedback on invalid configuration files.

**Detailed Steps:**
1.  **Introduce JSON Schema:** Create a `svgo.schema.json` file in the root of the project. This file will define the structure, types, and allowed values for `svgo.config.js`.
2.  **Add validation logic:** In `crates/cli/src/config.rs`, use a library like `jsonschema` to validate loaded configurations against the schema. If validation fails, print a user-friendly error message that explains what is wrong and points to the documentation.
    ```rust
    // Example validation logic
    let schema = load_schema("svgo.schema.json")?;
    let instance = serde_json::to_value(config)?;
    let result = jsonschema::validate(&instance, &schema);
    
    if let Err(errors) = result {
        for error in errors {
            eprintln!("Configuration error: {}", error);
        }
        return Err(VexyError::Config("Invalid configuration".to_string()));
    }
    ```

## 4. Documentation & Examples

### 4.1. Create a Plugin Development Example
**Goal:** Lower the barrier to entry for new plugin developers by providing a clear, working example.

**Detailed Steps:**
1.  **Create a new example crate:** Add a new crate under `examples/example-plugin`. This crate will demonstrate how to create a simple plugin that removes all `<g>` elements from an SVG.
2.  **Write a tutorial:** Add a new page to the documentation (`docs_src/developer/creating-a-plugin.md`) that walks through the process of creating, testing, and using the example plugin.

### 4.2. Add Integration Examples
**Goal:** Show users how to integrate `vexy-svgo` into their projects.

**Detailed Steps:**
1.  **Node.js example:** Create a simple Node.js project in `examples/nodejs-integration` that uses the WASM build of `vexy-svgo` to optimize an SVG file.
2.  **Python example:** Create a Python script in `examples/python-integration` that uses the FFI bindings to call `vexy-svgo` from Python.