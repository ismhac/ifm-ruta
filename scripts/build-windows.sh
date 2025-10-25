#!/bin/bash

# Build script for IFM-Ruta Windows Executable
# Creates .exe files for Windows users

set -e

echo "Building IFM-Ruta for Windows..."

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build for Windows x64
echo "Building for Windows x64..."
cargo build --release --package ifm-ruta-unified --target x86_64-pc-windows-gnu

# Build for Windows x86 (32-bit)
echo "Building for Windows x86 (32-bit)..."
cargo build --release --package ifm-ruta-unified --target i686-pc-windows-gnu

# Check if builds were successful
if [ $? -eq 0 ]; then
    echo ""
    echo "Windows builds successful!"
    echo ""
    echo "Windows Executables:"
    echo "   target/x86_64-pc-windows-gnu/release/ifm-ruta.exe  (64-bit)"
    echo "   target/i686-pc-windows-gnu/release/ifm-ruta.exe    (32-bit)"
    echo ""
    echo "Usage on Windows:"
    echo "   # Run as MCP server (for Cursor integration):"
    echo "   ifm-ruta.exe --mcp-server"
    echo ""
    echo "   # Run as GUI application:"
    echo "   ifm-ruta.exe <project_directory> [summary]"
    echo ""
    echo "   # Show help:"
    echo "   ifm-ruta.exe"
    echo ""
    echo "Windows executable includes:"
    echo "   - MCP Server functionality"
    echo "   - GUI Application (egui)"
    echo "   - All dependencies bundled"
    echo "   - Vietnamese font support"
    echo "   - No additional installation required"
    echo ""
    
    # Show file sizes
    if [ -f "target/x86_64-pc-windows-gnu/release/ifm-ruta.exe" ]; then
        SIZE64=$(du -h target/x86_64-pc-windows-gnu/release/ifm-ruta.exe | cut -f1)
        echo "64-bit executable size: $SIZE64"
    fi
    
    if [ -f "target/i686-pc-windows-gnu/release/ifm-ruta.exe" ]; then
        SIZE32=$(du -h target/i686-pc-windows-gnu/release/ifm-ruta.exe | cut -f1)
        echo "32-bit executable size: $SIZE32"
    fi
    
    echo ""
    echo "For distribution:"
    echo "   - Copy the .exe file to Windows machine"
    echo "   - No additional dependencies needed"
    echo "   - Works on Windows 7, 8, 10, 11"
else
    echo "Windows build failed!"
    exit 1
fi
