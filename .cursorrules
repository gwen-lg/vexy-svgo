
After every iteration, /report and mark completed items as done in @PLAN.md and @TODO.md. Then run `./build.sh` and then check the `./build_logs`. If needed read the @llms.txt code snapshot. Then /work on items from @TODO.md consulting on @PLAN.md. Then review reflect refine revise, and then continue to /work on @PLAN.md and @TODO.md until every single item and issue has been fixed. Iterate iterate iterate! Do not stop, do not ask for confirmation. Work! When you're finishing one task or item, say "Wait, but..." and go on to the next task/item. It’s CRUCIAL that we get to a solution that BUILDS everything correctly!

# Claude Development Guide for vexy_svgo

This document provides the necessary context and guidelines for developing `vexy_svgo`, a high-performance SVG optimizer written in Rust. It is an API-compatible port of the popular Node.js-based tool, `svgo`.

## 1. Project Overview

`vexy_svgo` is an advanced, native Rust port of `svgo`. It is designed for speed, efficiency, and seamless integration into various workflows.

**Key Features:**
*   **High Performance:** Significantly faster than the original `svgo`.
*   **API-Compatible:** Aims to be a drop-in replacement for `svgo`, supporting its plugin-based architecture and configuration.
*   **Cross-Platform:** Provides a native CLI for macOS, Windows, and Linux.
*   **WebAssembly (WASM) Support:** Can be compiled to WASM for use in browsers and Node.js environments, making it a versatile tool for web development.
*   **Extensive Plugin System:** Implements a wide range of optimization plugins ported from `svgo`.

The project is well-established, with a mature codebase, comprehensive test suite, and automated CI/CD pipelines.

## 2. Development Workflow

A consistent development workflow is crucial. All code should be built, tested, and linted before submission.

### 2.1. Local Setup
1.  **Install Rust:** Use `rustup` to install the latest stable Rust toolchain.
2.  **Install Dependencies:** No special dependencies are needed beyond the Rust toolchain. `cargo` will handle everything.
3.  **Build & Test:** Use the main build script to ensure everything is correct. This script runs builds, tests, lints, and format checks.

    ```bash
    ./build.sh
    ```
    Review the output in `build.log.txt`.

### 2.2. Core Commands
*   **Build:** `cargo build --release`
*   **Test:** `cargo test`
*   **Lint:** `cargo clippy -- -D warnings`
*   **Format:** `cargo fmt --check`

### 2.3. WebAssembly (WASM)
The WASM module is built using `wasm-pack`. The `build-wasm.sh` script handles the creation of different packages for web, Node.js, and bundlers.

```bash
./build-wasm.sh
```

### 2.4. Documentation
The project documentation is in the `docs/` directory and is built using Jekyll with the "Just the Docs" theme.

```bash
cd docs
./_build.sh
```

## 3. Project Structure

The repository is organized as a Cargo workspace with several crates:

*   `crates/`: Home for all the Rust crates.
    *   `cli/`: The command-line interface for `vexy_svgo`. This crate handles argument parsing, configuration loading, and file I/O.
    *   `core/`: The heart of `vexy_svgo`. It contains the SVG parser, the optimizer, the plugin driver, and the stringifier. It does not know about specific plugins.
    *   `ffi/`: Provides Foreign Function Interface (FFI) bindings for `vexy_svgo`, allowing it to be called from other languages like C, Python, or Node.js.
    *   `plugin-sdk/`: Provides the traits and types for creating plugins. It also includes a registry for discovering and managing plugins.
    *   `test-utils/`: Contains helper functions and macros for testing plugins and the core engine.
    *   `wasm/`: The WebAssembly bindings for `vexy_svgo`, allowing it to run in the browser and Node.js.
*   `vexy_svgo/`: The main Rust crate that integrates all the other crates. Currently serves primarily as a re-export crate.
    *   `tests/`: Contains integration and compatibility tests, including those that compare `vexy_svgo`'s output with `svgo`'s.
*   `ref/svgo/`: A git submodule pointing to the original `svgo` repository. This is used for reference and for running compatibility tests.
*   `docs/`: The documentation website, built with Jekyll.
*   `scripts/`: A collection of shell scripts for building, testing, and benchmarking the project.
*   `.github/workflows/`: GitHub Actions workflows for continuous integration, release management, and other automated tasks.
*   `testdata/`: A collection of SVG files used for testing and benchmarking.
*   `Cargo.toml`: The root `Cargo.toml` file that defines the workspace and its members.

## 4. Architecture

`vexy_svgo`'s architecture is modular and inspired by `svgo` to ensure API and functional compatibility. The core logic is separated into several crates:

1.  **`crates/core`:** This crate is the heart of the optimizer. It is responsible for:
    *   **Parsing (`parser.rs`):** An XML parser (`roxmltree`) reads the input SVG string into a custom Abstract Syntax Tree (AST) defined in `ast.rs`. It includes advanced features like XML entity expansion and selective whitespace preservation.
    *   **Optimization (`optimizer.rs`):** A plugin-based pipeline iterates through the AST. It applies enabled plugins in a specific order. It supports multi-threading using `rayon` for large files.
    *   **Stringification (`stringifier.rs`):** After the AST has been processed by all plugins, the stringifier converts the modified AST back into an optimized SVG string. This module is highly optimized for performance and memory efficiency.
    *   **Configuration (`config.rs`):** Manages the plugin configurations and optimization settings, mirroring `svgo`'s config format.
    *   **Visitor (`visitor.rs`):** Implements a visitor pattern that allows plugins to traverse and modify the AST.

2.  **`crates/plugin-sdk`:** This crate defines the interface for creating plugins. It provides:
    *   The `Plugin` trait that all plugins must implement.
    *   A plugin registry for discovering and managing available plugins.
    *   The implementations of all the optimization plugins in `src/plugins/`. This includes advanced geometric features using the `lyon` crate for path data optimization.

3.  **`vexy_svgo` crate:** This is the main crate that brings everything together. It primarily serves as a re-export crate that provides the public API.

4.  **`crates/cli`:** This crate provides the command-line interface for `vexy_svgo`. It is responsible for parsing command-line arguments, loading configuration files, and running the optimizer on the specified files.

5.  **`crates/wasm`:** This crate is a thin wrapper around the `vexy_svgo` crate that exposes a WebAssembly-compatible API.

6.  **`crates/ffi`:** This crate provides Foreign Function Interface (FFI) bindings for `vexy_svgo`, allowing it to be called from other languages like C, Python, or Node.js.

## 5. Testing Strategy

The project relies on a robust testing strategy to ensure correctness and compatibility.

*   **Unit & Integration Tests:** Located within the `vexy_svgo` crate in `vexy_svgo/tests`, these tests cover individual functions and modules. Run with `cargo test`.
*   **SVGO Compatibility Tests:** The `vexy_svgo/tests/svgo_compatibility_tests.rs` file runs a large suite of tests ported from `svgo`. The script `generate_compatibility_tests.py` helps automate the creation of these test cases from the `ref/svgo` submodule. This is critical for maintaining parity.
*   **Plugin Tests:** Each plugin has its own set of tests in `test/plugins/`. These tests are automatically generated from the `svgo` test fixtures. Additionally, property-based tests are used to ensure robustness and correctness across a wide range of inputs.
*   **CI Automation:** GitHub Actions automatically run all tests, lints, and format checks on every push and pull request across macOS, Windows, and Linux.

## 6. Reference: SVGO Specification

Since `vexy_svgo` is an API-compatible port, the `svgo` specification is the primary reference for functionality and the plugin system. The goal is to match the behavior of `svgo` as closely as possible.

### 6.1. Core API: `optimize(input, config)`
The main function takes an SVG string and an optional configuration object. The configuration allows enabling/disabling plugins and setting their parameters.

### 6.2. Plugins
Plugins are the core of the optimization process. `vexy_svgo` aims to implement all major `svgo` plugins. When working on a plugin, refer to its counterpart in `ref/svgo/plugins/` for the exact logic and behavior. The default plugin preset from `svgo` is the baseline for default optimizations.

*(A full list of SVGO plugins can be found in the original `svgo` documentation or the `GEMINI.md` file for this project. This list serves as the implementation roadmap for `vexy_svgo`.)*

## 7. AI Assistant Guidelines

### 7.1. General Principles
*   **Iterate Gradually:** Make small, incremental changes.
*   **Preserve Structure:** Do not change existing code structure unless necessary.
*   **Follow Conventions:** Adhere to existing Rust and project-specific coding conventions.
*   **Test Thoroughly:** Always run `./build.sh` after making changes to ensure all checks pass.
*   **Write Clear Code:** Use descriptive names and add comments to explain the *why* behind complex logic.
*   **Update Documentation:** If you change functionality, update the corresponding documentation in the `docs/` directory.

### 7.2. Command Workflow


**Note on build commands:** Do not run `cargo` commands directly. Always use the `./build.sh` script, as it ensures a consistent set of checks is performed.

---

# Consolidated Software Development Rules

## 8. Pre-Work Preparation

### 8.1. Before Starting Any Work
- **ALWAYS** read `WORK.md` in the main project folder for work progress
- Read `README.md` to understand the project
- STEP BACK and THINK HEAVILY STEP BY STEP about the task
- Consider alternatives and carefully choose the best option
- Check for existing solutions in the codebase before starting

### 8.2. Project Documentation to Maintain
- `README.md` - purpose and functionality
- `CHANGELOG.md` - past change release notes (accumulative)
- `PLAN.md` - detailed future goals, clear plan that discusses specifics
- `TODO.md` - flat simplified itemized `- [ ]`-prefixed representation of `PLAN.md`
- `WORK.md` - work progress updates

## 9. General Coding Principles

### 9.1. Core Development Approach
- Iterate gradually, avoiding major changes
- Focus on minimal viable increments and ship early
- Minimize confirmations and checks
- Preserve existing code/structure unless necessary
- Check often the coherence of the code you're writing with the rest of the code
- Analyze code line-by-line

### 9.2. Code Quality Standards
- Use constants over magic numbers
- Write explanatory docstrings/comments that explain what and WHY
- Explain where and how the code is used/referred to elsewhere
- Handle failures gracefully with retries, fallbacks, user guidance
- Address edge cases, validate assumptions, catch errors early
- Let the computer do the work, minimize user decisions
- Reduce cognitive load, beautify code
- Modularize repeated logic into concise, single-purpose functions
- Favor flat over nested structures

## 10. Tool Usage (When Available)

### 10.1. MCP Tools to Consult
- `codex` tool - for additional reasoning, summarization of files and second opinion
- `context7` tool - for most up-to-date software package documentation
- `sequentialthinking` tool - to think about the best way to solve tasks
- `perplexity_ask` - for up-to-date information or context

### 10.2. Additional Tools
- Use `tree` CLI app if available to verify file locations
- Check existing code with `.venv` folder to scan and consult dependency source code
- Run `DIR="."; uvx codetoprompt --compress --output "$DIR/llms.txt"  --respect-gitignore --cxml --exclude "*.svg,.specstory,*.md,*.txt,ref,testdata,*.lock,*.svg" "$DIR"` to get a condensed snapshot of the codebase into `llms.txt`

## 11. File Management

### 11.1. File Path Tracking
- **MANDATORY**: In every source file, maintain a `this_file` record showing the path relative to project root
- Place `this_file` record near the top:
  - As a comment after shebangs in code files
  - In YAML frontmatter for Markdown files
- Update paths when moving files
- Omit leading `./`
- Check `this_file` to confirm you're editing the right file

## 12. Python-Specific Guidelines

### 12.1. PEP Standards
- PEP 8: Use consistent formatting and naming, clear descriptive names
- PEP 20: Keep code simple and explicit, prioritize readability over cleverness
- PEP 257: Write clear, imperative docstrings
- Use type hints in their simplest form (list, dict, | for unions)

### 12.2. Modern Python Practices
- Use f-strings and structural pattern matching where appropriate
- Write modern code with `pathlib`
- ALWAYS add "verbose" mode loguru-based logging & debug-log
- Use `uv pip install` instead of `pip install`
- Prefix Python CLI tools with `python -m` (e.g., `python -m pytest`)

### 12.3. CLI Scripts Setup
For CLI Python scripts, use `fire` & `rich`, and start with:
```python
#!/usr/bin/env -S uv run -s
# /// script
# dependencies = ["PKG1", "PKG2"]
# ///
# this_file: PATH_TO_CURRENT_FILE
```

### 12.4. Post-Edit Python Commands
```bash
fd -e py -x uvx autoflake -i {}; fd -e py -x uvx pyupgrade --py312-plus {}; fd -e py -x uvx ruff check --output-format=github --fix --unsafe-fixes {}; fd -e py -x uvx ruff format --respect-gitignore --target-version py312 {}; python -m pytest;
```

## 13. Post-Work Activities

### 13.1. Critical Reflection
- After completing a step, say "Wait, but" and do additional careful critical reasoning
- Go back, think & reflect, revise & improve what you've done
- Don't invent functionality freely
- Stick to the goal of "minimal viable next version"

### 13.2. Documentation Updates
- Update `WORK.md` with what you've done and what needs to be done next
- Document all changes in `CHANGELOG.md`
- Update `TODO.md` and `PLAN.md` accordingly

## 14. Work Methodology

### 14.1. Virtual Team Approach
Be creative, diligent, critical, relentless & funny! Lead two experts:
- **"Ideot"** - for creative, unorthodox ideas
- **"Critin"** - to critique flawed thinking and moderate for balanced discussions

Collaborate step-by-step, sharing thoughts and adapting. If errors are found, step back and focus on accuracy and progress.

### 14.2. Continuous Work Mode
- Treat all items in `PLAN.md` and `TODO.md` as one huge TASK
- Work on implementing the next item
- Review, reflect, refine, revise your implementation
- Periodically check off completed issues
- Continue to the next item without interruption

## 15. Special Commands

### 15.1. `/report` Command
1. Read all `./TODO.md` and `./PLAN.md` files
2. Analyze recent changes
3. Document all changes in `./CHANGELOG.md`
4. Remove completed items from `./TODO.md` and `./PLAN.md`
5. Ensure `./PLAN.md` contains detailed, clear plans with specifics
6. Ensure `./TODO.md` is a flat simplified itemized representation

### 15.2. `/work` Command
1. Read all `./TODO.md` and `./PLAN.md` files and reflect
2. Work on the tasks
3. Think, contemplate, research, reflect, refine, revise
4. Be careful, curious, vigilant, energetic
5. Verify your changes and think aloud
6. Consult, research, reflect
7. Update `./PLAN.md` and `./TODO.md` with improvement tasks
8. Execute `/report`
9. Iterate again

## 16. Additional Guidelines

- Ask before extending/refactoring existing code that may add complexity or break things
- Work tirelessly without constant updates when in continuous work mode
- Only notify when you've completed all `PLAN.md` and `TODO.md` items

## 17. Custom commands: 

When I say "/report", you must: Read all `./TODO.md` and `./PLAN.md` files and analyze recent changes. Document all changes in `./CHANGELOG.md`. From `./TODO.md` and `./PLAN.md` remove things that are done. Make sure that `./PLAN.md` contains a detailed, clear plan that discusses specifics, while `./TODO.md` is its flat simplified itemized `- [ ]`-prefixed representation. You may also say "/report" to yourself and that will prompt you to perform the above-described task autonomously. 

When I say "/work", you must work in iterations like so: Read all `./TODO.md` and `./PLAN.md` files and reflect. Write down the immediate items in this iteration into `./WORK.md` and work on these items. Think, contemplate, research, reflect, refine, revise. Be careful, curious, vigilant, energetic. Verify your changes. Think aloud. Consult, research, reflect. Periodically remove completed items from `./WORK.md` and tick off completed items from `./TODO.md` and `./PLAN.md`. Update `./WORK.md` with items that will lead to improving the work you've just done, and /work on these. When you're happy with your implementation of the most recent item, '/report', and consult `./PLAN.md` and `./TODO.md`, and /work on implementing the next item, and so on and so on. Work tirelessly without informing me. Only let me know when you've completed the task of implementing all `./PLAN.md` and `./TODO.md` items. You may also say "/report" to yourself and that will prompt you to perform the above-described task autonomously. 