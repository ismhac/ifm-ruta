#!/bin/bash

# Create Windows distribution package
# Creates a zip file with the executable and documentation

set -e

echo "Creating Windows distribution package..."

# Check if Windows builds exist
if [ ! -f "target/x86_64-pc-windows-gnu/release/ifm-ruta.exe" ]; then
    echo "Windows 64-bit build not found. Run ./build-windows.sh first."
    exit 1
fi

if [ ! -f "target/i686-pc-windows-gnu/release/ifm-ruta.exe" ]; then
    echo "Windows 32-bit build not found. Run ./build-windows.sh first."
    exit 1
fi

# Create distribution directory
DIST_DIR="ifm-ruta-windows"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copy executables
echo "Copying executables..."
cp "target/x86_64-pc-windows-gnu/release/ifm-ruta.exe" "$DIST_DIR/ifm-ruta-64bit.exe"
cp "target/i686-pc-windows-gnu/release/ifm-ruta.exe" "$DIST_DIR/ifm-ruta-32bit.exe"

# Create README for Windows users
cat > "$DIST_DIR/README.txt" << 'EOF'
IFM-Ruta - Interactive Feedback MCP for Windows
==============================================

This package contains IFM-Ruta, a unified application that provides both
MCP server functionality for Cursor IDE integration and a GUI application
for interactive feedback collection.

Files:
- ifm-ruta-64bit.exe  (For 64-bit Windows)
- ifm-ruta-32bit.exe  (For 32-bit Windows)

Usage:
======

1. MCP Server Mode (for Cursor IDE):
   ifm-ruta-64bit.exe --mcp-server
   
   Add this to your Cursor MCP configuration:
   {
     "mcpServers": {
       "ifm-ruta": {
         "command": "C:\\path\\to\\ifm-ruta-64bit.exe",
         "args": ["--mcp-server"]
       }
     }
   }

2. GUI Mode (for interactive feedback):
   ifm-ruta-64bit.exe <project_directory> [summary]
   
   Example:
   ifm-ruta-64bit.exe C:\my-project "Fix login bug"

3. Show help:
   ifm-ruta-64bit.exe

Features:
=========
- Single executable file (no installation required)
- MCP server for Cursor IDE integration
- GUI application for feedback collection
- Vietnamese font support
- Conversation history storage
- Works on Windows 7, 8, 10, 11

Requirements:
=============
- Windows 7 or later
- No additional dependencies required

Installation:
=============
1. Download and extract this package
2. Copy the appropriate .exe file to your desired location
3. Run the executable as needed

For Cursor integration:
1. Open Cursor settings
2. Add the MCP server configuration (see Usage section above)
3. Restart Cursor

Support:
========
- GitHub: https://github.com/ismhac/ifm-ruta
- Issues: https://github.com/ismhac/ifm-ruta/issues

Version: 0.1.0
EOF

# Create batch file for easy usage
cat > "$DIST_DIR/run-mcp-server.bat" << 'EOF'
@echo off
echo Starting IFM-Ruta MCP Server...
echo.
echo This will run the MCP server for Cursor integration.
echo Press Ctrl+C to stop the server.
echo.
if exist "ifm-ruta-64bit.exe" (
    ifm-ruta-64bit.exe --mcp-server
) else if exist "ifm-ruta-32bit.exe" (
    ifm-ruta-32bit.exe --mcp-server
) else (
    echo Error: No executable found!
    pause
)
EOF

cat > "$DIST_DIR/run-gui.bat" << 'EOF'
@echo off
echo Starting IFM-Ruta GUI...
echo.
if exist "ifm-ruta-64bit.exe" (
    ifm-ruta-64bit.exe %*
) else if exist "ifm-ruta-32bit.exe" (
    ifm-ruta-32bit.exe %*
) else (
    echo Error: No executable found!
    pause
)
EOF

# Create version info
echo "IFM-Ruta v0.1.0" > "$DIST_DIR/VERSION.txt"
echo "Build date: $(date)" >> "$DIST_DIR/VERSION.txt"
echo "Target: Windows x86_64 and i686" >> "$DIST_DIR/VERSION.txt"

# Create zip package
PACKAGE_NAME="ifm-ruta-windows-v0.1.0.zip"
echo "Creating zip package: $PACKAGE_NAME"
zip -r "$PACKAGE_NAME" "$DIST_DIR"

# Show package contents
    echo ""
    echo "Windows package created successfully!"
    echo ""
    echo "Package: $PACKAGE_NAME"
    echo "Contents:"
    ls -la "$DIST_DIR"
    echo ""
    echo "Package size: $(du -h "$PACKAGE_NAME" | cut -f1)"
    echo ""
    echo "Distribution ready!"
    echo "   Users can download and extract the zip file"
    echo "   No installation required - just run the .exe files"
