#!/bin/bash

# Build script for Interactive Feedback MCP

set -e

echo "Building Interactive Feedback MCP..."

# Build core library
echo "Building core library..."
cd core
cargo build --release
cd ..

# Build MCP server
echo "Building MCP server..."
cd mcp
cargo build --release
cd ..

# Build egui application
echo "Building egui application..."
cd egui
cargo build --release
cd ..

echo "Build completed successfully!"

# Create distribution directory
echo "Creating distribution..."
mkdir -p dist

# Copy binaries
cp target/release/ifm-ruta-mcp dist/
cp target/release/ifm-ruta-egui dist/

echo "Distribution created in ./dist/"
echo "Binaries:"
echo "  - ifm-ruta-mcp: MCP server"
echo "  - ifm-ruta-egui: egui application"
