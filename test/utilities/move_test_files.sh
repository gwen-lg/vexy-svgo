#!/bin/bash
# Move existing test files to organized structure

set -e

echo "🔄 Moving existing test files to organized structure..."

# Create samples subdirectories
mkdir -p testdata/samples/inline_styles
mkdir -p testdata/samples/simple
mkdir -p testdata/samples/complex

# Move inline styles test files
mv test_inline_* testdata/samples/inline_styles/ 2>/dev/null || echo "No inline_styles files to move"

# Move simple test files  
mv test_simple* testdata/samples/simple/ 2>/dev/null || echo "No simple test files to move"

# Move complex test files
mv test_complex* testdata/samples/complex/ 2>/dev/null || echo "No complex test files to move"

# Move any remaining test_*.svg files
find . -maxdepth 1 -name "test_*.svg" -exec mv {} testdata/samples/ \; 2>/dev/null || echo "No additional test files to move"

# Move test_css_debug.rs to appropriate location
mv test_css_debug.rs testdata/samples/ 2>/dev/null || echo "No test_css_debug.rs to move"

echo "✅ Test files moved successfully!"
echo "📁 Check testdata/samples/ for organized test files"
