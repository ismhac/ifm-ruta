#!/bin/bash

# IFM-Ruta Release Build Script
# Builds release binaries for Windows and Fedora

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VERSION=${1:-"1.0.0"}
PLATFORM=${2:-"all"}
ARCH=${3:-"both"}

echo -e "${BLUE}IFM-Ruta Release Build Script${NC}"
echo -e "${BLUE}Version: ${VERSION}${NC}"
echo -e "${BLUE}Platform: ${PLATFORM}${NC}"
echo -e "${BLUE}Architecture: ${ARCH}${NC}"
echo ""

# Create output directory
mkdir -p dist

# Function to build Windows
build_windows() {
    echo -e "${YELLOW}Building Windows binaries...${NC}"
    
    # Install cross-compilation target if needed
    rustup target add x86_64-pc-windows-gnu
    rustup target add i686-pc-windows-gnu
    
    # Build x64
    if [[ "$ARCH" == "x64" || "$ARCH" == "both" ]]; then
        echo -e "${YELLOW}Building Windows x64...${NC}"
        cargo build --release --target x86_64-pc-windows-gnu --workspace
        
        # Create Windows x64 package
        mkdir -p dist/windows-x64
        cp target/x86_64-pc-windows-gnu/release/ifm-ruta-mcp.exe dist/windows-x64/
        cp target/x86_64-pc-windows-gnu/release/ifm-ruta-egui.exe dist/windows-x64/
        cp README.md dist/windows-x64/
        cp LICENSE dist/windows-x64/
        
        cd dist/windows-x64
        zip -r ../ifm-ruta-windows-x64-${VERSION}.zip *
        cd ../..
        echo -e "${GREEN}Windows x64 build complete: dist/ifm-ruta-windows-x64-${VERSION}.zip${NC}"
    fi
    
    # Build x86
    if [[ "$ARCH" == "x86" || "$ARCH" == "both" ]]; then
        echo -e "${YELLOW}Building Windows x86...${NC}"
        cargo build --release --target i686-pc-windows-gnu --workspace
        
        # Create Windows x86 package
        mkdir -p dist/windows-x86
        cp target/i686-pc-windows-gnu/release/ifm-ruta-mcp.exe dist/windows-x86/
        cp target/i686-pc-windows-gnu/release/ifm-ruta-egui.exe dist/windows-x86/
        cp README.md dist/windows-x86/
        cp LICENSE dist/windows-x86/
        
        cd dist/windows-x86
        zip -r ../ifm-ruta-windows-x86-${VERSION}.zip *
        cd ../..
        echo -e "${GREEN}Windows x86 build complete: dist/ifm-ruta-windows-x86-${VERSION}.zip${NC}"
    fi
}

# Function to build Fedora RPM
build_fedora() {
    echo -e "${YELLOW}Building Fedora RPM packages...${NC}"
    
    # Check if rpm is installed
    if ! command -v rpmbuild &> /dev/null; then
        echo -e "${RED}Error: rpmbuild not found. Please install rpm-build package.${NC}"
        echo -e "${YELLOW}On Ubuntu/Debian: sudo apt-get install rpm${NC}"
        echo -e "${YELLOW}On Fedora/RHEL: sudo dnf install rpm-build${NC}"
        exit 1
    fi
    
    # Build x64
    if [[ "$ARCH" == "x64" || "$ARCH" == "both" ]]; then
        echo -e "${YELLOW}Building Fedora x86_64 RPM...${NC}"
        cargo build --release --target x86_64-unknown-linux-gnu --workspace
        
        # Create RPM spec file
        cat > ifm-ruta.spec << EOF
Name:           ifm-ruta
Version:        ${VERSION}
Release:        1%{?dist}
Summary:        Interactive Feedback MCP - Rust + egui
License:        MIT
URL:            https://github.com/ismhac/ifm-ruta
Source0:        %{name}-%{version}.tar.gz

BuildArch:      x86_64

%description
A high-performance, cross-platform MCP (Model Context Protocol) server 
for interactive feedback in AI-assisted development, built with Rust and egui.

%prep
%setup -q

%build
# Binaries are pre-built

%install
rm -rf \$RPM_BUILD_ROOT
mkdir -p \$RPM_BUILD_ROOT/usr/bin
mkdir -p \$RPM_BUILD_ROOT/usr/share/doc/ifm-ruta
mkdir -p \$RPM_BUILD_ROOT/usr/share/licenses/ifm-ruta

# Install binaries
install -m 755 target/x86_64-unknown-linux-gnu/release/ifm-ruta-mcp \$RPM_BUILD_ROOT/usr/bin/
install -m 755 target/x86_64-unknown-linux-gnu/release/ifm-ruta-egui \$RPM_BUILD_ROOT/usr/bin/

# Install documentation
install -m 644 README.md \$RPM_BUILD_ROOT/usr/share/doc/ifm-ruta/
install -m 644 LICENSE \$RPM_BUILD_ROOT/usr/share/licenses/ifm-ruta/

%files
%defattr(-,root,root,-)
/usr/bin/ifm-ruta-mcp
/usr/bin/ifm-ruta-egui
%doc /usr/share/doc/ifm-ruta/README.md
%license /usr/share/licenses/ifm-ruta/LICENSE

%changelog
* $(date '+%a %b %d %Y') IFM-Ruta Team <team@ifm-ruta.dev> - ${VERSION}-1
- Initial release
EOF

        # Create source tarball
        tar -czf ifm-ruta-${VERSION}.tar.gz \
          --exclude='.git' \
          --exclude='target' \
          --exclude='.github' \
          --exclude='*.rpm' \
          --exclude='dist' \
          .

        # Build RPM
        rpmbuild -bb ifm-ruta.spec \
          --define "_sourcedir $(pwd)" \
          --define "_rpmdir $(pwd)/dist" \
          --define "_builddir $(pwd)" \
          --define "_specdir $(pwd)" \
          --define "_srcrpmdir $(pwd)/dist"

        echo -e "${GREEN}Fedora x86_64 RPM build complete: dist/ifm-ruta-${VERSION}-1.x86_64.rpm${NC}"
    fi
    
    # Build x86
    if [[ "$ARCH" == "x86" || "$ARCH" == "both" ]]; then
        echo -e "${YELLOW}Building Fedora i686 RPM...${NC}"
        cargo build --release --target i686-unknown-linux-gnu --workspace
        
        # Create RPM spec file for i686
        cat > ifm-ruta-i686.spec << EOF
Name:           ifm-ruta
Version:        ${VERSION}
Release:        1%{?dist}
Summary:        Interactive Feedback MCP - Rust + egui
License:        MIT
URL:            https://github.com/ismhac/ifm-ruta
Source0:        %{name}-%{version}.tar.gz

BuildArch:      i686

%description
A high-performance, cross-platform MCP (Model Context Protocol) server 
for interactive feedback in AI-assisted development, built with Rust and egui.

%prep
%setup -q

%build
# Binaries are pre-built

%install
rm -rf \$RPM_BUILD_ROOT
mkdir -p \$RPM_BUILD_ROOT/usr/bin
mkdir -p \$RPM_BUILD_ROOT/usr/share/doc/ifm-ruta
mkdir -p \$RPM_BUILD_ROOT/usr/share/licenses/ifm-ruta

# Install binaries
install -m 755 target/i686-unknown-linux-gnu/release/ifm-ruta-mcp \$RPM_BUILD_ROOT/usr/bin/
install -m 755 target/i686-unknown-linux-gnu/release/ifm-ruta-egui \$RPM_BUILD_ROOT/usr/bin/

# Install documentation
install -m 644 README.md \$RPM_BUILD_ROOT/usr/share/doc/ifm-ruta/
install -m 644 LICENSE \$RPM_BUILD_ROOT/usr/share/licenses/ifm-ruta/

%files
%defattr(-,root,root,-)
/usr/bin/ifm-ruta-mcp
/usr/bin/ifm-ruta-egui
%doc /usr/share/doc/ifm-ruta/README.md
%license /usr/share/licenses/ifm-ruta/LICENSE

%changelog
* $(date '+%a %b %d %Y') IFM-Ruta Team <team@ifm-ruta.dev> - ${VERSION}-1
- Initial release
EOF

        # Build RPM for i686
        rpmbuild -bb ifm-ruta-i686.spec \
          --define "_sourcedir $(pwd)" \
          --define "_rpmdir $(pwd)/dist" \
          --define "_builddir $(pwd)" \
          --define "_specdir $(pwd)" \
          --define "_srcrpmdir $(pwd)/dist"

        echo -e "${GREEN}Fedora i686 RPM build complete: dist/ifm-ruta-${VERSION}-1.i686.rpm${NC}"
    fi
}

# Main build logic
case $PLATFORM in
    "windows")
        build_windows
        ;;
    "fedora")
        build_fedora
        ;;
    "all")
        build_windows
        build_fedora
        ;;
    *)
        echo -e "${RED}Error: Invalid platform. Use 'windows', 'fedora', or 'all'${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Build complete!${NC}"
echo -e "${BLUE}Output directory: dist/${NC}"
echo -e "${BLUE}Files created:${NC}"
ls -la dist/

echo ""
echo -e "${YELLOW}To install RPM packages:${NC}"
echo -e "${YELLOW}  sudo rpm -i dist/ifm-ruta-*.rpm${NC}"
echo ""
echo -e "${YELLOW}To extract Windows ZIP:${NC}"
echo -e "${YELLOW}  unzip dist/ifm-ruta-windows-*.zip${NC}"
