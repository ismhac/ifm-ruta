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
   ./scripts/build.sh
   ```

3. **Configure MCP server**:
   Add to your Cursor settings (`~/.cursor/mcp.json`):
   ```json
   {
     "mcpServers": {
       "rust-mcp": {
         "command": "/path/to/ifm-ruta/target/release/ifm-ruta",
         "args": ["--mcp-server"],
         "timeout": 10,
         "env": {
           "RUST_LOG": "debug"
         },
         "autoApprove": ["interactive_feedback"]
       }
     }
   }
   ```

4. **Activate MCP Interactive Feedback**:
   To enable the MCP interactive feedback functionality, you need to communicate with the AI assistant using these specific phrases:
   
   - **For questions**: "Whenever you want to ask a question, always call the MCP interactive_feedback."
   - **For request completion**: "Whenever you're about to complete a user request, call the MCP interactive_feedback instead of simply ending the process. Keep calling MCP until the user's feedback is empty, then end the request."
   
   These phrases tell the AI assistant to use the MCP interactive feedback tool instead of ending conversations abruptly.

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

## Build Options

### Unified Build (Recommended)
Creates a single executable that can run in both MCP server and GUI modes:

```bash
# Build unified executable
./scripts/build.sh --unified

# Usage:
./target/release/ifm-ruta --mcp-server    # MCP server mode
./target/release/ifm-ruta <dir> [summary] # GUI mode
```

### Windows Build
Creates Windows .exe files for distribution:

```bash
# Build Windows executables
./scripts/build.sh --windows

# Creates:
# target/x86_64-pc-windows-gnu/release/ifm-ruta.exe  (64-bit)
# target/i686-pc-windows-gnu/release/ifm-ruta.exe    (32-bit)
```

### Standard Build
Builds all components separately:

```bash
# Build all components
./scripts/build.sh

# Creates:
# target/release/ifm-ruta    # Unified executable
```

## Windows Distribution

For Windows users, we provide a complete distribution package:

1. **Build Windows package**:
   ```bash
   ./scripts/build-windows.sh
   ./scripts/create-windows-package.sh
   ```

2. **Download**: `ifm-ruta-windows-v0.1.0.zip` (5.8MB)

3. **Installation**: Extract and run - no additional dependencies required

See [WINDOWS_INSTALLATION.md](WINDOWS_INSTALLATION.md) for detailed Windows setup instructions.

## Architecture

### Core System
- **Minimal Dependencies**: Core system with minimal external dependencies
- **Stable Interfaces**: Well-defined trait-based interfaces
- **Single Responsibility**: Clear separation of concerns
- **Extension Points**: Plugin system for future enhancements

### Components
- **core**: Core system with traits, models, and services
- **unified**: Unified executable with both MCP and GUI functionality

## Project Structure
```
ifm-ruta/
├── core/                   # Core system
│   ├── src/
│   │   ├── traits/         # Core interfaces
│   │   ├── models/         # Data models
│   │   ├── services/       # Core services
│   │   └── utils/         # Utilities
├── unified/                # Unified executable
│   ├── src/
│   │   ├── mcp/           # MCP protocol
│   │   └── tools/         # MCP tools
├── scripts/                # Build and utility scripts
│   ├── build.sh           # Main build script
│   ├── build-unified.sh   # Unified build script
│   ├── build-windows.sh   # Windows build script
│   ├── create-windows-package.sh
│   ├── test-mcp.sh        # MCP testing script
│   ├── debug-mcp.sh       # MCP debugging script
│   └── restart-mcp.sh     # MCP restart script
├── README.md              # This file
├── WINDOWS_INSTALLATION.md # Windows setup guide
└── LICENSE                # MIT License
```

## Development

### Building

```bash
# Build all components
./scripts/build.sh

# Build unified executable
./scripts/build.sh --unified

# Build Windows executables
./scripts/build.sh --windows

# Build individual components
cargo build --release --package ifm-ruta-core
cargo build --release --package ifm-ruta-unified
```

### Testing

```bash
# Run tests
cargo test --workspace

# Test MCP server
./scripts/test-mcp.sh

# Debug MCP connection
./scripts/debug-mcp.sh
```

## Conversation Storage

### Directory Structure

When you first use the MCP tool in a project, it automatically creates:

```
project/
├── .ifm-ruta/
│   ├── README.md              # Explains the directory purpose
│   └── conversations/         # Conversation storage
│       ├── current-conversation.json
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

## API Reference

### MCP Tools

#### `interactive_feedback`

Request interactive feedback for a project.

**Input**:
- `projectDirectory` (string): Full path to the project directory
- `prompt` (string): The prompt to show to the user
- `previousUserRequest` (string): The previous user request that triggered this interactive feedback

**Output**:
- `command_logs` (string): Output from executed commands
- `interactive_feedback` (string): User-provided feedback
- `conversation_history` (array): Conversation history in JSON format

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

## Troubleshooting

### MCP Connection Issues

1. **Check MCP server**:
   ```bash
   ./scripts/test-mcp.sh
   ```

2. **Debug connection**:
   ```bash
   ./scripts/debug-mcp.sh
   ```

3. **Restart MCP**:
   ```bash
   ./scripts/restart-mcp.sh
   ```

4. **Common solutions**:
   - Restart Cursor completely
   - Check file permissions: `chmod +x target/release/ifm-ruta`
   - Verify MCP configuration path
   - Check Cursor MCP logs in Developer Tools

### Build Issues

1. **Clean build**:
   ```bash
   cargo clean
   ./scripts/build.sh --unified
   ```

2. **Check dependencies**:
   ```bash
   rustup update
   cargo update
   ```

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
- Keep source code clean and well-organized
- Use English for all documentation and comments

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by the original [interactive-feedback-mcp](https://github.com/noopstudios/interactive-feedback-mcp)
- Built with [Rust](https://www.rust-lang.org/) and [egui](https://github.com/emilk/egui)
- Uses [MCP protocol](https://modelcontextprotocol.io/) for AI tool integration