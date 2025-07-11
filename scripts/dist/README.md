# Vexy SVGO Distribution Scripts

This directory contains robust, maintainable, and failsafe scripts for building and packaging Vexy SVGO deliverables for all major platforms. These scripts are intended to be run locally and are also used by the GitHub Actions release workflow to ensure a single source of truth for packaging logic.

## Scripts

- `build_macos.sh`: Builds the release binary, creates a `.pkg` installer, and wraps it in a `.dmg` for macOS. The `.pkg` installs the CLI tool into `/usr/local/bin`.
- `build_windows.sh`: Cross-compiles the release binary for Windows, zips it, and places it in `dist/windows`.
- `build_linux.sh`: Builds the release binary for Linux, tars and gzips it, and places it in `dist/linux`.
- `release_all.sh`: Runs all platform-specific build scripts and collects deliverables in `dist/`.

## Usage

Run any script directly from the project root:

```bash
./scripts/dist/build_macos.sh
./scripts/dist/build_windows.sh
./scripts/dist/build_linux.sh
./scripts/dist/release_all.sh
```

All deliverables will be placed in the `dist/` directory under their respective platform subfolders.

## Requirements

- **macOS**: `cargo`, `pkgbuild`, `productbuild`, `hdiutil` (for .dmg), `codesign` (optional for signing)
- **Windows**: `cargo`, `zip`, `x86_64-pc-windows-gnu` toolchain (install with `rustup`)
- **Linux**: `cargo`, `tar`, `gzip`

## Failsafe Design

- All scripts use `set -euo pipefail` to abort on any error.
- Temporary and output directories are cleaned before each build.
- Versioning is automatically detected from `Cargo.toml`.
- All scripts are idempotent and can be run repeatedly.

## Maintenance

- Update these scripts if packaging requirements change.
- The GitHub Actions workflow (`.github/workflows/release.yml`) calls only these scripts for all build and packaging logic.
- To test a release locally, simply run `./scripts/dist/release_all.sh` and inspect the `dist/` directory.

## Contribution

If you add new platforms or change packaging logic, update this README and the corresponding scripts.
