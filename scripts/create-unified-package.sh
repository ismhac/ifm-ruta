#!/bin/bash

# Create unified packages for easier installation
set -e

VERSION=${1:-"1.0.0"}
PLATFORM=${2:-"all"}

echo "Creating unified packages for IFM-Ruta v${VERSION}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create dist directory
mkdir -p dist

# Function to create Windows unified package
create_windows_package() {
    local arch=$1
    echo -e "${YELLOW}Creating Windows unified package (${arch})...${NC}"
    
    # Create package directory
    local package_dir="dist/ifm-ruta-windows-${arch}-${VERSION}"
    mkdir -p "$package_dir"
    
    # Copy binaries
    cp "target/x86_64-pc-windows-gnu/release/ifm-ruta-mcp.exe" "$package_dir/"
    cp "target/x86_64-pc-windows-gnu/release/ifm-ruta-egui.exe" "$package_dir/"
    
    # Copy documentation
    cp README.md "$package_dir/"
    cp LICENSE "$package_dir/"
    
    # Create installation script
    cat << 'EOF' > "$package_dir/install.bat"
@echo off
echo Installing IFM-Ruta...
echo.

REM Create installation directory
set INSTALL_DIR=%PROGRAMFILES%\ifm-ruta
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy files
copy "ifm-ruta-mcp.exe" "%INSTALL_DIR%\"
copy "ifm-ruta-egui.exe" "%INSTALL_DIR%\"
copy "README.md" "%INSTALL_DIR%\"
copy "LICENSE" "%INSTALL_DIR%\"

REM Add to PATH
echo Adding to PATH...
setx PATH "%PATH%;%INSTALL_DIR%" /M

echo.
echo Installation complete!
echo IFM-Ruta has been installed to: %INSTALL_DIR%
echo.
echo To use:
echo   - MCP Server: ifm-ruta-mcp.exe
echo   - GUI App: ifm-ruta-egui.exe
echo.
pause
EOF

    # Create uninstall script
    cat << 'EOF' > "$package_dir/uninstall.bat"
@echo off
echo Uninstalling IFM-Ruta...
echo.

set INSTALL_DIR=%PROGRAMFILES%\ifm-ruta

REM Remove files
if exist "%INSTALL_DIR%" (
    rmdir /s /q "%INSTALL_DIR%"
    echo Removed installation directory: %INSTALL_DIR%
)

echo.
echo Uninstallation complete!
pause
EOF

    # Create ZIP package
    cd dist
    zip -r "ifm-ruta-windows-${arch}-${VERSION}.zip" "ifm-ruta-windows-${arch}-${VERSION}/"
    cd ..
    
    echo -e "${GREEN}Windows unified package created: dist/ifm-ruta-windows-${arch}-${VERSION}.zip${NC}"
}

# Function to create Fedora unified package
create_fedora_package() {
    local arch=$1
    echo -e "${YELLOW}Creating Fedora unified package (${arch})...${NC}"
    
    # Create package directory
    local package_dir="dist/ifm-ruta-fedora-${arch}-${VERSION}"
    mkdir -p "$package_dir"
    
    # Copy binaries
    cp "target/x86_64-unknown-linux-gnu/release/ifm-ruta-mcp" "$package_dir/"
    cp "target/x86_64-unknown-linux-gnu/release/ifm-ruta-egui" "$package_dir/"
    
    # Copy documentation
    cp README.md "$package_dir/"
    cp LICENSE "$package_dir/"
    
    # Create installation script
    cat << 'EOF' > "$package_dir/install.sh"
#!/bin/bash

echo "Installing IFM-Ruta..."

# Create installation directory
INSTALL_DIR="/opt/ifm-ruta"
sudo mkdir -p "$INSTALL_DIR"

# Copy files
sudo cp ifm-ruta-mcp "$INSTALL_DIR/"
sudo cp ifm-ruta-egui "$INSTALL_DIR/"
sudo cp README.md "$INSTALL_DIR/"
sudo cp LICENSE "$INSTALL_DIR/"

# Make executable
sudo chmod +x "$INSTALL_DIR/ifm-ruta-mcp"
sudo chmod +x "$INSTALL_DIR/ifm-ruta-egui"

# Create symlinks in /usr/bin
sudo ln -sf "$INSTALL_DIR/ifm-ruta-mcp" /usr/bin/ifm-ruta-mcp
sudo ln -sf "$INSTALL_DIR/ifm-ruta-egui" /usr/bin/ifm-ruta-egui

# Create desktop file
sudo tee /usr/share/applications/ifm-ruta-egui.desktop > /dev/null << 'DESKTOP_EOF'
[Desktop Entry]
Version=1.0
Type=Application
Name=IFM-Ruta GUI
Comment=Interactive Feedback MCP GUI
Exec=ifm-ruta-egui
Icon=applications-development
Terminal=false
Categories=Development;
DESKTOP_EOF

echo "Installation complete!"
echo "IFM-Ruta has been installed to: $INSTALL_DIR"
echo ""
echo "To use:"
echo "  - MCP Server: ifm-ruta-mcp"
echo "  - GUI App: ifm-ruta-egui"
EOF

    chmod +x "$package_dir/install.sh"
    
    # Create uninstall script
    cat << 'EOF' > "$package_dir/uninstall.sh"
#!/bin/bash

echo "Uninstalling IFM-Ruta..."

# Remove symlinks
sudo rm -f /usr/bin/ifm-ruta-mcp
sudo rm -f /usr/bin/ifm-ruta-egui

# Remove desktop file
sudo rm -f /usr/share/applications/ifm-ruta-egui.desktop

# Remove installation directory
sudo rm -rf /opt/ifm-ruta

echo "Uninstallation complete!"
EOF

    chmod +x "$package_dir/uninstall.sh"
    
    # Create TAR package
    cd dist
    tar -czf "ifm-ruta-fedora-${arch}-${VERSION}.tar.gz" "ifm-ruta-fedora-${arch}-${VERSION}/"
    cd ..
    
    echo -e "${GREEN}Fedora unified package created: dist/ifm-ruta-fedora-${arch}-${VERSION}.tar.gz${NC}"
}

# Main logic
if [[ "${PLATFORM}" == "windows" || "${PLATFORM}" == "all" ]]; then
    create_windows_package "x64"
    create_windows_package "x86"
fi

if [[ "${PLATFORM}" == "fedora" || "${PLATFORM}" == "all" ]]; then
    create_fedora_package "x86_64"
fi

echo -e "${GREEN}Unified packages created successfully!${NC}"
echo -e "${BLUE}Output directory: dist/${NC}"
echo -e "${BLUE}Files created:${NC}"
ls -la dist/
