#!/bin/bash

# Build script for IFM-Ruta Unified Application
# Creates a single executable that includes both GUI and MCP functionality

set -e

echo "Building IFM-Ruta Unified Application..."

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build the unified package in release mode
echo "Building unified package..."
cargo build --release --package ifm-ruta-unified

# Check if build was successful
if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful!"
    echo ""
    echo "Executable location:"
    echo "   target/release/ifm-ruta"
    echo ""
    echo "Usage:"
    echo "   # Run as MCP server (for Cursor integration):"
    echo "   ./target/release/ifm-ruta --mcp-server"
    echo ""
    echo "   # Run as GUI application:"
    echo "   ./target/release/ifm-ruta <project_directory> [summary]"
    echo ""
    echo "   # Show help:"
    echo "   ./target/release/ifm-ruta"
    echo ""
    echo "Single executable includes:"
    echo "   - MCP Server functionality"
    echo "   - GUI Application (egui)"
    echo "   - All dependencies bundled"
    echo "   - Vietnamese font support"
    echo ""
    
    # Show file size
    if [ -f "target/release/ifm-ruta" ]; then
        SIZE=$(du -h target/release/ifm-ruta | cut -f1)
        echo "Executable size: $SIZE"
    fi
else
    echo "Build failed!"
    exit 1
fi
