#!/bin/bash

# Build script for Interactive Feedback MCP

set -e

echo "Building Interactive Feedback MCP..."

# Build core library
echo "Building core library..."
cd ifm-ruta-core
cargo build --release
cd ..

# Build MCP server
echo "Building MCP server..."
cd ifm-ruta-mcp
cargo build --release
cd ..

# Build Tauri application
echo "Building Tauri application..."
cd ifm-ruta-tauri
cargo build --release
cd ..

echo "Build completed successfully!"

# Create distribution directory
echo "Creating distribution..."
mkdir -p dist

# Copy binaries
cp target/release/ifm-ruta-mcp dist/
cp target/release/ifm-ruta-tauri dist/

echo "Distribution created in ./dist/"
echo "Binaries:"
echo "  - ifm-ruta-mcp: MCP server"
echo "  - ifm-ruta-tauri: Tauri application"
