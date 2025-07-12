#!/bin/bash
# this_file: scripts/install-docs-deps.sh

set -e

echo "Installing MkDocs documentation dependencies..."

# Ensure pip is up to date
python3 -m pip install --upgrade pip

# Install documentation requirements
pip install -r requirements.txt

echo "Documentation dependencies installed successfully!"
echo "You can now run:"
echo "  - 'mkdocs serve' to preview the documentation locally"
echo "  - 'mkdocs build' to build the documentation"
echo "  - 'make docs-serve' or 'make docs-build' if you prefer using make"