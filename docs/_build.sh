#!/bin/bash
# Build script for VEXYSVGO documentation site
# Builds Jekyll site with Just the Docs theme and custom CSS

set -e

echo "🚀 Building VEXYSVGO Documentation Site"

# Change to docs directory
cd "$(dirname "$0")"

# Check if Bundle is available
if ! command -v bundle &>/dev/null; then
    echo "❌ Bundle (Ruby gem manager) not found. Please install Ruby and Bundler first."
    echo "   On macOS: brew install ruby && gem install bundler"
    echo "   On Ubuntu: sudo apt install ruby-dev && gem install bundler"
    exit 1
fi

# Install Ruby dependencies
if [ ! -f "Gemfile.lock" ]; then
    echo "📦 Installing Ruby dependencies..."
    bundle install
fi

# Check if Node.js and npm are available for PostCSS processing
if command -v npm &>/dev/null; then
    # Install Node.js dependencies if needed
    if [ ! -d "node_modules" ]; then
        echo "📦 Installing Node.js dependencies for PostCSS/Tailwind..."
        npm install
    fi

    # Process CSS with PostCSS/Tailwind before Jekyll build
    echo "🎨 Processing CSS with PostCSS and Tailwind CSS..."
    npm run build-postcss-prod

    # Verify the CSS was generated
    if [ -f "assets/css/main.css" ]; then
        echo "✅ PostCSS processing complete: assets/css/main.css generated"
        echo "   CSS file size: $(du -h assets/css/main.css | cut -f1)"
    else
        echo "⚠️  Warning: PostCSS processing may have failed"
        echo "   Creating an empty CSS file to prevent errors"
        touch assets/css/main.css
    fi
else
    echo "⚠️  Node.js not found - skipping PostCSS processing"
    echo "   Custom Tailwind styles will not be compiled"
    echo "   Creating an empty CSS file to prevent errors"
    touch assets/css/main.css
fi

# Build Jekyll site (now with processed CSS)
echo "🔨 Building Jekyll site with Just the Docs theme..."
bundle exec jekyll build

# Verify CSS is in the output directory
if [ -f "_site/assets/css/main.css" ]; then
    echo "✅ CSS file successfully included in the build"
else
    echo "❌ CSS file is missing from the build output!"
fi

echo "✅ Build complete! Site built to _site/"
echo "💡 To serve locally: bundle exec jekyll serve"
