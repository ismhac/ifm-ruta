#!/bin/bash

# Build Windows MSI installer
set -e

VERSION=${1:-"1.0.0"}
ARCH=${2:-"x64"}

echo "Building Windows MSI installer for IFM-Ruta v${VERSION} (${ARCH})"

# Check if WiX Toolset is installed
if ! command -v candle.exe &> /dev/null; then
    echo "Error: WiX Toolset not found. Please install WiX Toolset v3.11 or later."
    echo "Download from: https://wixtoolset.org/releases/"
    exit 1
fi

# Set paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
INSTALLER_DIR="$SCRIPT_DIR"
BUILD_DIR="$INSTALLER_DIR/build"

# Create build directory
mkdir -p "$BUILD_DIR"

# Copy binaries to installer directory
echo "Copying binaries..."
cp "$PROJECT_ROOT/target/x86_64-pc-windows-gnu/release/ifm-ruta-mcp.exe" "$INSTALLER_DIR/"
cp "$PROJECT_ROOT/target/x86_64-pc-windows-gnu/release/ifm-ruta-egui.exe" "$INSTALLER_DIR/"
cp "$PROJECT_ROOT/README.md" "$INSTALLER_DIR/"
cp "$PROJECT_ROOT/LICENSE" "$INSTALLER_DIR/"

# Compile WiX source
echo "Compiling WiX source..."
candle.exe -out "$BUILD_DIR/ifm-ruta.wixobj" "$INSTALLER_DIR/ifm-ruta.wxs"

# Link MSI
echo "Creating MSI installer..."
light.exe -out "$BUILD_DIR/ifm-ruta-${VERSION}-${ARCH}.msi" "$BUILD_DIR/ifm-ruta.wixobj"

# Copy to dist directory
mkdir -p "$PROJECT_ROOT/dist"
cp "$BUILD_DIR/ifm-ruta-${VERSION}-${ARCH}.msi" "$PROJECT_ROOT/dist/"

echo "MSI installer created: dist/ifm-ruta-${VERSION}-${ARCH}.msi"
echo "Size: $(du -h "$PROJECT_ROOT/dist/ifm-ruta-${VERSION}-${ARCH}.msi" | cut -f1)"
