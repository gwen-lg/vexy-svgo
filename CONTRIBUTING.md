# Contributing to Vexy SVGO

Thank you for your interest in contributing to Vexy SVGO! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Contributions](#making-contributions)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please read and follow our Code of Conduct:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/vexy_svgo.git
   cd vexy_svgo
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/twardoch/vexy_svgo.git
   ```

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- For WASM development:
  - wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
  - binaryen (for wasm-opt): `brew install binaryen` (macOS) or `apt-get install binaryen` (Ubuntu)

### Building the Project

```bash
# Run the comprehensive build script
./build.sh

# Or build manually
cargo build --release

# Build WASM modules
./build-wasm-v2.sh
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench
```

## Project Structure

```
vexy_svgo/
├── crates/
│   ├── core/          # Core SVG processing engine
│   ├── cli/           # Command-line interface
│   ├── plugin-sdk/    # Plugin development kit
│   ├── wasm/          # WebAssembly bindings
│   └── test-utils/    # Shared testing utilities
├── docs/              # Documentation website
├── scripts/           # Build and utility scripts
├── testdata/          # Test SVG files
└── ref/svgo/          # SVGO reference (git submodule)
```

### Key Modules

- **Parser** (`crates/core/src/parser/`): XML/SVG parsing with streaming support
- **AST** (`crates/core/src/ast.rs`): Abstract Syntax Tree representation
- **Optimizer** (`crates/core/src/optimizer/`): Plugin orchestration and optimization
- **Plugins** (`crates/plugin-sdk/src/plugins/`): Individual optimization plugins
- **Stringifier** (`crates/core/src/stringifier.rs`): AST to SVG conversion

## Making Contributions

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `test/description` - Test additions/improvements

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
type(scope): description

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or corrections
- `chore`: Maintenance tasks

Examples:
```
feat(parser): add streaming parser support
fix(plugin): correct color conversion in convertColors
docs(readme): update installation instructions
```

### Pull Request Process

1. **Create a feature branch** from `main`
2. **Make your changes** following the coding standards
3. **Add/update tests** for your changes
4. **Update documentation** if needed
5. **Run the build script** to ensure everything passes:
   ```bash
   ./build.sh
   ```
6. **Commit your changes** with clear messages
7. **Push to your fork** and create a pull request
8. **Describe your changes** in the PR description
9. **Link any related issues**

### Code Style

- Follow Rust standard style guidelines
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Add comments for complex logic
- Use descriptive variable and function names

## Testing Guidelines

### Unit Tests

- Place unit tests in the same file as the code being tested
- Use the `#[cfg(test)]` module pattern
- Test edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

- Place integration tests in `tests/` directories
- Test complete workflows and plugin interactions
- Use the test utilities from `crates/test-utils/`

### Plugin Tests

- Each plugin should have comprehensive tests
- Test with various SVG inputs
- Verify optimization correctness
- Check for regressions against SVGO behavior

## Documentation

### Code Documentation

- Add doc comments to all public APIs
- Use `///` for function/struct documentation
- Use `//!` for module-level documentation
- Include examples in doc comments when helpful

```rust
/// Optimizes an SVG document with the given configuration.
///
/// # Arguments
/// * `input` - The SVG string to optimize
/// * `config` - The optimization configuration
///
/// # Returns
/// The optimized SVG string or an error
pub fn optimize(input: &str, config: Config) -> Result<String> {
    // Implementation
}
```

### User Documentation

- Update README.md for user-facing changes
- Update docs/ for detailed documentation
- Include migration guides for breaking changes

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking API changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)

### Release Steps

1. Update version in `Cargo.toml` files
2. Update CHANGELOG.md with release notes
3. Create a git tag: `git tag -a v1.2.3 -m "Release v1.2.3"`
4. Push the tag: `git push upstream v1.2.3`
5. GitHub Actions will automatically create a release

## Getting Help

- **Discord**: Join our community chat (link in README)
- **Issues**: Check existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- The project website

Thank you for contributing to Vexy SVGO! 🎉
