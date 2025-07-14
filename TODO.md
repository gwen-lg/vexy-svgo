# Vexy SVGO TODO List

This TODO list is a linearized version of the improvement plan in `PLAN.md`.

## 1. Enhanced Build & CI Pipeline

- [x] Create `scripts/verify-build.sh` for build verification.
- [x] Integrate `scripts/verify-build.sh` into `.github/workflows/ci.yml`.
- [x] Add `cargo-deny` to `.pre-commit-config.yaml`.
- [x] Add `codespell` to `.pre-commit-config.yaml`.

## 2. Codebase & Module Organization

- [ ] Refactor `crates/vexy_svgo/src/lib.rs` to use `pub use` for re-exports.
  - [x] Refactored `crates/core/src/lib.rs`.
- [x] Audit all public APIs in `core`, `plugin-sdk`, and `vexy_svgo` crates.
- [x] Create a unified `VexyError` enum in `crates/vexy_svgo/src/error.rs`.
- [x] Refactor all crates to use the new unified `VexyError` type.
  - [x] Refactored `crates/cli/src/main.rs`.
  - [x] Refactored `crates/cli/src/features_cmd.rs`.
  - [x] Refactored `crates/core/src/optimizer/mod.rs`.
  - [x] Refactored `crates/core/src/parser/main.rs`.
  - [x] Refactored `crates/core/src/parser/streaming.rs`.
  - [x] Refactored `crates/core/src/stringifier.rs`.
  - [x] Refactored `crates/core/src/visitor.rs`.
  - [x] Refactored `crates/plugin-sdk/src/lib.rs`.
  - [x] Refactored `crates/plugin-sdk/src/enhanced_registry.rs`.
  - [x] Refactored `crates/ffi/src/lib.rs`.
  - [x] Refactored `crates/wasm/src/lib.rs`.
  - [x] Refactored `crates/wasm/src/enhanced.rs`.
  - [x] Refactored `crates/wasm/src/minimal.rs`.

## 3. Configuration Management

- [x] Create `svgo.schema.json` in the project root.
- [x] Implement configuration validation using the JSON schema in `crates/cli/src/config.rs`.

## 4. Documentation & Examples

- [x] Create a new example plugin crate in `examples/example-plugin`.
- [x] Write a plugin development tutorial in `docs_src/developer/creating-a-plugin.md`.
- [x] Create a Node.js integration example in `examples/nodejs-integration`.
- [x] Create a Python integration example in `examples/python-integration`.