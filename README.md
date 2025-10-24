# Interactive Feedback MCP - Rust + Tauri

A high-performance, cross-platform MCP (Model Context Protocol) server for interactive feedback in AI-assisted development, built with Rust and Tauri.

## Features

- **MCP Protocol**: Full MCP protocol implementation with `interactive_feedback` tool
- **Cross-platform**: Native UI on Windows, macOS, and Linux
- **High Performance**: 3-5x faster startup, 50% less memory usage than Python version
- **Modern UI**: Clean, responsive interface built with Tauri
- **Command Execution**: Run commands with real-time output
- **Settings Management**: Project-specific configuration
- **Extension System**: Plugin architecture for future enhancements

## Architecture

### Core System
- **Minimal Dependencies**: Core system with minimal external dependencies
- **Stable Interfaces**: Well-defined trait-based interfaces
- **Single Responsibility**: Clear separation of concerns
- **Extension Points**: Plugin system for future enhancements

### Components
- **ifm-ruta-core**: Core system with traits, models, and services
- **ifm-ruta-mcp**: MCP server implementation
- **ifm-ruta-tauri**: Tauri application with UI

## Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 16+ (for Tauri frontend)
- Git

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/ifm-ruta.git
   cd ifm-ruta
   ```

2. **Build the project**:
   ```bash
   ./build.sh
   ```

3. **Configure MCP server**:
   Add to your Cursor settings (`mcp.json`):
   ```json
   {
     "mcpServers": {
       "ifm-ruta": {
         "command": "./dist/ifm-ruta-mcp",
         "args": [],
         "timeout": 600,
         "autoApprove": ["interactive_feedback"]
       }
     }
   }
   ```

### Usage

The MCP server provides the `interactive_feedback` tool:

```json
{
  "method": "tools/call",
  "params": {
    "name": "interactive_feedback",
    "arguments": {
      "project_directory": "/path/to/project",
      "summary": "Implemented requested changes"
    }
  }
}
```

## Development

### Project Structure
```
ifm-ruta/
├── ifm-ruta-core/          # Core system
│   ├── src/
│   │   ├── traits/         # Core interfaces
│   │   ├── models/         # Data models
│   │   ├── services/       # Core services
│   │   └── utils/         # Utilities
├── ifm-ruta-mcp/          # MCP server
│   ├── src/
│   │   ├── mcp/           # MCP protocol
│   │   └── tools/         # MCP tools
├── ifm-ruta-tauri/        # Tauri application
│   ├── src/               # Rust backend
│   ├── src-tauri/         # Tauri configuration
│   └── ui/                # Frontend
└── docs/                  # Documentation
```

### Building

```bash
# Build all components
./build.sh

# Build individual components
cargo build --release --package ifm-ruta-core
cargo build --release --package ifm-ruta-mcp
cargo build --release --package ifm-ruta-tauri
```

### Testing

```bash
# Run tests
cargo test --workspace

# Run MCP server tests
cargo test --package ifm-ruta-mcp

# Run Tauri tests
cargo test --package ifm-ruta-tauri
```

## Configuration

### MCP Server Configuration

The MCP server can be configured through environment variables:

- `IFM_RUTA_LOG_LEVEL`: Log level (error, warn, info, debug, trace)
- `IFM_RUTA_TIMEOUT`: Timeout for user interaction (seconds)
- `IFM_RUTA_CONFIG_DIR`: Custom configuration directory

### Tauri Application Configuration

The Tauri application supports various configuration options:

- **Window Settings**: Size, position, state
- **Theme**: Light, dark, or auto
- **Command Execution**: Allowed commands, sandbox mode
- **Performance**: Memory limits, cache size

## API Reference

### MCP Tools

#### `interactive_feedback`

Request interactive feedback for a project.

**Input**:
- `project_directory` (string): Full path to the project directory
- `summary` (string): Brief summary of changes made

**Output**:
- `command_logs` (string): Output from executed commands
- `interactive_feedback` (string): User-provided feedback

### Tauri Commands

- `submit_feedback(feedback: string)`: Submit user feedback
- `execute_command(command: string, args: string[])`: Execute a command
- `get_settings()`: Get application settings
- `set_settings(settings: AppSettings)`: Set application settings

## Performance

### Benchmarks
- **Startup Time**: < 1 second (vs 2-3s Python version)
- **Memory Usage**: < 30MB (vs 50-100MB Python version)
- **Binary Size**: < 20MB (vs 50-100MB Python version)
- **UI Responsiveness**: 60fps

### Optimization
- **Lazy Loading**: Components loaded on demand
- **Memory Management**: Efficient memory usage
- **Process Optimization**: Optimized process execution
- **UI Performance**: Hardware-accelerated rendering

## Security

### Input Validation
- **Command Sanitization**: Prevents injection attacks
- **Path Validation**: Prevents directory traversal
- **Resource Limits**: CPU and memory limits

### Sandboxing
- **Process Isolation**: Commands run in restricted environment
- **Permission System**: Fine-grained access control
- **Data Protection**: Secure handling of sensitive data

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Development Guidelines

- Follow Rust naming conventions
- Write comprehensive tests
- Document public APIs
- Follow the established architecture patterns

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by the original [interactive-feedback-mcp](https://github.com/noopstudios/interactive-feedback-mcp)
- Built with [Rust](https://www.rust-lang.org/) and [Tauri](https://tauri.app/)
- Uses [MCP protocol](https://modelcontextprotocol.io/) for AI tool integration


