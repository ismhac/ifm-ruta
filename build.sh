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

echo "Binaries available in target/release/:"
echo "  - ifm-ruta-mcp: MCP server"
echo "  - ifm-ruta-egui: egui application"
