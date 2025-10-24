# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Real conversation history storage and display
- Automatic project setup with `.gitignore` and README creation
- Conversation storage service with cleanup and statistics
- Enhanced egui GUI with Vietnamese font support
- Memory management for conversation history (100 conversations in memory)
- Git integration for automatic `.ifm-ruta/` directory exclusion
- Storage statistics and cleanup tools
- Comprehensive error handling and logging

### Changed
- Updated JSON-RPC implementation to support optional `id` and `params` fields
- Improved egui GUI layout with fixed height and scroll areas
- Enhanced conversation memory management
- Updated MCP protocol compatibility

### Fixed
- JSON-RPC parsing errors with Cursor MCP client
- GUI layout issues with long conversation history
- Memory management for conversation storage
- Error handling in conversation storage service

## [0.1.0] - 2024-10-24

### Added
- Initial release of IFM-Ruta
- MCP (Model Context Protocol) server implementation
- egui-based native GUI application
- Core system with trait-based architecture
- Interactive feedback tool
- Cross-platform support (Windows, macOS, Linux)
- Vietnamese font support
- Basic conversation storage
- Project structure with core, mcp, and egui modules

### Features
- High-performance Rust implementation
- Native GUI with egui
- MCP protocol compliance
- Cross-platform binaries
- Minimal dependencies
- Extensible architecture

## [0.0.1] - 2024-10-24

### Added
- Initial project setup
- Basic MCP server structure
- Core trait definitions
- Basic GUI framework
- Build system and configuration
