# IFM-Ruta - Interactive Feedback MCP

A high-performance, cross-platform MCP (Model Context Protocol) server for interactive feedback in AI-assisted development, built with Rust and egui. Features real conversation history storage, automatic project setup, and native GUI integration.

## Features

- **MCP Protocol**: Full JSON-RPC 2.0 implementation with `interactive_feedback` tool
- **Conversation History**: Real conversation storage and display between users and AI assistants
- **Auto-Setup**: Automatic `.gitignore` and README creation for project directories
- **Cross-platform**: Native UI on Windows, macOS, and Linux
- **High Performance**: 3-5x faster startup, 50% less memory usage than Python version
- **Modern UI**: Clean, responsive interface built with egui with Vietnamese font support
- **Storage Management**: Conversation cleanup and statistics
- **Git Integration**: Automatic `.ifm-ruta/` directory exclusion from version control

## Architecture

### Core System
- **Minimal Dependencies**: Core system with minimal external dependencies
- **Stable Interfaces**: Well-defined trait-based interfaces
- **Single Responsibility**: Clear separation of concerns
- **Extension Points**: Plugin system for future enhancements

### Components
- **core**: Core system with traits, models, and services
- **mcp**: MCP server implementation
- **egui**: egui application with native GUI

## Quick Start

### Prerequisites
- Rust 1.70+
- Git

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/ismhac/ifm-ruta.git
   cd ifm-ruta
   ```

2. **Build the project**:
   ```bash
   ./build.sh
   ```

3. **Configure MCP server**:
   Add to your Cursor settings (`~/.cursor/mcp.json`):
   ```json
   {
     "mcpServers": {
       "rust-mcp": {
         "command": "/path/to/ifm-ruta/target/release/ifm-ruta-mcp",
         "args": [],
         "timeout": 10,
         "env": {
           "RUST_LOG": "debug"
         },
         "autoApprove": ["interactive_feedback"]
       }
     }
   }
   ```

### Usage

The MCP server provides the `interactive_feedback` tool with conversation history:

```json
{
  "method": "tools/call",
  "params": {
    "name": "interactive_feedback",
    "arguments": {
      "projectDirectory": "/path/to/project",
      "prompt": "Please review the changes I made",
      "previousUserRequest": "I need help with implementing a feature"
    }
  }
}
```

#### Features

- **Real Conversation History**: Displays actual user-assistant conversations from Cursor
- **Auto-Setup**: Automatically creates `.ifm-ruta/` directory with README and updates `.gitignore`
- **Storage Management**: Stores conversations as JSON files with session tracking
- **Memory Management**: Keeps 100 conversations in memory, unlimited storage
- **Cleanup Tools**: Built-in cleanup for old conversation sessions

## Development

### Project Structure
```
ifm-ruta/
├── core/                   # Core system
│   ├── src/
│   │   ├── traits/         # Core interfaces
│   │   ├── models/         # Data models
│   │   ├── services/       # Core services
│   │   └── utils/         # Utilities
├── mcp/                   # MCP server
│   ├── src/
│   │   ├── mcp/           # MCP protocol
│   │   └── tools/         # MCP tools
├── egui/                  # egui application
│   ├── src/               # Rust GUI
│   └── fonts/             # Vietnamese fonts
└── docs/                  # Documentation
```

### Building

```bash
# Build all components
./build.sh

# Build individual components
cargo build --release --package ifm-ruta-core
cargo build --release --package ifm-ruta-mcp
cargo build --release --package ifm-ruta-egui
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

## Conversation Storage

### Directory Structure

When you first use the MCP tool in a project, it automatically creates:

```
project/
├── .ifm-ruta/
│   ├── README.md              # Explains the directory purpose
│   └── conversations/         # Conversation storage
│       ├── cursor-chat-abc123.json
│       ├── cursor-chat-def456.json
│       └── ...
└── .gitignore                 # Updated to exclude .ifm-ruta/
```

### Storage Features

- **Automatic Setup**: Creates `.ifm-ruta/` directory and README on first use
- **Git Integration**: Automatically adds `.ifm-ruta/` to `.gitignore`
- **Session Tracking**: Each conversation is stored as a separate JSON file
- **Memory Management**: 100 conversations kept in memory for fast access
- **Cleanup Tools**: Built-in cleanup for old sessions
- **Statistics**: Storage usage and conversation counts

### Privacy

- Conversation data may contain sensitive information
- Automatically excluded from version control
- Stored locally in project directory
- No external data transmission

## Configuration

### MCP Server Configuration

The MCP server can be configured through environment variables:

- `RUST_LOG`: Log level (error, warn, info, debug, trace)
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


