Name:           ifm-ruta
Version:        1.0.0
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
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT/usr/bin
mkdir -p $RPM_BUILD_ROOT/usr/share/doc/ifm-ruta
mkdir -p $RPM_BUILD_ROOT/usr/share/licenses/ifm-ruta

        # Install binaries (copy from build directory)
        cp /home/ismverseinfinity/workspaces/mcp/ifm-ruta/target/x86_64-unknown-linux-gnu/release/ifm-ruta-mcp $RPM_BUILD_ROOT/usr/bin/
        cp /home/ismverseinfinity/workspaces/mcp/ifm-ruta/target/x86_64-unknown-linux-gnu/release/ifm-ruta-egui $RPM_BUILD_ROOT/usr/bin/
        chmod +x $RPM_BUILD_ROOT/usr/bin/ifm-ruta-mcp
        chmod +x $RPM_BUILD_ROOT/usr/bin/ifm-ruta-egui

# Install documentation
install -m 644 README.md $RPM_BUILD_ROOT/usr/share/doc/ifm-ruta/
install -m 644 LICENSE $RPM_BUILD_ROOT/usr/share/licenses/ifm-ruta/

%files
%defattr(-,root,root,-)
/usr/bin/ifm-ruta-mcp
/usr/bin/ifm-ruta-egui
%doc /usr/share/doc/ifm-ruta/README.md
%license /usr/share/licenses/ifm-ruta/LICENSE

%changelog
* Fri Oct 24 2025 IFM-Ruta Team <team@ifm-ruta.dev> - 1.0.0-1
- Initial release
