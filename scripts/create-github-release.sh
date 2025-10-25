#!/bin/bash

# Create GitHub Release with Windows executables
# Usage: ./scripts/create-github-release.sh

set -e

echo "Creating GitHub Release for v0.1.0..."

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed."
    echo "Please install it from: https://cli.github.com/"
    exit 1
fi

# Check if user is authenticated
if ! gh auth status &> /dev/null; then
    echo "Error: Not authenticated with GitHub CLI."
    echo "Please run: gh auth login"
    exit 1
fi

# Check if package exists
PACKAGE_FILE="ifm-ruta-windows-v0.1.0.zip"
if [ ! -f "$PACKAGE_FILE" ]; then
    echo "Error: Package file $PACKAGE_FILE not found."
    echo "Please run: ./scripts/create-windows-package.sh first"
    exit 1
fi

# Create release with package
echo "Creating release v0.1.0 with Windows package..."

gh release create v0.1.0 \
    --title "IFM-Ruta v0.1.0 - Unified Interactive Feedback MCP" \
    --notes-file RELEASE_NOTES.md \
    "$PACKAGE_FILE" \
    --repo ismhac/ifm-ruta

echo ""
echo "GitHub Release created successfully!"
echo ""
echo "Release URL: https://github.com/ismhac/ifm-ruta/releases/tag/v0.1.0"
echo ""
echo "Users can now:"
echo "1. Download the Windows package from GitHub Releases"
echo "2. Extract the zip file"
echo "3. Run ifm-ruta-64bit.exe or ifm-ruta-32bit.exe"
echo "4. No additional installation required!"
echo ""
echo "Package includes:"
echo "- 64-bit Windows executable (6.0MB)"
echo "- 32-bit Windows executable (5.8MB)"
echo "- README.txt with usage instructions"
echo "- Batch files for easy execution"
echo "- Version information"
