#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Generating code snapshot in ./llms.txt ..."
llms . "llms*.txt,*.d,*.json,*.html,*.svg,.specstory,ref,testdata,*.lock,*.svg,*.css"

echo "Building the vexy_svgo project..."
{
    # Call the consolidated build script
    ./scripts/build.sh

    echo "Running tests..."
    # Run all unit and integration tests
    cargo test

    echo "Running linter (clippy)..."
    # Run clippy to catch common mistakes and improve code quality
    cargo clippy -- -D warnings

    echo "Checking code formatting..."
    # Check if code is formatted according to rustfmt rules
    cargo fmt --check

    echo "Build and verification complete."
    echo "To run the optimized binary, use: ./target/release/vexy-svgo"

    # ./target/release/vexy-svgo --help # This will be handled by the new build script
} >build.log.txt 2>&1

echo "build log created in: build.log.txt"
