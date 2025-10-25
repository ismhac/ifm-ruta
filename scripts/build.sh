#!/bin/bash

# Build script for IFM-Ruta
set -e

echo "Building IFM-Ruta..."

# Check if Windows build is requested
if [ "$1" = "--windows" ]; then
    echo "Building Windows executables..."
    cargo build --release --package ifm-ruta-unified --target x86_64-pc-windows-gnu
    cargo build --release --package ifm-ruta-unified --target i686-pc-windows-gnu
    echo ""
    echo "Windows builds complete!"
    echo "Windows executables:"
    echo "   target/x86_64-pc-windows-gnu/release/ifm-ruta.exe  (64-bit)"
    echo "   target/i686-pc-windows-gnu/release/ifm-ruta.exe    (32-bit)"
    echo ""
    echo "Usage on Windows:"
    echo "   ifm-ruta.exe --mcp-server    # MCP server mode"
    echo "   ifm-ruta.exe <dir> [summary] # GUI mode"
# Check if unified build is requested
elif [ "$1" = "--unified" ]; then
    echo "Building unified executable..."
    cargo build --release --package ifm-ruta-unified
    echo ""
    echo "Unified build complete!"
    echo "Single executable: target/release/ifm-ruta"
    echo ""
    echo "Usage:"
    echo "   ./target/release/ifm-ruta --mcp-server    # MCP server mode"
    echo "   ./target/release/ifm-ruta <dir> [summary] # GUI mode"
else
    # Build all packages
    cargo build --release --workspace
    echo ""
    echo "Build complete!"
    echo "Binaries:"
    echo "  - MCP Server: target/release/ifm-ruta-mcp"
    echo "  - GUI App: target/release/ifm-ruta-egui"
    echo "  - Unified: target/release/ifm-ruta (use --unified flag)"
    echo "  - Windows: target/*/release/ifm-ruta.exe (use --windows flag)"
    echo ""
    echo "Options:"
    echo "   $0 --unified   # Single Linux executable"
    echo "   $0 --windows   # Windows .exe files"
fi


