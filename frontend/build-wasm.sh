#!/bin/bash
set -e

echo "Building Brick Creator for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Build the WASM package
echo "Running wasm-pack build..."
wasm-pack build --target web --out-dir pkg

# Copy index.html to pkg directory
echo "Copying index.html..."
cp index.html pkg/

echo ""
echo "Build complete! To run the application:"
echo "1. Install a local server (e.g., 'cargo install basic-http-server')"
echo "2. Run: basic-http-server pkg"
echo "3. Open http://localhost:4000 in your browser"
