# Contributing to Vexy SVGO

First off, thank you for considering contributing to Vexy SVGO! It's people like you that make Vexy SVGO such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by the [Vexy SVGO Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [adam+github@twardoch.com](mailto:adam+github@twardoch.com).

## How Can I Contribute?

### **Reporting Bugs**

- **Ensure the bug was not already reported** by searching on GitHub under [Issues](https://github.com/vexyart/vexy-svgo/issues).
- If you're unable to find an open issue addressing the problem, [open a new one](https://github.com/vexyart/vexy-svgo/issues/new). Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample** or an **executable test case** demonstrating the expected behavior that is not occurring.

### **Suggesting Enhancements**

- Open a new issue with the title `[Enhancement] ...` and provide a clear and detailed explanation of the feature you would like to see.

### **Pull Requests**

- Fork the repository and create your branch from `main`.
- Open a new pull request with the patch.
- Ensure the PR description clearly describes the problem and solution. Include the relevant issue number if applicable.

## Development Setup

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/vexy-svgo.git
   cd vexy-svgo
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/vexyart/vexy-svgo
   ```

## Project Structure

```
vexy-svgo/
├── crates/
│   ├── core/          # Core SVG processing engine
│   ├── cli/           # Command-line interface
│   ├── plugin-sdk/    # Plugin development kit
│   ├── wasm/          # WebAssembly bindings
│   └── ffi/           # C-compatible FFI
├── docs/              # Documentation website
├── examples/          # Usage examples
├── test/              # Integration and compatibility tests
└── ...
```

## Coding Conventions

- Follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `cargo fmt` to format your code.
- Use `cargo clippy` to lint your code.
- Add unit tests for new functionality.

## Submitting a Pull Request

- Rebase your branch on the latest `main` from the upstream repository.
- Ensure all tests pass (`cargo test --workspace`).
- Make sure your code lints (`cargo clippy --workspace -- -D warnings`).
- Submit your pull request!

